mod db;

use axum::{
    extract::{Path, Query, Request, State},
    http::{header, HeaderValue, StatusCode},
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::{get, patch, post},
    Json, Router,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;
use std::{env, net::SocketAddr, path::PathBuf, sync::Arc};
use thiserror::Error;
use tokio::signal;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::{services::ServeDir, set_header::SetResponseHeaderLayer};
use tracing::{error, info};
use url::Url;

const BUILD_SHA: &str = match option_env!("BUILD_SHA") {
    Some(value) => value,
    None => "dev",
};

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
    static_dir: String,
}

#[derive(Serialize)]
struct Health {
    status: &'static str,
    build: &'static str,
}
#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: i64,
    title: String,
    url: String,
    tags: Vec<String>,
    saved_at: String,
    status: String,
    priority: String,
}
#[derive(Deserialize)]
struct ListQuery {
    status: Option<String>,
    search: Option<String>,
}
#[derive(Deserialize)]
struct UpdateItem {
    status: Option<String>,
    priority: Option<String>,
}
#[derive(Debug, Deserialize)]
struct SaveItem {
    title: String,
    url: String,
    #[serde(default)]
    tags: Vec<String>,
}
#[derive(Deserialize)]
struct CreateToken {
    label: Option<String>,
}
#[derive(Serialize, sqlx::FromRow)]
struct TokenInfo {
    id: i64,
    label: String,
    created_at: String,
    revoked_at: Option<String>,
}
#[derive(Serialize)]
struct Session {
    token: String,
}
#[derive(Serialize)]
struct CreatedToken {
    id: i64,
    label: String,
    token: String,
    feed_path: String,
}
#[derive(Serialize)]
struct ApiMessage {
    error: String,
}

#[derive(Debug, Error)]
enum AppError {
    #[error("{0}")]
    Bad(String),
    #[error("{0}")]
    Missing(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    Internal(String),
    #[error(transparent)]
    Db(#[from] sqlx::Error),
}
impl AppError {
    fn bad(s: impl Into<String>) -> Self {
        Self::Bad(s.into())
    }
    fn not_found(s: impl Into<String>) -> Self {
        Self::Missing(s.into())
    }
    fn unauthorized(s: impl Into<String>) -> Self {
        Self::Unauthorized(s.into())
    }
    fn internal(s: impl Into<String>) -> Self {
        Self::Internal(s.into())
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::Bad(_) => StatusCode::BAD_REQUEST,
            Self::Missing(_) => StatusCode::NOT_FOUND,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Internal(_) | Self::Db(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        if let Self::Db(ref e) = self {
            error!(error=%e, "database error");
        }
        let error = match self {
            Self::Bad(s) | Self::Missing(s) | Self::Unauthorized(s) | Self::Internal(s) => s,
            Self::Db(_) => "The queue could not be updated. Please try again.".into(),
        };
        (status, Json(ApiMessage { error })).into_response()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))
        .init();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:///data/rss-saved-queue.db?mode=rwc".into());
    if let Some(path) = database_url
        .strip_prefix("sqlite://")
        .and_then(|v| v.split('?').next())
    {
        if let Some(parent) = PathBuf::from(path)
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            tokio::fs::create_dir_all(parent).await?;
        }
    }
    let state = Arc::new(AppState {
        pool: db::connect(&database_url).await?,
        static_dir: env::var("STATIC_DIR").unwrap_or_else(|_| "dist".into()),
    });
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let listener = tokio::net::TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port))).await?;
    info!(port, build = BUILD_SHA, "rss saved queue listening");
    axum::serve(
        listener,
        app(state).into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;
    Ok(())
}

fn app(state: Arc<AppState>) -> Router {
    let static_dir = state.static_dir.clone();
    // Session creation persists an account. Keep anonymous creation tight and
    // give ordinary state changes their own, practical per-peer bucket.
    let session_limit = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(6)
            .burst_size(8)
            .finish()
            .expect("valid session rate-limit configuration"),
    );
    let mutation_limit = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(24)
            .finish()
            .expect("valid mutation rate-limit configuration"),
    );
    Router::new()
        .route("/health", get(health))
        .route("/api/session", post(create_session).layer(GovernorLayer { config: session_limit }))
        .route("/api/items", get(list_items))
        .route("/api/items", post(save_item).layer(GovernorLayer { config: mutation_limit.clone() }))
        .route("/api/items/:id", patch(update_item).delete(delete_item).layer(GovernorLayer { config: mutation_limit.clone() }))
        .route("/api/export.csv", get(export_csv))
        .route("/api/feed-tokens", get(list_feed_tokens))
        .route("/api/feed-tokens", post(create_feed_token).layer(GovernorLayer { config: mutation_limit.clone() }))
        .route("/api/feed-tokens/:id/revoke", post(revoke_feed_token).layer(GovernorLayer { config: mutation_limit.clone() }))
        .route("/feed/:token/rss", get(render_feed))
        .route("/reader/:token/items/:id/read", post(reader_mark_read).layer(GovernorLayer { config: mutation_limit }))
        .route("/privacy", get(index_page)).route("/terms", get(index_page))
        .with_state(state)
        .fallback_service(ServeDir::new(static_dir).append_index_html_on_directories(true))
        .layer(middleware::from_fn(cache_headers))
        .layer(SetResponseHeaderLayer::overriding(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff")))
        .layer(SetResponseHeaderLayer::overriding(header::REFERRER_POLICY, HeaderValue::from_static("same-origin")))
        .layer(SetResponseHeaderLayer::overriding(http::HeaderName::from_static("x-frame-options"), HeaderValue::from_static("DENY")))
        .layer(SetResponseHeaderLayer::overriding(http::HeaderName::from_static("permissions-policy"), HeaderValue::from_static("camera=(), microphone=(), geolocation=()")))
        .layer(SetResponseHeaderLayer::overriding(http::HeaderName::from_static("content-security-policy"), HeaderValue::from_static("default-src 'self'; base-uri 'self'; form-action 'self'; frame-ancestors 'none'; object-src 'none'")))
}

async fn cache_headers(request: Request, next: Next) -> Response {
    let path = request.uri().path().to_owned();
    let mut response = next.run(request).await;
    let value = if path.starts_with("/assets/") {
        "public, max-age=31536000, immutable"
    } else if path.starts_with("/api/") || path == "/health" {
        "no-store"
    } else {
        "no-cache"
    };
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static(value));
    response
}
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("install Ctrl+C handler");
    };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("install SIGTERM handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! { _ = ctrl_c => {}, _ = terminate => {} }
}
async fn health() -> Json<Health> {
    Json(Health {
        status: "ok",
        build: BUILD_SHA,
    })
}
async fn index_page(State(state): State<Arc<AppState>>) -> Result<Html<String>, AppError> {
    Ok(Html(
        tokio::fs::read_to_string(PathBuf::from(&state.static_dir).join("index.html"))
            .await
            .map_err(|_| AppError::bad("The web application is not installed."))?,
    ))
}

fn random_token() -> String {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}
fn token_hash(token: &str) -> String {
    format!("{:x}", Sha256::digest(token.as_bytes()))
}
fn bearer(headers: &axum::http::HeaderMap) -> Result<&str, AppError> {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .filter(|v| !v.is_empty())
        .ok_or_else(|| {
            AppError::unauthorized("Sign in on this device to access your private queue.")
        })
}
async fn account_from_headers(
    state: &AppState,
    headers: &axum::http::HeaderMap,
) -> Result<i64, AppError> {
    db::account_for_token(&state.pool, &token_hash(bearer(headers)?))
        .await?
        .ok_or_else(|| AppError::unauthorized("This device key is no longer active."))
}
fn clean_save(mut input: SaveItem) -> Result<SaveItem, AppError> {
    input.title = input.title.trim().to_string();
    input.url = input.url.trim().to_string();
    let url = Url::parse(&input.url)
        .map_err(|_| AppError::bad("Enter a complete http or https article URL."))?;
    if !matches!(url.scheme(), "http" | "https")
        || url.host_str().is_none()
        || url.username() != ""
        || url.password().is_some()
    {
        return Err(AppError::bad("Enter a complete http or https article URL."));
    }
    if input.title.is_empty() || input.title.chars().count() > 300 {
        return Err(AppError::bad(
            "Give this saved page a title of up to 300 characters.",
        ));
    }
    let tags = input
        .tags
        .into_iter()
        .map(|tag| tag.trim().to_string())
        .filter(|tag| !tag.is_empty())
        .collect::<Vec<_>>();
    if tags.len() > 12 {
        return Err(AppError::bad("Use up to 12 tags per saved page."));
    }
    if tags.iter().any(|tag| tag.chars().count() > 40) {
        return Err(AppError::bad("Tags must be 40 characters or fewer."));
    }
    input.tags = tags;
    Ok(input)
}
async fn create_session(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<Session>), AppError> {
    let token = random_token();
    db::create_account(&state.pool, &token_hash(&token)).await?;
    Ok((StatusCode::CREATED, Json(Session { token })))
}
async fn list_items(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Item>>, AppError> {
    let account = account_from_headers(&state, &headers).await?;
    Ok(Json(
        db::list(
            &state.pool,
            account,
            query.status.as_deref(),
            query.search.as_deref(),
        )
        .await?,
    ))
}
async fn save_item(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Json(input): Json<SaveItem>,
) -> Result<(StatusCode, Json<Item>), AppError> {
    let account = account_from_headers(&state, &headers).await?;
    Ok((
        StatusCode::CREATED,
        Json(db::save(&state.pool, account, &clean_save(input)?).await?),
    ))
}
async fn update_item(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Path(id): Path<i64>,
    Json(input): Json<UpdateItem>,
) -> Result<Json<Item>, AppError> {
    if input.status.is_none() && input.priority.is_none() {
        return Err(AppError::bad("Choose a status or priority to update."));
    }
    let account = account_from_headers(&state, &headers).await?;
    Ok(Json(
        db::update(
            &state.pool,
            account,
            id,
            input.status.as_deref(),
            input.priority.as_deref(),
        )
        .await?,
    ))
}
async fn delete_item(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let account = account_from_headers(&state, &headers).await?;
    db::remove(&state.pool, account, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
async fn list_feed_tokens(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Vec<TokenInfo>>, AppError> {
    let account = account_from_headers(&state, &headers).await?;
    Ok(Json(db::list_feed_tokens(&state.pool, account).await?))
}
async fn create_feed_token(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Json(input): Json<CreateToken>,
) -> Result<(StatusCode, Json<CreatedToken>), AppError> {
    let account = account_from_headers(&state, &headers).await?;
    let label = input
        .label
        .unwrap_or_else(|| "My feed reader".into())
        .trim()
        .chars()
        .take(80)
        .collect::<String>();
    if label.is_empty() {
        return Err(AppError::bad("Name this feed-reader connection."));
    }
    let token = random_token();
    let info = db::create_feed_token(&state.pool, account, &token_hash(&token), &label).await?;
    Ok((
        StatusCode::CREATED,
        Json(CreatedToken {
            id: info.id,
            label,
            feed_path: format!("/feed/{token}/rss"),
            token,
        }),
    ))
}
async fn revoke_feed_token(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let account = account_from_headers(&state, &headers).await?;
    db::revoke_feed_token(&state.pool, account, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
async fn account_from_feed_token(state: &AppState, token: &str) -> Result<i64, AppError> {
    db::account_for_feed_token(&state.pool, &token_hash(token))
        .await?
        .ok_or_else(|| AppError::not_found("This private feed link is unavailable."))
}
fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
fn rss_pub_date(saved_at: &str) -> Result<String, AppError> {
    chrono::DateTime::parse_from_rfc3339(saved_at)
        .map(|date| date.to_rfc2822())
        .map_err(|_| AppError::internal("Saved date could not be read."))
}
async fn render_feed(
    State(state): State<Arc<AppState>>,
    Path(token): Path<String>,
) -> Result<Response, AppError> {
    let account = account_from_feed_token(&state, &token).await?;
    let items = db::list(&state.pool, account, Some("queue"), None).await?;
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?><rss version=\"2.0\"><channel><title>RSS Saved Queue</title><description>Private saved reading queue</description><link>https://rss-saved-queue.sociobot.in</link>");
    for item in items {
        xml.push_str(&format!("<item><title>{}</title><link>{}</link><guid isPermaLink=\"false\">{}</guid><description>{}</description><pubDate>{}</pubDate></item>", xml_escape(&item.title), xml_escape(&item.url), item.id, xml_escape(&item.tags.join(", ")), xml_escape(&rss_pub_date(&item.saved_at)?)));
    }
    xml.push_str("</channel></rss>");
    Ok((
        [(header::CONTENT_TYPE, "application/rss+xml; charset=utf-8")],
        xml,
    )
        .into_response())
}
async fn reader_mark_read(
    State(state): State<Arc<AppState>>,
    Path((token, id)): Path<(String, i64)>,
) -> Result<Json<Item>, AppError> {
    let account = account_from_feed_token(&state, &token).await?;
    Ok(Json(
        db::update(&state.pool, account, id, Some("read"), None).await?,
    ))
}
async fn export_csv(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
) -> Result<Response, AppError> {
    let account = account_from_headers(&state, &headers).await?;
    let items = db::list(&state.pool, account, None, None).await?;
    let mut csv = String::from("title,url,tags,status,priority,saved_at\n");
    for item in items {
        csv.push_str(
            &[
                item.title,
                item.url,
                item.tags.join("; "),
                item.status,
                item.priority,
                item.saved_at,
            ]
            .iter()
            .map(|s| format!("\"{}\"", s.replace('"', "\"\"")))
            .collect::<Vec<_>>()
            .join(","),
        );
        csv.push('\n');
    }
    Ok((
        [
            (header::CONTENT_TYPE, "text/csv; charset=utf-8"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=reading-queue.csv",
            ),
        ],
        csv,
    )
        .into_response())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{to_bytes, Body},
        extract::connect_info::ConnectInfo,
        http::Request,
    };
    use tower::ServiceExt;
    async fn test_app() -> Router {
        let pool = db::connect("sqlite::memory:?cache=shared").await.unwrap();
        app(Arc::new(AppState {
            pool,
            static_dir: "dist".into(),
        }))
    }
    async fn send(app: &Router, mut request: Request<Body>) -> (StatusCode, String) {
        request.extensions_mut().insert(ConnectInfo(
            "127.0.0.1:41000".parse::<SocketAddr>().unwrap(),
        ));
        let response = app.clone().oneshot(request).await.unwrap();
        let status = response.status();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        (status, String::from_utf8(body.to_vec()).unwrap())
    }
    async fn session(app: &Router) -> String {
        let (status, body) = send(
            app,
            Request::post("/api/session").body(Body::empty()).unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::CREATED, "{body}");
        serde_json::from_str::<serde_json::Value>(&body).unwrap()["token"]
            .as_str()
            .unwrap()
            .into()
    }
    fn auth(request: axum::http::request::Builder, token: &str) -> axum::http::request::Builder {
        request.header(header::AUTHORIZATION, format!("Bearer {token}"))
    }
    #[tokio::test]
    async fn queue_data_is_private_and_requires_device_key() {
        let app = test_app().await;
        let alice = session(&app).await;
        let bob = session(&app).await;
        let payload =
            r#"{"title":"Private article","url":"https://example.com/post","tags":["notes"]}"#;
        let (status, body) = send(
            &app,
            auth(Request::post("/api/items"), &alice)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(payload))
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::CREATED);
        let id = serde_json::from_str::<serde_json::Value>(&body).unwrap()["id"]
            .as_i64()
            .unwrap();
        let (status, body) = send(
            &app,
            auth(Request::get("/api/items"), &bob)
                .body(Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "[]");
        let (status, _) = send(
            &app,
            auth(Request::patch(format!("/api/items/{id}")), &bob)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"status":"read"}"#))
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        let (status, _) = send(
            &app,
            Request::get("/api/items").body(Body::empty()).unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }
    #[tokio::test]
    async fn private_feed_token_can_mark_read_then_is_revoked() {
        let app = test_app().await;
        let key = session(&app).await;
        let (_, item) = send(
            &app,
            auth(Request::post("/api/items"), &key)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    r#"{"title":"Reader item","url":"https://example.com/reader","tags":[]}"#,
                ))
                .unwrap(),
        )
        .await;
        let id = serde_json::from_str::<serde_json::Value>(&item).unwrap()["id"]
            .as_i64()
            .unwrap();
        let (_, token) = send(
            &app,
            auth(Request::post("/api/feed-tokens"), &key)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"label":"Reader"}"#))
                .unwrap(),
        )
        .await;
        let value: serde_json::Value = serde_json::from_str(&token).unwrap();
        let raw = value["token"].as_str().unwrap();
        let token_id = value["id"].as_i64().unwrap();
        let (status, feed) = send(
            &app,
            Request::get(format!("/feed/{raw}/rss"))
                .body(Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert!(feed.contains("Reader item"));
        let pub_date = feed
            .split("<pubDate>")
            .nth(1)
            .unwrap()
            .split("</pubDate>")
            .next()
            .unwrap();
        assert!(chrono::DateTime::parse_from_rfc2822(pub_date).is_ok());
        let (status, _) = send(
            &app,
            Request::post(format!("/reader/{raw}/items/{id}/read"))
                .body(Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let (status, _) = send(
            &app,
            auth(
                Request::post(format!("/api/feed-tokens/{token_id}/revoke")),
                &key,
            )
            .body(Body::empty())
            .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::NO_CONTENT);
        let (status, _) = send(
            &app,
            Request::get(format!("/feed/{raw}/rss"))
                .body(Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }
    #[test]
    fn removes_remote_import_surface_and_rejects_non_web_saved_links() {
        assert!(clean_save(SaveItem {
            title: "x".into(),
            url: "ftp://example.com/a".into(),
            tags: vec![]
        })
        .is_err());
    }
    #[test]
    fn rejects_over_limit_tags_instead_of_silently_dropping_them() {
        let error = clean_save(SaveItem {
            title: "x".into(),
            url: "https://example.com/a".into(),
            tags: (1..=13).map(|number| format!("tag-{number}")).collect(),
        })
        .unwrap_err();
        assert!(
            matches!(error, AppError::Bad(message) if message == "Use up to 12 tags per saved page.")
        );
    }
    #[tokio::test]
    async fn session_creation_is_rate_limited_by_peer_address() {
        let app = test_app().await;
        for _ in 0..8 {
            let (status, _) = send(
                &app,
                Request::post("/api/session").body(Body::empty()).unwrap(),
            )
            .await;
            assert_eq!(status, StatusCode::CREATED);
        }
        let (status, _) = send(
            &app,
            Request::post("/api/session").body(Body::empty()).unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    }
}

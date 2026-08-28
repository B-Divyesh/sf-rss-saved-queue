mod db;

use axum::{
    body::Body,
    extract::{Path, Query, Request, State},
    http::{header, HeaderValue, StatusCode},
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::{get, patch, post},
    Json, Router,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chrono::Utc;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;
use std::{
    collections::HashMap,
    env,
    net::SocketAddr,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};
use thiserror::Error;
use tokio::{signal, sync::RwLock};
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor, GovernorError,
    GovernorLayer,
};
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
    demos: Arc<RwLock<HashMap<String, DemoWorkspace>>>,
}

#[derive(Clone)]
struct DemoWorkspace {
    created_at: Instant,
    items: Vec<Item>,
    tokens: Vec<DemoToken>,
}

#[derive(Clone)]
struct DemoToken {
    info: TokenInfo,
    token_hash: String,
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
#[derive(Serialize, sqlx::FromRow, Clone)]
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
struct DemoSession {
    token: String,
    feed_path: String,
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
        demos: Arc::new(RwLock::new(HashMap::new())),
    });
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let listener = tokio::net::TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port))).await?;
    info!(
        port,
        build = BUILD_SHA,
        database = "generated default or supplied override",
        "rss saved queue listening"
    );
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
    let session_limit = governor(1_000, 8);
    let mutation_limit = governor(250, 24);
    let standard_limit = governor(50, 40);
    let limited = Router::new()
        .route(
            "/api/session",
            post(create_session).layer(GovernorLayer {
                config: session_limit,
            }),
        )
        .route("/api/items", get(list_items))
        .route(
            "/api/items",
            post(save_item).layer(GovernorLayer {
                config: mutation_limit.clone(),
            }),
        )
        .route(
            "/api/items/:id",
            patch(update_item).delete(delete_item).layer(GovernorLayer {
                config: mutation_limit.clone(),
            }),
        )
        .route("/api/export.csv", get(export_csv))
        .route("/api/feed-tokens", get(list_feed_tokens))
        .route(
            "/api/feed-tokens",
            post(create_feed_token).layer(GovernorLayer {
                config: mutation_limit.clone(),
            }),
        )
        .route(
            "/api/feed-tokens/:id/revoke",
            post(revoke_feed_token).layer(GovernorLayer {
                config: mutation_limit.clone(),
            }),
        )
        .route("/feed/:token/rss", get(render_feed))
        .route(
            "/reader/:token/items/:id/read",
            post(reader_mark_read).layer(GovernorLayer {
                config: mutation_limit,
            }),
        )
        .route(
            "/api/demo/session",
            post(create_demo_session).delete(delete_demo_session),
        )
        .route("/api/demo/items", get(list_demo_items).post(save_demo_item))
        .route(
            "/api/demo/items/:id",
            patch(update_demo_item).delete(delete_demo_item),
        )
        .route("/api/demo/export.csv", get(export_demo_csv))
        .route(
            "/api/demo/feed-tokens",
            get(list_demo_feed_tokens).post(create_demo_feed_token),
        )
        .route(
            "/api/demo/feed-tokens/:id/revoke",
            post(revoke_demo_feed_token),
        )
        .route("/demo/feed/:token/rss", get(render_demo_feed))
        .route("/", get(index_page))
        .route("/demo", get(index_page))
        .route("/privacy", get(index_page))
        .route("/terms", get(index_page))
        .route("/404", get(not_found_page))
        .nest_service("/assets", ServeDir::new(format!("{static_dir}/assets")))
        .nest_service(
            "/extension",
            ServeDir::new(format!("{static_dir}/extension")),
        )
        .route("/robots.txt", get(static_text))
        .route("/sitemap.xml", get(static_text))
        .route("/favicon.svg", get(static_text))
        .route("/favicon.ico", get(static_bytes))
        .route("/apple-touch-icon.png", get(static_bytes))
        .route("/og-card.png", get(static_bytes))
        .route("/og-card.svg", get(static_text))
        .route("/staticwebapp.config.json", get(static_text))
        .fallback(not_found_page)
        .with_state(state)
        .layer(GovernorLayer {
            config: standard_limit,
        });
    Router::new()
        .route("/health", get(health))
        .merge(limited)
        .layer(middleware::from_fn(cache_headers))
        .layer(SetResponseHeaderLayer::overriding(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff")))
        .layer(SetResponseHeaderLayer::overriding(header::REFERRER_POLICY, HeaderValue::from_static("same-origin")))
        .layer(SetResponseHeaderLayer::overriding(http::HeaderName::from_static("x-frame-options"), HeaderValue::from_static("DENY")))
        .layer(SetResponseHeaderLayer::overriding(http::HeaderName::from_static("permissions-policy"), HeaderValue::from_static("camera=(), microphone=(), geolocation=()")))
        .layer(SetResponseHeaderLayer::overriding(http::HeaderName::from_static("strict-transport-security"), HeaderValue::from_static("max-age=63072000; includeSubDomains; preload")))
        .layer(SetResponseHeaderLayer::overriding(http::HeaderName::from_static("content-security-policy"), HeaderValue::from_static("default-src 'self'; base-uri 'self'; form-action 'self'; frame-ancestors 'none'; object-src 'none'")))
}

fn governor(
    period_ms: u64,
    burst: u32,
) -> Arc<
    tower_governor::governor::GovernorConfig<
        SmartIpKeyExtractor,
        governor::middleware::NoOpMiddleware,
    >,
> {
    let mut builder = GovernorConfigBuilder::default().key_extractor(SmartIpKeyExtractor);
    builder
        .per_millisecond(period_ms)
        .burst_size(burst)
        .error_handler(rate_limit_error);
    Arc::new(builder.finish().expect("valid rate-limit configuration"))
}

fn rate_limit_error(error: GovernorError) -> Response<Body> {
    match error {
        GovernorError::TooManyRequests { wait_time, .. } => Response::builder()
            .status(StatusCode::TOO_MANY_REQUESTS)
            .header(header::RETRY_AFTER, wait_time.max(1).to_string())
            .body(Body::from(
                "Too many requests. Retry after the stated delay.",
            ))
            .expect("valid rate-limit response"),
        _ => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("The client address could not be read."))
            .expect("valid rate-limit response"),
    }
}

async fn cache_headers(request: Request, next: Next) -> Response {
    let path = request.uri().path().to_owned();
    let mut response = next.run(request).await;
    let value = if path.starts_with("/assets/") || path == "/og-card.png" {
        "public, max-age=31536000, immutable"
    } else if path.starts_with("/api/")
        || path.starts_with("/feed/")
        || path.starts_with("/reader/")
        || path.starts_with("/demo/feed/")
        || path == "/health"
    {
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

async fn not_found_page(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Html<String>), AppError> {
    Ok((StatusCode::NOT_FOUND, index_page(State(state)).await?))
}

async fn static_text(
    State(state): State<Arc<AppState>>,
    request: Request,
) -> Result<Response, AppError> {
    static_file(&state, request.uri().path(), true).await
}

async fn static_bytes(
    State(state): State<Arc<AppState>>,
    request: Request,
) -> Result<Response, AppError> {
    static_file(&state, request.uri().path(), false).await
}

async fn static_file(state: &AppState, path: &str, text: bool) -> Result<Response, AppError> {
    let name = path.trim_start_matches('/');
    let bytes = tokio::fs::read(PathBuf::from(&state.static_dir).join(name))
        .await
        .map_err(|_| AppError::not_found("That file does not exist."))?;
    let content_type = match name.rsplit('.').next() {
        Some("txt") => "text/plain; charset=utf-8",
        Some("xml") => "application/xml; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("ico") => "image/x-icon",
        _ if text => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    };
    Ok(([(header::CONTENT_TYPE, content_type)], bytes).into_response())
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
            "Give this saved link a title of up to 300 characters.",
        ));
    }
    let tags = input
        .tags
        .into_iter()
        .map(|tag| tag.trim().to_string())
        .filter(|tag| !tag.is_empty())
        .collect::<Vec<_>>();
    if tags.len() > 12 {
        return Err(AppError::bad("Use up to 12 tags per saved link."));
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

fn demo_key(headers: &axum::http::HeaderMap) -> Result<&str, AppError> {
    headers
        .get("x-demo-workspace")
        .and_then(|value| value.to_str().ok())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| AppError::unauthorized("This demo has ended. Reset it to start again."))
}

fn sample_items() -> Vec<Item> {
    vec![
        Item {
            id: 1,
            title: "A field guide to calmer web typography".into(),
            url: "https://example.com/library/web-typography".into(),
            tags: vec!["design".into(), "typography".into()],
            saved_at: "2026-08-26T09:30:00+00:00".into(),
            status: "queue".into(),
            priority: "next".into(),
        },
        Item {
            id: 2,
            title: "Why a shorter reading list improves recall".into(),
            url: "https://example.com/notes/reading-and-recall".into(),
            tags: vec!["reading".into(), "habits".into()],
            saved_at: "2026-08-24T14:15:00+00:00".into(),
            status: "queue".into(),
            priority: "soon".into(),
        },
        Item {
            id: 3,
            title: "Keep up with the web using private RSS".into(),
            url: "https://example.com/guides/private-rss".into(),
            tags: vec!["rss".into(), "privacy".into()],
            saved_at: "2026-08-21T18:45:00+00:00".into(),
            status: "read".into(),
            priority: "later".into(),
        },
    ]
}

fn demo_is_active(workspace: &DemoWorkspace) -> bool {
    workspace.created_at.elapsed() < Duration::from_secs(86_400)
}

async fn create_demo_session(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<DemoSession>), AppError> {
    let workspace_token = random_token();
    let feed_token = random_token();
    let now = Instant::now();
    let mut demos = state.demos.write().await;
    demos.retain(|_, workspace| {
        now.duration_since(workspace.created_at) < Duration::from_secs(86_400)
    });
    demos.insert(
        workspace_token.clone(),
        DemoWorkspace {
            created_at: now,
            items: sample_items(),
            tokens: vec![DemoToken {
                info: TokenInfo {
                    id: 1,
                    label: "Sample reader".into(),
                    created_at: "2026-08-26T09:30:00+00:00".into(),
                    revoked_at: None,
                },
                token_hash: token_hash(&feed_token),
            }],
        },
    );
    Ok((
        StatusCode::CREATED,
        Json(DemoSession {
            token: workspace_token,
            feed_path: format!("/demo/feed/{feed_token}/rss"),
        }),
    ))
}

async fn delete_demo_session(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
) -> Result<StatusCode, AppError> {
    state.demos.write().await.remove(demo_key(&headers)?);
    Ok(StatusCode::NO_CONTENT)
}

async fn list_demo_items(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Vec<Item>>, AppError> {
    let demos = state.demos.read().await;
    let workspace = demos
        .get(demo_key(&headers)?)
        .filter(|workspace| demo_is_active(workspace))
        .ok_or_else(|| AppError::unauthorized("This demo has ended. Reset it to start again."))?;
    Ok(Json(workspace.items.clone()))
}

async fn save_demo_item(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Json(input): Json<SaveItem>,
) -> Result<(StatusCode, Json<Item>), AppError> {
    let input = clean_save(input)?;
    let mut demos = state.demos.write().await;
    let workspace = demos
        .get_mut(demo_key(&headers)?)
        .filter(|workspace| demo_is_active(workspace))
        .ok_or_else(|| AppError::unauthorized("This demo has ended. Reset it to start again."))?;
    let item = Item {
        id: workspace
            .items
            .iter()
            .map(|item| item.id)
            .max()
            .unwrap_or(0)
            + 1,
        title: input.title,
        url: input.url,
        tags: input.tags,
        saved_at: Utc::now().to_rfc3339(),
        status: "queue".into(),
        priority: "next".into(),
    };
    workspace.items.push(item.clone());
    Ok((StatusCode::CREATED, Json(item)))
}

async fn update_demo_item(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Path(id): Path<i64>,
    Json(input): Json<UpdateItem>,
) -> Result<Json<Item>, AppError> {
    if input.status.is_none() && input.priority.is_none() {
        return Err(AppError::bad("Choose a status or priority to update."));
    }
    if input
        .status
        .as_deref()
        .is_some_and(|value| !["queue", "read", "archived"].contains(&value))
        || input
            .priority
            .as_deref()
            .is_some_and(|value| !["next", "soon", "later"].contains(&value))
    {
        return Err(AppError::bad("Choose a known status and priority."));
    }
    let mut demos = state.demos.write().await;
    let workspace = demos
        .get_mut(demo_key(&headers)?)
        .filter(|workspace| demo_is_active(workspace))
        .ok_or_else(|| AppError::unauthorized("This demo has ended. Reset it to start again."))?;
    let item = workspace
        .items
        .iter_mut()
        .find(|item| item.id == id)
        .ok_or_else(|| AppError::not_found("That demo link no longer exists."))?;
    if let Some(status) = input.status {
        item.status = status;
    }
    if let Some(priority) = input.priority {
        item.priority = priority;
    }
    Ok(Json(item.clone()))
}

async fn delete_demo_item(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let mut demos = state.demos.write().await;
    let workspace = demos
        .get_mut(demo_key(&headers)?)
        .filter(|workspace| demo_is_active(workspace))
        .ok_or_else(|| AppError::unauthorized("This demo has ended. Reset it to start again."))?;
    let before = workspace.items.len();
    workspace.items.retain(|item| item.id != id);
    if before == workspace.items.len() {
        return Err(AppError::not_found("That demo link no longer exists."));
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn list_demo_feed_tokens(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Vec<TokenInfo>>, AppError> {
    let demos = state.demos.read().await;
    let workspace = demos
        .get(demo_key(&headers)?)
        .filter(|workspace| demo_is_active(workspace))
        .ok_or_else(|| AppError::unauthorized("This demo has ended. Reset it to start again."))?;
    Ok(Json(
        workspace
            .tokens
            .iter()
            .map(|token| token.info.clone())
            .collect(),
    ))
}

async fn create_demo_feed_token(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Json(input): Json<CreateToken>,
) -> Result<(StatusCode, Json<CreatedToken>), AppError> {
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
    let raw = random_token();
    let mut demos = state.demos.write().await;
    let workspace = demos
        .get_mut(demo_key(&headers)?)
        .filter(|workspace| demo_is_active(workspace))
        .ok_or_else(|| AppError::unauthorized("This demo has ended. Reset it to start again."))?;
    let id = workspace
        .tokens
        .iter()
        .map(|token| token.info.id)
        .max()
        .unwrap_or(0)
        + 1;
    workspace.tokens.push(DemoToken {
        info: TokenInfo {
            id,
            label: label.clone(),
            created_at: Utc::now().to_rfc3339(),
            revoked_at: None,
        },
        token_hash: token_hash(&raw),
    });
    Ok((
        StatusCode::CREATED,
        Json(CreatedToken {
            id,
            label,
            token: raw.clone(),
            feed_path: format!("/demo/feed/{raw}/rss"),
        }),
    ))
}

async fn revoke_demo_feed_token(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let mut demos = state.demos.write().await;
    let workspace = demos
        .get_mut(demo_key(&headers)?)
        .filter(|workspace| demo_is_active(workspace))
        .ok_or_else(|| AppError::unauthorized("This demo has ended. Reset it to start again."))?;
    let token = workspace
        .tokens
        .iter_mut()
        .find(|token| token.info.id == id && token.info.revoked_at.is_none())
        .ok_or_else(|| AppError::not_found("That demo feed link is already revoked."))?;
    token.info.revoked_at = Some(Utc::now().to_rfc3339());
    Ok(StatusCode::NO_CONTENT)
}

fn csv_for(items: &[Item]) -> String {
    let mut csv = String::from("title,url,tags,status,priority,saved_at\n");
    for item in items {
        let tags = item.tags.join("; ");
        csv.push_str(
            &[
                &item.title,
                &item.url,
                &tags,
                &item.status,
                &item.priority,
                &item.saved_at,
            ]
            .iter()
            .map(|value| format!("\"{}\"", value.replace('"', "\"\"")))
            .collect::<Vec<_>>()
            .join(","),
        );
        csv.push('\n');
    }
    csv
}

async fn export_demo_csv(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
) -> Result<Response, AppError> {
    let demos = state.demos.read().await;
    let workspace = demos
        .get(demo_key(&headers)?)
        .filter(|workspace| demo_is_active(workspace))
        .ok_or_else(|| AppError::unauthorized("This demo has ended. Reset it to start again."))?;
    Ok((
        [
            (header::CONTENT_TYPE, "text/csv; charset=utf-8"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=reading-queue.csv",
            ),
        ],
        csv_for(&workspace.items),
    )
        .into_response())
}

fn rss_for(items: &[Item]) -> Result<String, AppError> {
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?><rss version=\"2.0\"><channel><title>RSS Saved Queue</title><description>Private saved-link queue</description><link>https://rss-saved-queue.sociobot.in</link>");
    for item in items.iter().filter(|item| item.status == "queue") {
        xml.push_str(&format!("<item><title>{}</title><link>{}</link><guid isPermaLink=\"false\">{}</guid><description>{}</description><pubDate>{}</pubDate></item>", xml_escape(&item.title), xml_escape(&item.url), item.id, xml_escape(&item.tags.join(", ")), xml_escape(&rss_pub_date(&item.saved_at)?)));
    }
    xml.push_str("</channel></rss>");
    Ok(xml)
}

async fn render_demo_feed(
    State(state): State<Arc<AppState>>,
    Path(token): Path<String>,
) -> Result<Response, AppError> {
    let hash = token_hash(&token);
    let demos = state.demos.read().await;
    let workspace = demos
        .values()
        .find(|workspace| {
            demo_is_active(workspace)
                && workspace.tokens.iter().any(|candidate| {
                    candidate.token_hash == hash && candidate.info.revoked_at.is_none()
                })
        })
        .ok_or_else(|| AppError::not_found("This demo feed link is unavailable."))?;
    Ok((
        [(header::CONTENT_TYPE, "application/rss+xml; charset=utf-8")],
        rss_for(&workspace.items)?,
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
            demos: Arc::new(RwLock::new(HashMap::new())),
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
    async fn send_response(app: &Router, mut request: Request<Body>) -> Response {
        request.extensions_mut().insert(ConnectInfo(
            "127.0.0.1:41000".parse::<SocketAddr>().unwrap(),
        ));
        app.clone().oneshot(request).await.unwrap()
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
            matches!(error, AppError::Bad(message) if message == "Use up to 12 tags per saved link.")
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
    #[tokio::test]
    async fn state_changing_routes_are_rate_limited_by_peer_address() {
        let app = test_app().await;
        let key = session(&app).await;
        for number in 0..24 {
            let (status, _) = send(
                &app,
                auth(Request::post("/api/items"), &key)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(format!(
                        r#"{{"title":"Item {number}","url":"https://example.com/{number}","tags":[]}}"#
                    )))
                    .unwrap(),
            )
            .await;
            assert_eq!(status, StatusCode::CREATED);
        }
        let (status, _) = send(
            &app,
            auth(Request::post("/api/items"), &key)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    r#"{"title":"One too many","url":"https://example.com/too-many","tags":[]}"#,
                ))
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    }
    #[tokio::test]
    async fn demo_workspace_is_ephemeral_and_cannot_touch_real_queue() {
        let app = test_app().await;
        let real_key = session(&app).await;
        let (status, _) = send(
            &app,
            auth(Request::post("/api/items"), &real_key)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    r#"{"title":"Real link","url":"https://example.com/real","tags":[]}"#,
                ))
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::CREATED);

        let (status, demo) = send(
            &app,
            Request::post("/api/demo/session")
                .body(Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::CREATED);
        let demo_key = serde_json::from_str::<serde_json::Value>(&demo).unwrap()["token"]
            .as_str()
            .unwrap()
            .to_owned();
        let (status, sample) = send(
            &app,
            Request::get("/api/demo/items")
                .header("x-demo-workspace", &demo_key)
                .body(Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(serde_json::from_str::<Vec<Item>>(&sample).unwrap().len(), 3);

        let (status, _) = send(
            &app,
            Request::post("/api/demo/items")
                .header("x-demo-workspace", &demo_key)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    r#"{"title":"Demo only","url":"https://example.com/demo","tags":[]}"#,
                ))
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::CREATED);
        let (_, real_items) = send(
            &app,
            auth(Request::get("/api/items"), &real_key)
                .body(Body::empty())
                .unwrap(),
        )
        .await;
        let real_items = serde_json::from_str::<Vec<Item>>(&real_items).unwrap();
        assert_eq!(real_items.len(), 1);
        assert_eq!(real_items[0].title, "Real link");

        let (status, _) = send(
            &app,
            Request::delete("/api/demo/session")
                .header("x-demo-workspace", &demo_key)
                .body(Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::NO_CONTENT);
        let (status, _) = send(
            &app,
            Request::get("/api/demo/items")
                .header("x-demo-workspace", &demo_key)
                .body(Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }
    #[tokio::test]
    async fn forwarded_client_rate_limit_returns_retry_after() {
        let app = test_app().await;
        for _ in 0..8 {
            let response = send_response(
                &app,
                Request::post("/api/session")
                    .header("x-forwarded-for", "203.0.113.10, 10.0.0.1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await;
            assert_eq!(response.status(), StatusCode::CREATED);
        }
        let limited = send_response(
            &app,
            Request::post("/api/session")
                .header("x-forwarded-for", "203.0.113.10, 10.0.0.1")
                .body(Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(limited.status(), StatusCode::TOO_MANY_REQUESTS);
        assert!(limited.headers().contains_key(header::RETRY_AFTER));

        let other_client = send_response(
            &app,
            Request::post("/api/session")
                .header("x-forwarded-for", "203.0.113.11, 10.0.0.1")
                .body(Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(other_client.status(), StatusCode::CREATED);
    }
}

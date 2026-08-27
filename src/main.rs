mod db;

use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, patch, post},
    Json, Router,
};
use feed_rs::parser;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;
use std::{env, net::SocketAddr, path::PathBuf, sync::Arc};
use thiserror::Error;
use tokio::signal;
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
    client: reqwest::Client,
    static_dir: String,
}
#[derive(Serialize)]
struct Health {
    status: &'static str,
    build: &'static str,
}
#[derive(Serialize, Deserialize, sqlx::FromRow, Clone)]
struct Item {
    id: i64,
    title: String,
    url: String,
    source: String,
    summary: String,
    published_at: Option<String>,
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
#[derive(Deserialize)]
struct AddFeed {
    url: String,
}
#[derive(Serialize)]
struct ImportResult {
    feed_title: String,
    added: usize,
    duplicates: usize,
}
#[derive(Clone)]
struct ParsedItem {
    title: String,
    url: String,
    source: String,
    summary: String,
    published_at: Option<String>,
    hash: String,
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
    #[error(transparent)]
    Db(#[from] sqlx::Error),
    #[error(transparent)]
    Network(#[from] reqwest::Error),
}
impl AppError {
    fn bad(s: impl Into<String>) -> Self {
        Self::Bad(s.into())
    }
    fn not_found(s: impl Into<String>) -> Self {
        Self::Missing(s.into())
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::Bad(_) => StatusCode::BAD_REQUEST,
            Self::Missing(_) => StatusCode::NOT_FOUND,
            Self::Db(ref e) => {
                error!(error=%e, "database error");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::Network(ref e) => {
                error!(error=%e, "feed request failed");
                StatusCode::BAD_GATEWAY
            }
        };
        (
            status,
            Json(ApiMessage {
                error: match self {
                    Self::Db(_) => "The queue could not be updated. Please try again.".into(),
                    Self::Network(_) => {
                        "Could not fetch that feed. Check the URL and try again.".into()
                    }
                    Self::Bad(s) | Self::Missing(s) => s,
                },
            }),
        )
            .into_response()
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
    let static_dir = env::var("STATIC_DIR").unwrap_or_else(|_| "dist".into());
    let state = AppState {
        pool: db::connect(&database_url).await?,
        client: reqwest::Client::builder()
            .user_agent("RSS Saved Queue/1.0 (+https://rss-saved-queue.sociobot.in)")
            .timeout(std::time::Duration::from_secs(15))
            .build()?,
        static_dir: static_dir.clone(),
    };
    let api = Router::new()
        .route("/health", get(health))
        .route("/api/items", get(list_items))
        .route("/api/items/:id", patch(update_item).delete(delete_item))
        .route("/api/feeds", post(import_feed))
        .route("/api/export.csv", get(export_csv))
        .route("/privacy", get(index_page))
        .route("/terms", get(index_page))
        .with_state(Arc::new(state));
    let app = api
        .fallback_service(ServeDir::new(static_dir).append_index_html_on_directories(true))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::REFERRER_POLICY,
            HeaderValue::from_static("same-origin"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            http::HeaderName::from_static("x-frame-options"),
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            http::HeaderName::from_static("content-security-policy"),
            HeaderValue::from_static("default-src 'self'; base-uri 'self'; form-action 'self'; frame-ancestors 'none'; object-src 'none'"),
        ));
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let listener = tokio::net::TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port))).await?;
    info!(port, build = BUILD_SHA, "rss saved queue listening");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
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
async fn list_items(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Item>>, AppError> {
    Ok(Json(
        db::list(
            &state.pool,
            query.status.as_deref(),
            query.search.as_deref(),
        )
        .await?,
    ))
}
async fn update_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateItem>,
) -> Result<Json<Item>, AppError> {
    if input.status.is_none() && input.priority.is_none() {
        return Err(AppError::bad("Choose a status or priority to update."));
    }
    Ok(Json(
        db::update(
            &state.pool,
            id,
            input.status.as_deref(),
            input.priority.as_deref(),
        )
        .await?,
    ))
}
async fn delete_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    db::remove(&state.pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
async fn import_feed(
    State(state): State<Arc<AppState>>,
    Json(input): Json<AddFeed>,
) -> Result<Json<ImportResult>, AppError> {
    let url = validate_feed_url(&input.url)?;
    let response = state
        .client
        .get(url.as_str())
        .send()
        .await?
        .error_for_status()?;
    let bytes = response.bytes().await?;
    if bytes.len() > 3_000_000 {
        return Err(AppError::bad("That feed is too large (maximum 3 MB)."));
    }
    let feed = parser::parse(&bytes[..])
        .map_err(|_| AppError::bad("That URL did not return a readable RSS or Atom feed."))?;
    let title = feed
        .title
        .as_ref()
        .map(|v| v.content.trim())
        .filter(|v| !v.is_empty())
        .unwrap_or("Untitled feed")
        .to_string();
    let feed_id = db::insert_feed(&state.pool, url.as_str(), &title).await?;
    let mut added = 0;
    let mut duplicates = 0;
    for entry in feed.entries.into_iter().take(100) {
        let link = entry
            .links
            .iter()
            .find(|l| l.rel.as_deref().map_or(true, |r| r == "alternate"))
            .map(|l| l.href.clone());
        if let Some(link) = link {
            let title_text = entry
                .title
                .as_ref()
                .map(|t| t.content.trim())
                .filter(|t| !t.is_empty())
                .unwrap_or("Untitled article")
                .to_string();
            let summary = entry
                .summary
                .as_ref()
                .map(|s| strip_html(&s.content))
                .unwrap_or_default();
            let published_at = entry.published.or(entry.updated).map(|d| d.to_rfc3339());
            let mut hasher = Sha256::new();
            hasher.update(&link);
            hasher.update(&title_text);
            let item = ParsedItem {
                title: title_text,
                url: link,
                source: title.clone(),
                summary,
                published_at,
                hash: format!("{:x}", hasher.finalize()),
            };
            if db::insert_item(&state.pool, feed_id, &item).await? {
                added += 1;
            } else {
                duplicates += 1;
            }
        }
    }
    Ok(Json(ImportResult {
        feed_title: title,
        added,
        duplicates,
    }))
}
fn validate_feed_url(raw: &str) -> Result<Url, AppError> {
    let url = Url::parse(raw.trim())
        .map_err(|_| AppError::bad("Enter a complete http or https feed URL."))?;
    if !["http", "https"].contains(&url.scheme())
        || url.host_str().is_none()
        || url.username() != ""
        || url.password().is_some()
    {
        return Err(AppError::bad("Enter a public http or https feed URL."));
    }
    let host = url.host_str().unwrap_or("");
    let private_ip = host
        .parse::<std::net::IpAddr>()
        .map(|ip| match ip {
            std::net::IpAddr::V4(v4) => {
                v4.is_loopback() || v4.is_private() || v4.is_unspecified() || v4.is_link_local()
            }
            std::net::IpAddr::V6(v6) => v6.is_loopback() || v6.is_unspecified(),
        })
        .unwrap_or(false);
    if host.eq_ignore_ascii_case("localhost") || host.ends_with(".local") || private_ip {
        return Err(AppError::bad("Private network addresses are not allowed."));
    }
    Ok(url)
}
fn strip_html(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut inside = false;
    for c in raw.chars() {
        match c {
            '<' => inside = true,
            '>' => {
                inside = false;
                out.push(' ');
            }
            _ if !inside => out.push(c),
            _ => {}
        }
    }
    out.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(280)
        .collect()
}
async fn export_csv(State(state): State<Arc<AppState>>) -> Result<Response, AppError> {
    let items = db::list(&state.pool, None, None).await?;
    let mut csv = String::from("title,source,url,status,priority,published_at,saved_at\n");
    for item in items {
        csv.push_str(
            &[
                item.title,
                item.source,
                item.url,
                item.status,
                item.priority,
                item.published_at.unwrap_or_default(),
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
    #[test]
    fn permits_public_https() {
        assert!(validate_feed_url("https://example.com/feed.xml").is_ok());
    }
    #[test]
    fn rejects_private_targets() {
        assert!(validate_feed_url("http://127.0.0.1/feed").is_err());
        assert!(validate_feed_url("ftp://example.com/feed").is_err());
    }
    #[test]
    fn removes_markup() {
        assert_eq!(strip_html("<p>Hello <em>world</em></p>"), "Hello world");
    }
}

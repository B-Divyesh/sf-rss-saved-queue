use crate::{AppError, Item};
use chrono::Utc;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub async fn connect(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

pub async fn list(
    pool: &SqlitePool,
    status: Option<&str>,
    search: Option<&str>,
) -> Result<Vec<Item>, AppError> {
    let term = search.unwrap_or("").trim();
    let rows = sqlx::query_as::<_, Item>(
        "SELECT id, title, url, source, summary, published_at, saved_at, status, priority FROM items
         WHERE (?1 = '' OR status = ?1) AND (?2 = '' OR title LIKE '%' || ?2 || '%' OR source LIKE '%' || ?2 || '%')
         ORDER BY CASE priority WHEN 'next' THEN 0 WHEN 'soon' THEN 1 ELSE 2 END, saved_at DESC"
    ).bind(status.unwrap_or("")).bind(term).fetch_all(pool).await?;
    Ok(rows)
}

pub async fn update(
    pool: &SqlitePool,
    id: i64,
    status: Option<&str>,
    priority: Option<&str>,
) -> Result<Item, AppError> {
    if let Some(value) = status {
        if !["queue", "read", "archived"].contains(&value) {
            return Err(AppError::bad("Unknown status."));
        }
    }
    if let Some(value) = priority {
        if !["next", "soon", "later"].contains(&value) {
            return Err(AppError::bad("Unknown priority."));
        }
    }
    sqlx::query("UPDATE items SET status = COALESCE(?1, status), priority = COALESCE(?2, priority) WHERE id = ?3")
        .bind(status).bind(priority).bind(id).execute(pool).await?;
    sqlx::query_as::<_, Item>("SELECT id, title, url, source, summary, published_at, saved_at, status, priority FROM items WHERE id=?1")
        .bind(id).fetch_optional(pool).await?.ok_or_else(|| AppError::not_found("That saved item no longer exists."))
}

pub async fn remove(pool: &SqlitePool, id: i64) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM items WHERE id=?1")
        .bind(id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::not_found("That saved item no longer exists."));
    }
    Ok(())
}

pub async fn insert_feed(pool: &SqlitePool, url: &str, title: &str) -> Result<i64, AppError> {
    let now = Utc::now().to_rfc3339();
    sqlx::query("INSERT INTO feeds(url,title,created_at) VALUES (?1,?2,?3) ON CONFLICT(url) DO UPDATE SET title=excluded.title")
        .bind(url).bind(title).bind(now).execute(pool).await?;
    Ok(sqlx::query_scalar("SELECT id FROM feeds WHERE url=?1")
        .bind(url)
        .fetch_one(pool)
        .await?)
}

pub async fn insert_item(
    pool: &SqlitePool,
    feed_id: i64,
    item: &crate::ParsedItem,
) -> Result<bool, AppError> {
    let now = Utc::now().to_rfc3339();
    let result = sqlx::query("INSERT OR IGNORE INTO items(feed_id,title,url,source,summary,published_at,saved_at,content_hash) VALUES (?1,?2,?3,?4,?5,?6,?7,?8)")
        .bind(feed_id).bind(&item.title).bind(&item.url).bind(&item.source).bind(&item.summary).bind(&item.published_at).bind(now).bind(&item.hash)
        .execute(pool).await?;
    Ok(result.rows_affected() == 1)
}

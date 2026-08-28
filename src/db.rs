use crate::{AppError, Item, SaveItem, TokenInfo};
use chrono::Utc;
use sqlx::{sqlite::SqlitePoolOptions, FromRow, SqlitePool};

#[derive(FromRow)]
struct ItemRow {
    id: i64,
    title: String,
    url: String,
    tags_json: String,
    saved_at: String,
    status: String,
    priority: String,
}

impl TryFrom<ItemRow> for Item {
    type Error = AppError;

    fn try_from(row: ItemRow) -> Result<Self, Self::Error> {
        let tags = serde_json::from_str(&row.tags_json)
            .map_err(|_| AppError::internal("Saved tags could not be read."))?;
        Ok(Item {
            id: row.id,
            title: row.title,
            url: row.url,
            tags,
            saved_at: row.saved_at,
            status: row.status,
            priority: row.priority,
        })
    }
}

pub async fn connect(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

pub async fn create_account(pool: &SqlitePool, token_hash: &str) -> Result<(), AppError> {
    sqlx::query("INSERT INTO accounts(token_hash,created_at) VALUES (?1,?2)")
        .bind(token_hash)
        .bind(Utc::now().to_rfc3339())
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn account_for_token(
    pool: &SqlitePool,
    token_hash: &str,
) -> Result<Option<i64>, AppError> {
    Ok(
        sqlx::query_scalar("SELECT id FROM accounts WHERE token_hash=?1")
            .bind(token_hash)
            .fetch_optional(pool)
            .await?,
    )
}

pub async fn list(
    pool: &SqlitePool,
    account_id: i64,
    status: Option<&str>,
    search: Option<&str>,
) -> Result<Vec<Item>, AppError> {
    let term = search.unwrap_or("").trim();
    let rows = sqlx::query_as::<_, ItemRow>(
        "SELECT id,title,url,tags_json,saved_at,status,priority FROM saved_items
         WHERE account_id=?1 AND (?2='' OR status=?2)
           AND (?3='' OR title LIKE '%' || ?3 || '%' OR url LIKE '%' || ?3 || '%' OR tags_json LIKE '%' || ?3 || '%')
         ORDER BY CASE priority WHEN 'next' THEN 0 WHEN 'soon' THEN 1 ELSE 2 END, saved_at DESC",
    ).bind(account_id).bind(status.unwrap_or("")).bind(term).fetch_all(pool).await?;
    rows.into_iter().map(Item::try_from).collect()
}

pub async fn save(pool: &SqlitePool, account_id: i64, input: &SaveItem) -> Result<Item, AppError> {
    let tags = serde_json::to_string(&input.tags)
        .map_err(|_| AppError::internal("Saved tags could not be written."))?;
    let result = sqlx::query(
        "INSERT INTO saved_items(account_id,title,url,tags_json,saved_at) VALUES (?1,?2,?3,?4,?5)",
    )
    .bind(account_id)
    .bind(&input.title)
    .bind(&input.url)
    .bind(tags)
    .bind(Utc::now().to_rfc3339())
    .execute(pool)
    .await?;
    get(pool, account_id, result.last_insert_rowid()).await
}

pub async fn duplicate_exists(
    pool: &SqlitePool,
    account_id: i64,
    title: &str,
    url: &str,
) -> Result<bool, AppError> {
    Ok(sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM saved_items WHERE account_id=?1 AND title=?2 AND url=?3",
    )
    .bind(account_id)
    .bind(title)
    .bind(url)
    .fetch_one(pool)
    .await?
        > 0)
}

pub async fn get(pool: &SqlitePool, account_id: i64, id: i64) -> Result<Item, AppError> {
    let row = sqlx::query_as::<_, ItemRow>("SELECT id,title,url,tags_json,saved_at,status,priority FROM saved_items WHERE id=?1 AND account_id=?2")
        .bind(id).bind(account_id).fetch_optional(pool).await?;
    row.ok_or_else(|| AppError::not_found("That saved item no longer exists."))
        .and_then(Item::try_from)
}

pub async fn update(
    pool: &SqlitePool,
    account_id: i64,
    id: i64,
    status: Option<&str>,
    priority: Option<&str>,
) -> Result<Item, AppError> {
    if status.is_some_and(|value| !["queue", "read", "archived"].contains(&value)) {
        return Err(AppError::bad("Unknown status."));
    }
    if priority.is_some_and(|value| !["next", "soon", "later"].contains(&value)) {
        return Err(AppError::bad("Unknown priority."));
    }
    let result = sqlx::query("UPDATE saved_items SET status=COALESCE(?1,status),priority=COALESCE(?2,priority) WHERE id=?3 AND account_id=?4")
        .bind(status).bind(priority).bind(id).bind(account_id).execute(pool).await?;
    if result.rows_affected() == 0 {
        return Err(AppError::not_found("That saved item no longer exists."));
    }
    get(pool, account_id, id).await
}

pub async fn remove(pool: &SqlitePool, account_id: i64, id: i64) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM saved_items WHERE id=?1 AND account_id=?2")
        .bind(id)
        .bind(account_id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::not_found("That saved item no longer exists."));
    }
    Ok(())
}

pub async fn create_feed_token(
    pool: &SqlitePool,
    account_id: i64,
    token_hash: &str,
    label: &str,
) -> Result<TokenInfo, AppError> {
    let created_at = Utc::now().to_rfc3339();
    let result = sqlx::query(
        "INSERT INTO feed_tokens(account_id,token_hash,label,created_at) VALUES (?1,?2,?3,?4)",
    )
    .bind(account_id)
    .bind(token_hash)
    .bind(label)
    .bind(&created_at)
    .execute(pool)
    .await?;
    Ok(TokenInfo {
        id: result.last_insert_rowid(),
        label: label.into(),
        created_at,
        revoked_at: None,
    })
}

pub async fn list_feed_tokens(
    pool: &SqlitePool,
    account_id: i64,
) -> Result<Vec<TokenInfo>, AppError> {
    Ok(sqlx::query_as::<_, TokenInfo>("SELECT id,label,created_at,revoked_at FROM feed_tokens WHERE account_id=?1 ORDER BY created_at DESC")
        .bind(account_id).fetch_all(pool).await?)
}

pub async fn account_for_feed_token(
    pool: &SqlitePool,
    token_hash: &str,
) -> Result<Option<i64>, AppError> {
    Ok(sqlx::query_scalar(
        "SELECT account_id FROM feed_tokens WHERE token_hash=?1 AND revoked_at IS NULL",
    )
    .bind(token_hash)
    .fetch_optional(pool)
    .await?)
}

pub async fn revoke_feed_token(
    pool: &SqlitePool,
    account_id: i64,
    id: i64,
) -> Result<(), AppError> {
    let result = sqlx::query(
        "UPDATE feed_tokens SET revoked_at=?1 WHERE id=?2 AND account_id=?3 AND revoked_at IS NULL",
    )
    .bind(Utc::now().to_rfc3339())
    .bind(id)
    .bind(account_id)
    .execute(pool)
    .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::not_found(
            "That feed token no longer exists or is already revoked.",
        ));
    }
    Ok(())
}

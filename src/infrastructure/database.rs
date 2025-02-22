use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::url::Url;
use crate::error::Result;

pub async fn create_url(
    pool: &PgPool,
    original_url: &str,
    short_code: &str,
    expires_at: Option<time::OffsetDateTime>,
) -> Result<Url> {
    let url = sqlx::query_as!(
        Url,
        r#"
        INSERT INTO urls (original_url, short_code, expires_at)
        VALUES ($1, $2, $3)
        RETURNING id, original_url, short_code, visits, created_at, expires_at
        "#,
        original_url,
        short_code,
        expires_at,
    )
    .fetch_one(pool)
    .await?;

    Ok(url)
}

pub async fn get_url_by_code(pool: &PgPool, short_code: &str) -> Result<Option<Url>> {
    let url = sqlx::query_as!(
        Url,
        r#"
        SELECT id, original_url, short_code, visits, created_at, expires_at
        FROM urls
        WHERE short_code = $1
        AND (expires_at IS NULL OR expires_at > CURRENT_TIMESTAMP)
        "#,
        short_code
    )
    .fetch_optional(pool)
    .await?;

    Ok(url)
}

pub async fn get_url_stats(pool: &PgPool) -> Result<Vec<Url>> {
    let urls = sqlx::query_as!(
        Url,
        r#"
        SELECT id, original_url, short_code, visits, created_at, expires_at
        FROM urls
        ORDER BY visits DESC
        LIMIT 10
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(urls)
}

pub async fn get_stats_summary(pool: &PgPool) -> Result<(i64, i64)> {
    let row = sqlx::query!(
        r#"
        SELECT 
            COUNT(*) as total_urls,
            COALESCE(SUM(visits), 0) as total_visits
        FROM urls
        "#
    )
    .fetch_one(pool)
    .await?;

    Ok((row.total_urls, row.total_visits.unwrap_or(0) as i64))
}

pub async fn increment_visits(pool: &PgPool, url_id: Uuid) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE urls 
        SET visits = visits + 1 
        WHERE id = $1
        "#,
        url_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

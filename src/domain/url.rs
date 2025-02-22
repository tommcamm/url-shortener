use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Url {
    pub id: Uuid,
    pub original_url: String,
    pub short_code: String,
    pub visits: i64,
    pub created_at: OffsetDateTime,
    pub expires_at: Option<OffsetDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUrlRequest {
    pub url: String,
    pub expires_in_days: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct CreateUrlResponse {
    pub id: Uuid,
    pub original_url: String,
    pub short_url: String,
    pub expires_at: Option<OffsetDateTime>,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub total_urls: i64,
    pub total_visits: i64,
    pub urls: Vec<Url>,
}

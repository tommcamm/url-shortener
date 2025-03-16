use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Url {
    pub id: Uuid,
    pub original_url: String,
    pub short_code: String,
    pub visits: i64,
    pub created_at: OffsetDateTime,
    pub expires_at: Option<OffsetDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUrlRequest {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in_days: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUrlResponse {
    pub id: Uuid,
    pub original_url: String,
    pub short_url: String,
    pub expires_at: Option<OffsetDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlStats {
    pub id: Uuid,
    pub original_url: String,
    pub short_code: String,
    pub visits: i64,
    pub created_at: OffsetDateTime,
    pub expires_at: Option<OffsetDateTime>,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub total_urls: i64,
    pub total_visits: i64,
    pub urls: Vec<UrlStats>,
}

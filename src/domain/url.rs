use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct Url {
    pub id: Uuid,
    pub original_url: String,
    pub short_code: String,
    pub visits: i64,
    pub created_at: OffsetDateTime,
    pub expires_at: Option<OffsetDateTime>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example = json!({
    "url": "https://example.com",
    "expires_in_days": 1805226240
}))]
pub struct CreateUrlRequest {
    pub url: String,
    pub expires_in_days: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateUrlResponse {
    pub id: Uuid,
    pub original_url: String,
    pub short_url: String,
    pub expires_at: Option<OffsetDateTime>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct StatsResponse {
    pub total_urls: i64,
    pub total_visits: i64,
    pub urls: Vec<Url>,
}

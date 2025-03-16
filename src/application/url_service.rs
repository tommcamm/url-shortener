use nanoid::nanoid;
use time::Duration;

use crate::{
    config::AppConfig,
    domain::url::{CreateUrlRequest, CreateUrlResponse, StatsResponse},
    error::{AppError, Result},
    infrastructure::{
        cache::Cache,
        database,
    },
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct UrlService {
    config: AppConfig,
    db: PgPool,
    cache: Cache,
}

impl UrlService {
    pub fn new(config: AppConfig, db: PgPool, cache: Cache) -> Self {
        Self { config, db, cache }
    }

    pub fn get_api_key(&self) -> &str {
        &self.config.api_key
    }

    pub async fn create_short_url(&self, request: CreateUrlRequest) -> Result<CreateUrlResponse> {
        let short_code = nanoid!(8);
        
        // Calculate expiration date if provided
        let expires_at = request.expires_in_days.map(|days| {
            time::OffsetDateTime::now_utc() + Duration::days(days as i64)
        });

        // Create URL in database
        let url = database::create_url(&self.db, &request.url, &short_code, expires_at).await?;

        // Cache the URL
        let cache_key = Cache::url_cache_key(&short_code);
        self.cache
            .set_with_expiry(&cache_key, &url.original_url, 3600)
            .await?;

        // Create response
        Ok(CreateUrlResponse {
            id: url.id,
            original_url: url.original_url,
            short_url: format!("{}/{}", self.config.base_url, url.short_code),
            expires_at: url.expires_at,
        })
    }

    pub async fn get_url(&self, short_code: &str) -> Result<String> {
        // Try to get URL from cache first
        let cache_key = Cache::url_cache_key(short_code);
        if let Some(url) = self.cache.get(&cache_key).await? {
            return Ok(url);
        }

        // If not in cache, get from database
        let url = database::get_url_by_code(&self.db, short_code)
            .await?
            .ok_or_else(|| AppError::NotFound("URL not found".to_string()))?;

        // Cache the URL for future requests
        self.cache
            .set_with_expiry(&cache_key, &url.original_url, 3600)
            .await?;

        // Increment visit count
        database::increment_visits(&self.db, url.id).await?;

        Ok(url.original_url)
    }

    pub async fn get_stats(&self) -> Result<StatsResponse> {
        let urls = database::get_url_stats(&self.db).await?;
        let (total_urls, total_visits) = database::get_stats_summary(&self.db).await?;

        Ok(StatsResponse {
            total_urls,
            total_visits,
            urls,
        })
    }
    
    pub async fn check_database_connection(&self) -> anyhow::Result<()> {
        // Simple query to check DB connectivity
        sqlx::query("SELECT 1")
            .execute(&self.db)
            .await
            .map(|_| ())
            .map_err(|e| anyhow::anyhow!("Database connection check failed: {}", e))
    }
    
    pub async fn check_cache_connection(&self) -> anyhow::Result<()> {
        // Use the ping method to check Redis connectivity
        self.cache.ping().await
            .map_err(|e| anyhow::anyhow!("Redis connection check failed: {}", e))
    }
}

use anyhow::Context;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub redis_url: String,
    pub api_key: String,
    pub base_url: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .context("DATABASE_URL must be set")?,
            redis_url: std::env::var("REDIS_URL")
                .context("REDIS_URL must be set")?,
            api_key: std::env::var("API_KEY")
                .context("API_KEY must be set")?,
            base_url: std::env::var("BASE_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
        })
    }
}

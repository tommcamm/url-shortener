mod api;
mod application;
mod config;
mod domain;
mod error;
mod infrastructure;
mod models;

use crate::{
    api::routes::{admin_routes, url_routes, health_routes},
    application::url_service::UrlService,
    config::AppConfig,
    infrastructure::cache::Cache,
};
use axum::Router;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn connect_with_retry<F, Fut, T, E>(
    connect_fn: F,
    max_attempts: usize,
    service_name: &str,
) -> Result<T, anyhow::Error>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display + std::error::Error + Send + Sync + 'static,
{
    let mut attempt = 1;
    let mut delay = std::time::Duration::from_millis(100);

    loop {
        match connect_fn().await {
            Ok(conn) => {
                tracing::info!("Successfully connected to {}", service_name);
                return Ok(conn);
            }
            Err(err) => {
                if attempt >= max_attempts {
                    return Err(anyhow::anyhow!(
                        "Failed to connect to {} after {} attempts: {}",
                        service_name,
                        max_attempts,
                        err
                    ));
                }

                tracing::warn!(
                    "Failed to connect to {} (attempt {}/{}): {}. Retrying in {:?}...",
                    service_name,
                    attempt,
                    max_attempts,
                    err,
                    delay
                );

                tokio::time::sleep(delay).await;
                attempt += 1;
                // Exponential backoff with jitter
                delay = std::time::Duration::from_millis(
                    (delay.as_millis() as u64 * 2).min(5000)
                        + (rand::random::<u64>() % 100),
                );
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    dotenv::dotenv().ok();
    let config = AppConfig::from_env()?;

    // Initialize PostgreSQL connection pool with retry logic
    tracing::info!("Connecting to PostgreSQL...");
    let postgres_pool = connect_with_retry(
        || {
            sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(&config.database_url)
        },
        30, // Increased retry attempts for container startup
        "PostgreSQL",
    )
    .await?;

    // Run migrations
    tracing::info!("Running database migrations...");
    sqlx::query(include_str!("../migrations/20240214_create_urls_table.sql"))
        .execute(&postgres_pool)
        .await?;

    // Initialize Redis client with retry logic
    tracing::info!("Connecting to Redis...");
    let redis_client = redis::Client::open(config.redis_url.clone())?;
    let redis_conn = connect_with_retry(
        || redis_client.get_connection_manager(),
        10,
        "Redis"
    ).await?;
    let cache = Cache::new(redis_conn);

    // Initialize URL service
    let url_service = UrlService::new(config, postgres_pool, cache);

    // Create router with all routes
    let app = Router::new()
        .merge(health_routes())
        .merge(url_routes())
        .merge(admin_routes())
        .layer(TraceLayer::new_for_http())
        .with_state(url_service);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("listening on 0.0.0.0:3000");
    axum::serve(listener, app).await?;
    Ok(())
}

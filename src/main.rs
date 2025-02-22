mod api;
mod application;
mod config;
mod domain;
mod error;
mod infrastructure;
mod models;

use crate::{
    api::routes::{admin_routes, url_routes},
    application::url_service::UrlService,
    config::AppConfig,
    infrastructure::cache::Cache,
};
use axum::Router;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

    // Start PostgreSQL and Redis containers
    tracing::info!("Starting Docker containers...");
    let docker_status = std::process::Command::new("docker-compose")
        .arg("up")
        .arg("-d")
        .status()?;

    if !docker_status.success() {
        anyhow::bail!("Failed to start Docker containers");
    }

    // Wait a bit for services to be ready
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Initialize PostgreSQL connection pool
    let postgres_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    // Run migrations
    tracing::info!("Running database migrations...");
    sqlx::query(include_str!("../migrations/20240214_create_urls_table.sql"))
        .execute(&postgres_pool)
        .await?;

    // Initialize Redis client
    let redis_client = redis::Client::open(config.redis_url.clone())?;
    let redis_conn = redis_client.get_connection_manager()
        .await.expect("Failed to connect to Redis");
    let cache = Cache::new(redis_conn);

    // Initialize URL service
    let url_service = UrlService::new(config, postgres_pool, cache);

    // Create router with all routes
    let app = Router::new()
        .merge(url_routes())
        .merge(admin_routes())
        .layer(TraceLayer::new_for_http())
        .with_state(url_service);

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::info!("listening on 127.0.0.1:3000");
    axum::serve(listener, app).await?;
    Ok(())
}

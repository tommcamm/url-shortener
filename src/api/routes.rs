use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use crate::{
    api::{handlers, middleware::api_key_auth},
    application::url_service::UrlService,
};

pub fn url_routes() -> Router<UrlService> {
    Router::new()
        .route("/api/urls", post(handlers::create_short_url))
        .route("/{short_code}", get(handlers::redirect_to_url))
}

pub fn admin_routes() -> Router<UrlService> {
    Router::new()
        .route("/api/stats", get(handlers::get_stats))
        .route_layer(middleware::from_fn(api_key_auth))
}

pub fn health_routes() -> Router<UrlService> {
    Router::new().route("/health", get(health_check))
}

async fn health_check(State(url_service): State<UrlService>) -> impl IntoResponse {
    // Check database and cache connectivity
    let db_status = url_service.check_database_connection().await;
    let cache_status = url_service.check_cache_connection().await;

    if db_status.is_ok() && cache_status.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}

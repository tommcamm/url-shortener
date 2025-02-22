use axum::{
    middleware,
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
        .route("/:short_code", get(handlers::redirect_to_url))
}

pub fn admin_routes() -> Router<UrlService> {
    Router::new()
        .route("/api/stats", get(handlers::get_stats))
        .route_layer(middleware::from_fn(api_key_auth))
}

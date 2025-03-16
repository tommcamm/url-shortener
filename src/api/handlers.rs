use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};

use crate::{
    application::url_service::UrlService,
    domain::url::{CreateUrlRequest, CreateUrlResponse, StatsResponse},
    error::Result,
};

/// Create a short URL
///
/// Creates a new shortened URL from the original URL provided.
#[utoipa::path(
    post,
    path = "/api/urls",
    request_body = CreateUrlRequest,
    responses(
        (status = 200, description = "Short URL created successfully", body = CreateUrlResponse),
        (status = 400, description = "Invalid URL provided"),
        (status = 500, description = "Internal server error")
    ),
    tag = "URL Shortener API"
)]
pub async fn create_short_url(
    State(service): State<UrlService>,
    Json(request): Json<CreateUrlRequest>,
) -> impl IntoResponse {
    match service.create_short_url(request).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(err) => {
            // Log the error
            tracing::error!("Error creating short URL: {:?}", err);

            // Convert the error to a response
            err.into_response()
        }
    }
}

/// Redirect to original URL
///
/// Redirects to the original URL associated with the provided short code.
#[utoipa::path(
    get,
    path = "/{short_code}",
    params(
        ("short_code" = String, Path, description = "Short code for the URL")
    ),
    responses(
        (status = 302, description = "Redirect to the original URL"),
        (status = 404, description = "Short URL not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "URL Shortener API"
)]
pub async fn redirect_to_url(
    State(service): State<UrlService>,
    Path(short_code): Path<String>,
) -> impl IntoResponse {
    match service.get_url(&short_code).await {
        Ok(url) => Redirect::temporary(&url).into_response(),
        Err(err) => {
            // Log the error
            tracing::error!("Error redirecting to URL: {:?}", err);

            // Convert the error to a response
            err.into_response()
        }
    }
}

/// Get URL statistics
///
/// Returns statistics about all shortened URLs in the system.
#[utoipa::path(
    get,
    path = "/api/stats",
    responses(
        (status = 200, description = "Statistics retrieved successfully", body = StatsResponse),
        (status = 401, description = "Unauthorized - Invalid or missing API key"),
        (status = 500, description = "Internal server error")
    ),
    tag = "URL Shortener API"
)]
pub async fn get_stats(State(service): State<UrlService>) -> impl IntoResponse {
    match service.get_stats().await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(err) => {
            // Log the error
            tracing::error!("Error getting stats: {:?}", err);

            // Convert the error to a response
            err.into_response()
        }
    }
}

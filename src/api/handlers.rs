use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};

use crate::{
    application::url_service::UrlService,
    domain::url::{CreateUrlRequest, CreateUrlResponse, StatsResponse},
    error::ErrorResponse,
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
    // Debug log the request
    tracing::debug!("Received create short URL request: {:?}", request);

    match service.create_short_url(request).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(err) => {
            // Create error response with environment
            ErrorResponse::new(err, service.get_environment()).into_response()
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
    // Debug log the request
    tracing::debug!("Received redirect request for short code: {}", short_code);

    match service.get_url(&short_code).await {
        Ok(url) => Redirect::temporary(&url).into_response(),
        Err(err) => {
            // Create error response with environment
            ErrorResponse::new(err, service.get_environment()).into_response()
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
    // Debug log the request
    tracing::debug!("Received stats request");

    match service.get_stats().await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(err) => {
            // Create error response with environment
            ErrorResponse::new(err, service.get_environment()).into_response()
        }
    }
}

use axum::{body::Body, http::Request, middleware::Next, response::IntoResponse};

use crate::{application::url_service::UrlService, error::AppError};

pub async fn api_key_auth(
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // Extract the API key from request headers
    let auth_header = req
        .headers()
        .get("X-API-KEY")
        .and_then(|header| header.to_str().ok());

    // Get the service instance from the request extensions
    let service = req
        .extensions()
        .get::<UrlService>()
        .expect("UrlService not found in request extensions");

    let api_key = service.get_api_key();

    match auth_header {
        Some(key) if key == api_key => Ok(next.run(req).await),
        _ => {
            tracing::warn!("Unauthorized API access attempt");
            Err(AppError::Unauthorized.into_response())
        }
    }
}

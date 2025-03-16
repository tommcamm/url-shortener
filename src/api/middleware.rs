use axum::{body::Body, extract::State, http::Request, middleware::Next, response::IntoResponse};

use crate::{application::url_service::UrlService, error::AppError};

pub async fn api_key_auth(
    State(service): State<UrlService>,
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let auth_header = req
        .headers()
        .get("X-API-KEY")
        .and_then(|header| header.to_str().ok());

    let api_key = service.get_api_key();

    match auth_header {
        Some(key) if key == api_key => Ok(next.run(req).await),
        _ => {
            tracing::warn!("Unauthorized API access attempt");
            Err(AppError::Unauthorized.into_response())
        }
    }
}

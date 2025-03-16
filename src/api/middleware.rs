use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::application::url_service::UrlService;

pub async fn api_key_auth(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // Extract service from request extensions
    let service = req
        .extensions()
        .get::<UrlService>()
        .expect("Missing UrlService state");

    let api_key = req
        .headers()
        .get("X-API-Key")
        .and_then(|value| value.to_str().ok());

    match api_key {
        Some(key) if key == service.get_api_key() => Ok(next.run(req).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::application::url_service::UrlService;

pub async fn api_key_auth<B>(
    State(service): State<UrlService>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let api_key = req
        .headers()
        .get("X-API-Key")
        .and_then(|value| value.to_str().ok());

    match api_key {
        Some(key) if key == service.get_api_key() => Ok(next.run(req).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

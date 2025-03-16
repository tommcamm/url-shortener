use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    application::url_service::UrlService,
    domain::url::{CreateUrlRequest, CreateUrlResponse, StatsResponse, Url},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::create_short_url,
        crate::api::handlers::redirect_to_url,
        crate::api::handlers::get_stats,
    ),
    components(
        schemas(CreateUrlRequest, CreateUrlResponse, StatsResponse, Url)
    ),
    tags(
        (name = "URL Shortener API", description = "URL shortening service endpoints")
    ),
    info(
        title = "URL Shortener API",
        version = "1.0.0",
        description = "A simple URL shortening service API"
    )
)]
pub struct ApiDoc;

pub fn swagger_routes() -> Router<UrlService> {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}

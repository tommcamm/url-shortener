use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
    Json,
};

use crate::{
    application::url_service::UrlService,
    domain::url::{CreateUrlRequest, CreateUrlResponse, StatsResponse},
    error::Result,
};

pub async fn create_short_url(
    State(service): State<UrlService>,
    Json(request): Json<CreateUrlRequest>,
) -> Result<Json<CreateUrlResponse>> {
    let response = service.create_short_url(request).await?;
    Ok(Json(response))
}

pub async fn redirect_to_url(
    State(service): State<UrlService>,
    Path(short_code): Path<String>,
) -> Result<impl IntoResponse> {
    let url = service.get_url(&short_code).await?;
    Ok(Redirect::temporary(&url))
}

pub async fn get_stats(State(service): State<UrlService>) -> Result<Json<StatsResponse>> {
    let response = service.get_stats().await?;
    Ok(Json(response))
}

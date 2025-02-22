use axum::{
    extract::{Path, State},
    http::StatusCode,
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
) -> Result<(StatusCode, String)> {
    let url = service.get_url(&short_code).await?;
    Ok((StatusCode::TEMPORARY_REDIRECT, url))
}

pub async fn get_stats(
    State(service): State<UrlService>
) -> Result<Json<StatsResponse>> {
    let response = service.get_stats().await?;
    Ok(Json(response))
}

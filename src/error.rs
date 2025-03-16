use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(ref e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            ),
            AppError::Redis(ref e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Cache error: {}", e),
            ),
            AppError::NotFound(ref message) => (StatusCode::NOT_FOUND, message.clone()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::BadRequest(ref message) => (StatusCode::BAD_REQUEST, message.clone()),
            AppError::Internal(ref message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message.clone())
            }
        };

        let body = Json(json!({
            "error": message
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

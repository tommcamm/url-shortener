use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::error::Error as StdError;
use thiserror::Error;

use crate::config::Environment;

// Allow unused variants as they might be used in the future
#[allow(dead_code)]
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

impl AppError {
    // Enhanced logging for errors
    fn log_error(&self) {
        match self {
            AppError::Database(ref e) => {
                tracing::error!("Database error: {}", e);
                // Log additional database-specific context if available
                if let Some(source) = e.source() {
                    tracing::error!("Caused by: {}", source);
                }
            }
            AppError::Redis(ref e) => {
                tracing::error!("Redis error: {}", e);
            }
            AppError::NotFound(ref message) => {
                tracing::debug!("Not found: {}", message);
            }
            AppError::Unauthorized => {
                tracing::warn!("Unauthorized access attempt");
            }
            AppError::BadRequest(ref message) => {
                tracing::debug!("Bad request: {}", message);
            }
            AppError::Internal(ref message) => {
                tracing::error!("Internal server error: {}", message);
            }
        }
    }

    // Get user-friendly message based on environment
    fn user_message(&self, env: &Environment) -> String {
        match env {
            Environment::Development => {
                // In development, provide detailed error messages
                match self {
                    AppError::Database(ref e) => format!("Database error: {}", e),
                    AppError::Redis(ref e) => format!("Cache error: {}", e),
                    AppError::NotFound(ref message) => message.clone(),
                    AppError::Unauthorized => "Unauthorized".to_string(),
                    AppError::BadRequest(ref message) => message.clone(),
                    AppError::Internal(ref message) => format!("Internal error: {}", message),
                }
            }
            Environment::Production => {
                // In production, provide generic messages for server errors
                match self {
                    AppError::Database(_) => {
                        "An internal error occurred. Please try again later.".to_string()
                    }
                    AppError::Redis(_) => {
                        "An internal error occurred. Please try again later.".to_string()
                    }
                    AppError::NotFound(ref message) => message.clone(),
                    AppError::Unauthorized => "Unauthorized".to_string(),
                    AppError::BadRequest(ref message) => message.clone(),
                    AppError::Internal(_) => {
                        "An internal error occurred. Please try again later.".to_string()
                    }
                }
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Always log the error
        self.log_error();

        // Determine status code
        let status = match self {
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Redis(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        // For simplicity, assume development environment in this context
        // In a real implementation, you would access the config from application state
        let env = Environment::Development;
        let message = self.user_message(&env);

        let body = Json(json!({
            "error": message
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

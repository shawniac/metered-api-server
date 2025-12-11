use std::convert::Infallible;
use warp::http::StatusCode;
use warp::reply::{json, with_status, Json, WithStatus};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Warp error: {0}")]
    Warp(#[from] warp::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}

impl warp::reject::Reject for AppError {}

pub type Result<T> = std::result::Result<T, AppError>;

impl warp::Reply for AppError {
    fn into_response(self) -> warp::reply::Response {
        let (status, message) = match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::Warp(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Server error"),
            AppError::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, "IO error"),
            AppError::Config(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error"),
            AppError::RateLimitExceeded => (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded"),
        };
        with_status(json(&serde_json::json!({ "error": message })), status).into_response()
    }
}
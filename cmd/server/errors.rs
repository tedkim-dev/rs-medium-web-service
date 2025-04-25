use core::fmt;

use axum::{response::{IntoResponse, Response}, Json};
use http::StatusCode;
use serde_json::json;

pub enum ApiError {
    Service(anyhow::Error),
    // Unexpected(anyhow::Error),
    NotFound(anyhow::Error),
    BadRequest(anyhow::Error),
    // Validation(anyhow::Error),
    // Authorization(anyhow::Error),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Service(e) => write!(f, "Service error: {}", e),
            // ApiError::Unexpected(e) => write!(f, "Unexpected error: {}", e),
            ApiError::NotFound(e) => write!(f, "Not found error: {}", e),
            ApiError::BadRequest(e) => write!(f, "Bad request error: {}", e),
            // ApiError::Validation(e) => write!(f, "Validation error: {}", e),
            // ApiError::Authorization(e) => write!(f, "Authorization error: {}", e),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::Service(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{}", e),
            ),
            ApiError::BadRequest(e) => (
                StatusCode::BAD_REQUEST,
                format!("{}", e),
            ),
            // ApiError::Unexpected(e) => (
            //     StatusCode::INTERNAL_SERVER_ERROR,
            //     format!("Unexpected error: {}", e),
            // ),
            ApiError::NotFound(e) => (
                StatusCode::NOT_FOUND,
                format!("{}", e),
            ),
            // ApiError::Validation(e) => (
            //     StatusCode::BAD_REQUEST,
            //     format!("Validation error: {}", e),
            // ),
            // ApiError::Authorization(e) => (
            //     StatusCode::UNAUTHORIZED,
            //     format!("Authorization error: {}", e),
            // ),
        };

        let body = Json(json!({
            "error": {
                "message": error_message,
                "code": status.as_u16(),
            }
        }));

        (status, body).into_response()
    }
}
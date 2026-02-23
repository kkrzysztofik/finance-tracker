use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    Csv(String),
    NotFound(String),
    BadRequest(String),
    Unauthorized,
    Internal(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(e) => write!(f, "Database error: {e}"),
            Self::Csv(e) => write!(f, "CSV error: {e}"),
            Self::NotFound(e) => write!(f, "Not found: {e}"),
            Self::BadRequest(e) => write!(f, "Bad request: {e}"),
            Self::Unauthorized => write!(f, "Unauthorized"),
            Self::Internal(e) => write!(f, "Internal error: {e}"),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            Self::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::Csv(e) => (StatusCode::BAD_REQUEST, e.clone()),
            Self::NotFound(e) => (StatusCode::NOT_FOUND, e.clone()),
            Self::BadRequest(e) => (StatusCode::BAD_REQUEST, e.clone()),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".into()),
            Self::Internal(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.clone()),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        Self::Database(e)
    }
}

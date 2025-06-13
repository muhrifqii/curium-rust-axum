use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::error;

pub type AppResult<T, E = AppError> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    General(String),
    #[error("Not found")]
    NotFound,
}

impl AppError {
    pub fn map<E: std::error::Error + Send + Sync + 'static>(err: E) -> Self {
        Self::General(err.to_string())
    }

    pub fn string(text: &str) -> Self {
        Self::General(text.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "".to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Error".to_string()),
        };
        (status, Json(ErrorResponse { message })).into_response()
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

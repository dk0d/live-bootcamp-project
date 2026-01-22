use axum::response::IntoResponse;
use axum::Json;
use reqwest::StatusCode;
use utoipa::ToSchema;

use crate::errors::ErrorResponse;

#[derive(Debug, thiserror::Error, Clone, ToSchema)]
pub enum UserStoreError {
    #[error("User already exists")]
    UserAlreadyExists,

    #[error("User not found")]
    UserNotFound,

    #[error("Invalid credentials provided")]
    InvalidCredentials,

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

impl UserStoreError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            UserStoreError::UserAlreadyExists => StatusCode::CONFLICT,
            UserStoreError::UserNotFound => StatusCode::NOT_FOUND,
            UserStoreError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            UserStoreError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for UserStoreError {
    fn into_response(self) -> axum::response::Response {
        let code = self.status_code();
        let body = Json(ErrorResponse {
            error: self.to_string(),
        });
        (code, body).into_response()
    }
}

use crate::domain::{Email, Password, User};
use crate::errors::ErrorResponse;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use utoipa::ToSchema;

#[async_trait::async_trait]
pub trait UserStore: Send + Sync + std::fmt::Debug {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_credentials(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<bool, UserStoreError>;
}

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

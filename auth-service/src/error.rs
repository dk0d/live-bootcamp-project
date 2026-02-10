use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub trait StatusCoded {
    fn status_code(&self) -> axum::http::StatusCode;
}

/// Top level project error
#[derive(Debug, ThisError)]
pub enum AuthApiError {
    /// Malformed request data
    #[error("Malformed request data")]
    MalformedRequest,

    /// Error from user store
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Invalid format: {0}")]
    InvalidData(String),

    #[error("Password too short: {0}")]
    PasswordTooShort(usize),

    /// Error for invalid email format
    #[error("Invalid email: {0}")]
    InvalidEmail(String),

    /// Error for invalid email format
    #[error("Email send error: {0}")]
    EmailSendError(String),

    #[error("Missing field: {0}")]
    MissingField(String),

    /// User already exists
    #[error("User already exists")]
    UserAlreadyExists,

    /// User not found
    #[error("User not found")]
    UserNotFound,

    /// Two factor code not found
    #[error("Two factor code not found")]
    TwoFactorCodeNotFound,

    #[error("Two factor code mismatch")]
    TwoFactorCodeMismatch,

    #[error("Failed to generate two factor code")]
    TwoFactorCodeGenFailed,

    #[error("Invalid login attempt id")]
    InvalidLoginAttemptId,

    /// Invalid credentials provided
    #[error("Invalid credentials provided")]
    InvalidCredentials,

    /// Invalid JWT token
    #[error("Invalid jwt token")]
    InvalidToken,

    /// Missing token in request
    #[error("Missing token in request")]
    MissingToken,

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

impl Serialize for AuthApiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let error_response = ErrorResponse {
            error: self.to_string(),
        };
        error_response.serialize(serializer)
    }
}

impl StatusCoded for AuthApiError {
    fn status_code(&self) -> axum::http::StatusCode {
        match self {
            AuthApiError::InvalidCredentials => StatusCode::BAD_REQUEST,
            AuthApiError::PasswordTooShort(_) => StatusCode::BAD_REQUEST,
            AuthApiError::InvalidEmail(_) => StatusCode::BAD_REQUEST,
            AuthApiError::MissingToken => StatusCode::BAD_REQUEST,
            AuthApiError::MalformedRequest => StatusCode::UNPROCESSABLE_ENTITY,
            AuthApiError::InvalidData(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AuthApiError::MissingField(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AuthApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            AuthApiError::UserAlreadyExists => StatusCode::CONFLICT,
            AuthApiError::UserNotFound => StatusCode::NOT_FOUND,
            AuthApiError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AuthApiError::InvalidToken => StatusCode::UNAUTHORIZED,
            AuthApiError::TwoFactorCodeNotFound => StatusCode::NOT_FOUND,
            AuthApiError::TwoFactorCodeMismatch => StatusCode::UNAUTHORIZED,
            AuthApiError::TwoFactorCodeGenFailed => StatusCode::INTERNAL_SERVER_ERROR,
            AuthApiError::InvalidLoginAttemptId => StatusCode::BAD_REQUEST,
            AuthApiError::EmailSendError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AuthApiError {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code();
        let body = serde_json::to_string(&self).unwrap_or_else(|_| {
            serde_json::to_string(&ErrorResponse {
                error: "Internal Server Error".to_string(),
            })
            .unwrap()
        });
        (status_code, body).into_response()
    }
}

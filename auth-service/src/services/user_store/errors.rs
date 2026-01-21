use axum::response::IntoResponse;

#[derive(Debug, thiserror::Error, Clone)]
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

impl IntoResponse for UserStoreError {
    fn into_response(self) -> axum::response::Response {
        match self {
            UserStoreError::UserAlreadyExists => {
                (axum::http::StatusCode::CONFLICT, self.to_string()).into_response()
            }
            UserStoreError::UserNotFound => {
                (axum::http::StatusCode::NOT_FOUND, self.to_string()).into_response()
            }
            UserStoreError::InvalidCredentials => {
                (axum::http::StatusCode::UNAUTHORIZED, self.to_string()).into_response()
            }
            UserStoreError::UnexpectedError(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                self.to_string(),
            )
                .into_response(),
        }
    }
}

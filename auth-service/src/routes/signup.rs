use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use tracing::instrument;
use utoipa::ToSchema;

use crate::domain::user::User;
use crate::errors::ErrorResponse;
use crate::services::user_store::{UserStore, UserStoreError};
use crate::state::AppState;
use crate::utils::crypto::hash_password;

fn default_false() -> bool {
    false
}

#[derive(serde::Deserialize, Serialize, Debug, ToSchema)]
#[serde(tag = "method", rename_all = "snake_case")]
pub enum SignupRequest {
    /// Signup using email and password
    #[schema(title = "Email/Password")]
    EmailPassword {
        email: String,
        password: String,
        #[serde(default = "default_false")]
        requires_2fa: bool,
    },

    /// Signup using magic link sent to email
    ///
    /// Coming soon...
    #[schema(title = "Magic Link")]
    MagicLink { email: String },

    /// Signup using passkey (WebAuthn)
    ///
    /// Coming soon...
    #[schema(title = "Passkey/WebAuthn")]
    Passkey { email: String },
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum SignupError {
    #[error("Unsupported signup method")]
    UnsupportedMethod,

    #[error("Invalid signup request")]
    InvalidRequest,

    #[error(transparent)]
    UserStoreError(#[from] UserStoreError),

    /// Error for invalid email format
    #[error("Invalid email: {0}")]
    InvalidEmail(String),

    #[error("Invalid password: {0}")]
    InvalidPassword(String),
}

impl IntoResponse for SignupError {
    fn into_response(self) -> axum::response::Response {
        let code = match self {
            SignupError::UnsupportedMethod => StatusCode::BAD_REQUEST,
            SignupError::InvalidRequest => StatusCode::BAD_REQUEST,
            SignupError::UserStoreError(ref e) => e.status_code(),
            SignupError::InvalidEmail(_) => StatusCode::BAD_REQUEST,
            SignupError::InvalidPassword(_) => StatusCode::BAD_REQUEST,
        };
        let body = Json(ErrorResponse {
            error: self.to_string(),
        });
        (code, body).into_response()
    }
}

struct SignupValidator;
impl SignupValidator {
    fn validate_password(password: &str) -> Result<(), SignupError> {
        // Simple password strength check (at least 8 characters)
        if password.len() < 8 {
            return Err(SignupError::InvalidPassword(
                "Password must be at least 8 characters long".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_email(email: &str) -> Result<(), SignupError> {
        // Simple email format check
        if !email.contains('@') || !email.contains('.') {
            return Err(SignupError::InvalidEmail(
                "Email format is invalid".to_string(),
            ));
        }

        Ok(())
    }
}

impl TryFrom<SignupRequest> for User {
    type Error = SignupError;

    fn try_from(req: SignupRequest) -> Result<Self, SignupError> {
        match req {
            SignupRequest::EmailPassword {
                email,
                password,
                requires_2fa,
            } => {
                SignupValidator::validate_password(&password)?;
                SignupValidator::validate_email(&email)?;
                // Here you would hash the password before storing it
                let hashed_password = hash_password(&password)
                    .map_err(|e| SignupError::InvalidPassword(e.to_string()))?;
                Ok(Self::new(email, hashed_password, requires_2fa))
            }
            _ => Err(SignupError::UnsupportedMethod),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct SignupResponse {
    pub message: String,
}

#[utoipa::path(
    post,
    path = "/signup",
    tag = "Authentication",
    responses(
        (status = 201, description = "Signup successful"),
        (status = 400, description = "Bad Request")
    )
)]
#[instrument]
pub async fn signup_handler(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, SignupError> {
    // Placeholder for signup logic
    let user: User = request.try_into()?;
    let mut user_store = state.user_store.write().await;
    user_store.add_user(user)?;
    Ok((
        StatusCode::CREATED,
        Json(SignupResponse {
            message: "Signup successful".to_string(),
        }),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signup_request_schema() {
        let req = SignupRequest::EmailPassword {
            email: "testuser@hello.com".to_string(),
            password: "password123".to_string(),
            requires_2fa: false,
        };
        let schema = serde_json::to_string_pretty(&req).unwrap();
        println!("SignupRequest Schema: {}", schema);
        assert!(schema.contains("method"));
    }
}

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use tracing::instrument;
use utoipa::ToSchema;

use crate::domain::user::User;
use crate::services::user_store::UserStore;
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

    #[error("Invalid email or password")]
    InvalidEmailPasswordRequest,

    #[error(transparent)]
    UserStoreError(#[from] crate::services::user_store::errors::UserStoreError),
}

impl IntoResponse for SignupError {
    fn into_response(self) -> axum::response::Response {
        match self {
            SignupError::UnsupportedMethod => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            SignupError::InvalidEmailPasswordRequest => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            SignupError::UserStoreError(e) => e.into_response(),
        }
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
                // Here you would hash the password before storing it
                let hashed_password = hash_password(&password)
                    .map_err(|_| SignupError::InvalidEmailPasswordRequest)?;
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

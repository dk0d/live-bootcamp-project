use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;
use tracing::instrument;
use utoipa::ToSchema;

use crate::domain::{Email, Password, TwoFactorMethod, User, UserStore};
use crate::error::AuthApiError;
use crate::state::AppState;
use crate::utils::FormOrJson;

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
        #[serde(default = "TwoFactorMethod::default")]
        two_factor: TwoFactorMethod,
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

impl TryFrom<SignupRequest> for User {
    type Error = AuthApiError;

    fn try_from(req: SignupRequest) -> Result<Self, AuthApiError> {
        match req {
            SignupRequest::EmailPassword {
                email,
                password,
                two_factor,
            } => {
                let email: Email = email.try_into()?;
                let hashed_password: Password = password.try_into()?;
                Ok(Self::new(email, hashed_password, two_factor))
            }
            _ => Err(AuthApiError::MalformedRequest),
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
    FormOrJson(request): FormOrJson<SignupRequest>,
) -> Result<impl IntoResponse, AuthApiError> {
    // Placeholder for signup logic
    let user: User = request.try_into()?;
    let mut user_store = state.user_store.write().await;
    user_store.add_user(user).await?;
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
            two_factor: TwoFactorMethod::None,
        };
        let schema = serde_json::to_string_pretty(&req).unwrap();
        println!("SignupRequest Schema: {}", schema);
        assert!(schema.contains("method"));
    }
}

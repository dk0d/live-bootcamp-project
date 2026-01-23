use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use tracing::instrument;
use utoipa::ToSchema;

use crate::domain::{Email, Password, UserStore};
use crate::error::AuthApiError;
use crate::state::AppState;

#[derive(serde::Deserialize, Serialize, Debug, ToSchema)]
#[serde(tag = "method", rename_all = "snake_case")]
pub enum LoginRequest {
    /// Signup using email and password
    #[schema(title = "Email/Password")]
    EmailPassword { email: String, password: String },

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

#[utoipa::path(
    post,
    path = "/login",
    tag = "Authentication",
    responses(
        (status = 200, description = "Login successful"),
        (status = 401, description = "Unauthorized"),
        (status = 422, description = "Unprocessable Entity")
    )
)]
#[instrument]
pub async fn login_handler(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthApiError> {
    match &body {
        LoginRequest::EmailPassword { email, password } => {
            let email = Email::parse(email)?;
            let password = Password::parse(password)?;
            let user_store = &state.user_store.read().await;
            _ = user_store
                .get_user(&email)
                .await
                .map_err(|_| AuthApiError::Unauthorized)?;

            user_store
                .validate_credentials(&email, &password)
                .await
                .map_err(|_| AuthApiError::Unauthorized)?;

            Ok((StatusCode::OK, "Login successful"))
        }
        LoginRequest::MagicLink { .. } => Err(AuthApiError::MalformedRequest),
        LoginRequest::Passkey { .. } => Err(AuthApiError::MalformedRequest),
    }

    // Placeholder for login logic
}

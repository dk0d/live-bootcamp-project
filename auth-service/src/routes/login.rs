use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use serde::Serialize;
use tracing::instrument;
use utoipa::ToSchema;

use crate::domain::{Email, Password, UserStore};
use crate::error::{AuthApiError, StatusCoded};
use crate::state::AppState;

use crate::utils::FormOrJson;
use crate::utils::auth::generate_auth_cookie;

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

async fn login(state: &AppState, body: &LoginRequest) -> Result<Cookie<'static>, AuthApiError> {
    let validated_user_email = match &body {
        LoginRequest::EmailPassword { email, password } => {
            let email = Email::parse(email)?;
            let password = Password::parse(password)?;
            let user_store = &state.user_store.read().await;
            user_store
                .validate_credentials(&email, &password)
                .await
                .map_err(|_| AuthApiError::Unauthorized)?;
            Ok(email)
        }
        // magic link / OTP requires a different flow so will need to think about what thes
        // login endpoint return types should should like
        LoginRequest::MagicLink { .. } => Err(AuthApiError::MalformedRequest),

        // passkeys.rs likely - use WebAuthn flows
        LoginRequest::Passkey { .. } => Err(AuthApiError::MalformedRequest),
    }?;
    generate_auth_cookie(&validated_user_email, &state.config.jwt)
        .map_err(|_| AuthApiError::Unauthorized)
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
    jar: CookieJar, // must come before the body extractor
    State(state): State<AppState>,
    FormOrJson(body): FormOrJson<LoginRequest>, // must be last
) -> (CookieJar, impl IntoResponse) {
    match login(&state, &body).await {
        Ok(token_cookie) => {
            let jar = jar.add(token_cookie.clone());
            (
                jar,
                (
                    StatusCode::OK,
                    Json(serde_json::json!({"token": token_cookie.value()})),
                ),
            )
        }
        Err(ref error) => (
            jar,
            (
                error.status_code(),
                Json(serde_json::json!({"error": error.to_string()})),
            ),
        ),
    }
}

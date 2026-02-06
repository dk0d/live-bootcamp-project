use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;

use serde::Serialize;
use tracing::instrument;
use utoipa::ToSchema;

use crate::domain::{
    Email, LoginAttemptId, Password, TwoFactorCode, TwoFactorCodeStore, TwoFactorMethod, UserStore,
};
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

#[derive(Serialize, ToSchema, serde::Deserialize, Debug)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum LoginResponse {
    #[schema(title = "Auth Token")]
    Success { email: Email, token: String },

    /// Two Factor assumes user has validated primary credentials
    /// i.e. email/password
    #[schema(title = "Two Factor Authentication Required")]
    TwoFactor {
        method: TwoFactorMethod,
        message: String,
        login_attempt_id: LoginAttemptId,
        code: TwoFactorCode,
    },
}

impl LoginResponse {
    pub fn status_code(&self) -> StatusCode {
        match self {
            LoginResponse::Success { .. } => StatusCode::OK,
            LoginResponse::TwoFactor { .. } => StatusCode::PARTIAL_CONTENT, // 206
        }
    }
}

fn handle_2fa(
    jar: CookieJar,
    attempt_id: &LoginAttemptId,
    code: &TwoFactorCode,
) -> (CookieJar, (StatusCode, Json<serde_json::Value>)) {
    // For 2FA, we don't set the auth cookie yet.
    // Instead, we prompt the user to complete the 2FA step.
    (
        jar,
        (
            StatusCode::PARTIAL_CONTENT, // 206
            Json(serde_json::json!(LoginResponse::TwoFactor {
                method: TwoFactorMethod::Email,
                message: "Two-factor authentication required. Please complete the 2FA step."
                    .to_string(),
                login_attempt_id: attempt_id.clone(),
                code: code.clone(),
            })),
        ),
    )
}

fn handle_successful_login(
    jar: CookieJar,
    email: &Email,
    state: &AppState,
) -> (CookieJar, (StatusCode, Json<serde_json::Value>)) {
    let token = generate_auth_cookie(email, &state.config.jwt);

    if token.is_err() {
        return (
            jar,
            (
                AuthApiError::InvalidCredentials.status_code(),
                Json(serde_json::json!({ "error": AuthApiError::InvalidCredentials.to_string()})),
            ),
        );
    }

    let token = token.unwrap();
    let jar = jar.add(token.clone());

    (
        jar,
        (
            StatusCode::OK,
            Json(serde_json::json!(LoginResponse::Success {
                email: email.clone(),
                token: token.value().to_string(),
            })),
        ),
    )
}

async fn login(state: &AppState, body: &LoginRequest) -> Result<LoginResponse, AuthApiError> {
    match &body {
        LoginRequest::EmailPassword { email, password } => {
            let email = Email::parse(email)?;
            let password = Password::parse(password)?;
            let user_store = &state.user_store.read().await;
            let user = user_store
                .get_user(&email)
                .await
                .map_err(|_| AuthApiError::Unauthorized)?;

            user_store
                .validate_credentials(&email, &password)
                .await
                .map_err(|_| AuthApiError::Unauthorized)?;

            if let TwoFactorMethod::Email = user.two_factor {
                let mut codes = state.two_factor.write().await;
                let (login_attempt_id, code) = codes.new_login_attempt(&email, &user.two_factor)?;
                Ok(LoginResponse::TwoFactor {
                    method: user.two_factor,
                    message: "Check your email".to_string(),
                    login_attempt_id,
                    code,
                })
            } else {
                Ok(LoginResponse::Success {
                    email,
                    token: "".to_string(),
                }) // token will be generated in handler
            }
        }
        // magic link / OTP requires a different flow so will need to think about what thes
        // login endpoint return types should should like
        LoginRequest::MagicLink { .. } => Err(AuthApiError::MalformedRequest),

        // passkeys.rs likely - use WebAuthn flows
        LoginRequest::Passkey { .. } => Err(AuthApiError::MalformedRequest),
    }
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
    let result = login(&state, &body).await;

    match result {
        Ok(LoginResponse::Success { email, .. }) => handle_successful_login(jar, &email, &state),
        Ok(LoginResponse::TwoFactor {
            login_attempt_id,
            code,
            ..
        }) => handle_2fa(jar, &login_attempt_id, &code),
        Err(ref error) => (
            jar,
            (
                error.status_code(),
                Json(serde_json::json!({"error": error.to_string()})),
            ),
        ),
    }
}

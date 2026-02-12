use crate::domain::{Email, LoginAttemptId, TwoFactorCode, TwoFactorCodeStore, TwoFactorMethod};
use crate::error::AuthApiError;
use crate::routes::LoginResponse;
use crate::state::AppState;
use crate::utils::FormOrJson;
use crate::utils::auth::generate_auth_cookie;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use serde::Serialize;
use tracing::instrument;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, serde::Deserialize, Debug)]
pub struct Verify2FARequest {
    method: TwoFactorMethod,
    email: String,
    id: String,
    code: String,
}

// Consumes the request to verify the 2FA code. If successful, returns the email associated with the login attempt.
pub async fn verify_2fa(state: &AppState, body: Verify2FARequest) -> Result<Email, AuthApiError> {
    let two_factor = &state.two_factor.read().await;
    let email: Email = body
        .email
        .try_into()
        .map_err(|_| AuthApiError::Unauthorized)?;
    let attempt_id: LoginAttemptId = body.id.try_into().map_err(|_| AuthApiError::Unauthorized)?;
    let code: TwoFactorCode = body
        .code
        .try_into()
        .map_err(|_| AuthApiError::Unauthorized)?;
    if !two_factor
        .verify_code(&email, &attempt_id, &code)
        .unwrap_or(false)
    {
        return Err(AuthApiError::Unauthorized);
    }
    Ok(email)
}

#[utoipa::path(
    post,
    path = "/verify-2fa",
    tag = "Authentication",
    responses(
        (status = 200, description = "2FA verification successful"),
        (status = 401, description = "Unauthorized")
    )
)]
#[instrument(skip(jar, state, body))]
pub async fn verify_2fa_handler(
    jar: CookieJar,
    State(state): State<AppState>,
    FormOrJson(body): FormOrJson<Verify2FARequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthApiError>) {
    let result = verify_2fa(&state, body).await;
    if result.is_err() {
        return (jar, Err(result.err().unwrap()));
    }
    let email = result.unwrap();
    let token = generate_auth_cookie(&email, &state.config.jwt);
    if token.is_err() {
        return (jar, Err(AuthApiError::InvalidCredentials));
    }
    let token = token.unwrap();
    let jar = jar.add(token.clone());
    (
        jar,
        Ok((
            StatusCode::OK,
            Json(LoginResponse::Success {
                email: email.clone(),
                token: token.value().to_string(),
            }),
        )),
    )
}

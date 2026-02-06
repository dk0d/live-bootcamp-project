use crate::domain::{Email, LoginAttemptId, TwoFactorCode, TwoFactorCodeStore, TwoFactorMethod};
use crate::error::AuthApiError;
use crate::state::AppState;
use crate::utils::FormOrJson;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;
use tracing::instrument;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, serde::Deserialize, Debug)]
pub struct Verify2FARequest {
    method: TwoFactorMethod,
    email: String,
    login_attempt_id: String,
    code: String,
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
#[instrument]
pub async fn verify_2fa_handler(
    State(state): State<AppState>,
    FormOrJson(body): FormOrJson<Verify2FARequest>,
) -> Result<impl IntoResponse, AuthApiError> {
    let two_factor = &state.two_factor.read().await;
    let email: Email = body
        .email
        .try_into()
        .map_err(|_| AuthApiError::Unauthorized)?;
    let attempt_id: LoginAttemptId = body
        .login_attempt_id
        .try_into()
        .map_err(|_| AuthApiError::Unauthorized)?;
    let code: TwoFactorCode = body
        .code
        .try_into()
        .map_err(|_| AuthApiError::Unauthorized)?;

    if !two_factor
        .verify_code(&email, &attempt_id, &code)
        .map_err(|_| AuthApiError::Unauthorized)?
    {
        return Err(AuthApiError::Unauthorized);
    }

    Ok((StatusCode::OK, "2FA verification successful").into_response())
}

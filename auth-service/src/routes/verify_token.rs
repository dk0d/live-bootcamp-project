use crate::domain::BannedTokenStore;
use crate::error::AuthApiError;
use crate::state::AppState;
use crate::utils::auth::{Claims, validate_token};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use tracing::instrument;
use utoipa::ToSchema;

#[derive(serde::Deserialize, ToSchema, Debug, Clone)]
pub struct VerifyTokenRequest {
    pub token: String,
}

#[utoipa::path(
    post,
    path = "/verify-token",
    tag = "Authentication",
    responses(
        (status = 200, description = "Token verification successful"),
        (status = 401, description = "Unauthorized")
    )
)]
#[instrument]
pub async fn verify_token_handler(
    State(state): State<AppState>,
    Json(body): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthApiError> {
    let banned = state.banned_tokens.read().await;

    if banned.is_token_banned(&body.token) {
        return Err(AuthApiError::Unauthorized);
    }

    _ = validate_token::<Claims>(&body.token, &state.config.jwt)
        .await
        .map_err(|_| AuthApiError::InvalidToken)?;

    Ok((StatusCode::OK, "Token verification successful").into_response())
}

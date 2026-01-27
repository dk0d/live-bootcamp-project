use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use tracing::instrument;
use utoipa::ToSchema;

use crate::error::AuthApiError;
use crate::state::AppState;
use crate::utils::auth::{validate_token, Claims};

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
    // Placeholder for token verification logic

    _ = validate_token::<Claims>(&body.token, &state.config.jwt.secret)
        .await
        .map_err(|_| AuthApiError::InvalidToken)?;

    Ok((StatusCode::OK, "Token verification successful").into_response())
}

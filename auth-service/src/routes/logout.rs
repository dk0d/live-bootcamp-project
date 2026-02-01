use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use tracing::instrument;

use crate::domain::BannedTokenStore;
use crate::error::{AuthApiError, StatusCoded};
use crate::state::AppState;
use crate::utils::auth::{Claims, validate_token};

async fn logout(state: &AppState, jar: CookieJar) -> Result<CookieJar, AuthApiError> {
    let cookie = jar
        .get(&state.config.jwt.cookie_name)
        .ok_or(AuthApiError::MissingToken)?;
    let token = cookie.value().to_string();
    _ = validate_token::<Claims>(&token, &state.config.jwt.secret)
        .await
        .map_err(|_| AuthApiError::InvalidToken)?;
    let mut banned = state.banned_tokens.write().await;
    banned.ban_token(token)?;
    Ok(jar.remove(Cookie::from(state.config.jwt.cookie_name.clone())))
}

#[utoipa::path(
    post,
    path = "/logout",
    tag = "Authentication",
    responses(
        (status = 200, description = "Logout successful"),
        (status = 401, description = "Unauthorized")
    )
)]
#[instrument]
pub async fn logout_handler(
    State(state): State<AppState>,
    jar: CookieJar,
) -> (CookieJar, impl IntoResponse) {
    match logout(&state, jar.clone()).await {
        Ok(jar) => (jar, (StatusCode::OK, "Logout successful".to_string())),
        Err(ref error) => (jar, (error.status_code(), error.to_string())),
    }
}

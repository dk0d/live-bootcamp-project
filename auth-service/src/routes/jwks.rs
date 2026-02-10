use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::instrument;

use crate::state::AppState;
use crate::utils::auth::get_jwks;

#[utoipa::path(get, path = "/.well-known/jwks.json", tag = "JWKS", 
    responses(
        (status = 200, description = "JWKS endpoint", body = String)
    )
)]
#[instrument(skip(state))]
pub async fn jwks_handler(State(state): State<AppState>) -> impl IntoResponse {
    let keys = get_jwks(&state).await;
    (StatusCode::OK, Json(keys))
}

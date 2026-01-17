use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::instrument;

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
pub async fn logout_handler() -> impl IntoResponse {
    // Placeholder for logout logic
    (StatusCode::OK, "Logout successful").into_response()
}

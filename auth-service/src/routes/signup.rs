use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::instrument;

#[utoipa::path(
    post,
    path = "/signup",
    tag = "Authentication",
    responses(
        (status = 200, description = "Signup successful"),
        (status = 400, description = "Bad Request")
    )
)]
#[instrument]
pub async fn signup_handler() -> impl IntoResponse {
    // Placeholder for signup logic
    (StatusCode::OK, "Signup successful").into_response()
}

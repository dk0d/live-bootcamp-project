use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};
use tracing::instrument;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use axum::response::Html;

use crate::openapi::ApiDoc;

/// Health check endpoint
#[utoipa::path(get, path = "/healthz", tag = "Health")]
#[instrument]
pub async fn healthz() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "Healthy"
        })),
    )
        .into_response()
}

/// Liveness check endpoint
#[utoipa::path(get, path = "/livez", tag = "Health")]
#[instrument]
pub async fn livez() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "Alive"
        })),
    )
        .into_response()
}

/// Ready check endpoint
#[utoipa::path(get, path = "/readyz", tag = "Health")]
#[instrument]
pub async fn readyz() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "Ready"
        })),
    )
        .into_response()
}

/// Home
#[utoipa::path(get, path = "/")]
#[instrument]
pub async fn root() -> impl IntoResponse {
    Html("<h1>AuthX.rs</h1>")
}

/// Hello World
#[utoipa::path(get, path = "/hello/{name}", tag = "Greeting")]
#[instrument]
async fn hello_handler(Path(name): Path<String>) -> impl IntoResponse {
    let name = if name.is_empty() { "World" } else { &name };
    Html(format!(
        "<h1>Hello {}!</h1><div>Welcome to AuthX.rs</div>",
        name
    ))
}

#[utoipa::path(
    post,
    path = "/login",
    tag = "Authentication",
    responses(
        (status = 200, description = "Login successful"),
        (status = 401, description = "Unauthorized")
    )
)]
#[instrument]
async fn login_handler() -> impl IntoResponse {
    // Placeholder for login logic
    (StatusCode::OK, "Login successful").into_response()
}

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
async fn signup_handler() -> impl IntoResponse {
    // Placeholder for signup logic
    (StatusCode::OK, "Signup successful").into_response()
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
async fn logout_handler() -> impl IntoResponse {
    // Placeholder for logout logic
    (StatusCode::OK, "Logout successful").into_response()
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
async fn verify_2fa_handler() -> impl IntoResponse {
    // Placeholder for 2FA verification logic
    (StatusCode::OK, "2FA verification successful").into_response()
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
async fn verify_token_handler() -> impl IntoResponse {
    // Placeholder for token verification logic
    (StatusCode::OK, "Token verification successful").into_response()
}

pub fn build_app_router() -> OpenApiRouter {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(root))
        .routes(routes!(hello_handler))
        .routes(routes!(healthz))
        .routes(routes!(livez))
        .routes(routes!(login_handler))
        .routes(routes!(signup_handler))
        .routes(routes!(logout_handler))
        .routes(routes!(verify_2fa_handler))
        .routes(routes!(verify_token_handler))
        .routes(routes!(readyz))
}

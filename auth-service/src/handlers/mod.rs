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
    (StatusCode::OK, "AuthX.rs").into_response()
}

/// Home
#[utoipa::path(get, path = "/hello")]
#[instrument]
async fn hello_handler() -> Html<&'static str> {
    // TODO: Update this to a custom message!
    Html("<h1>Hello, World!</h1>")
}

pub fn build_app_router() -> OpenApiRouter {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(root))
        .routes(routes!(hello_handler))
        .routes(routes!(healthz))
        .routes(routes!(livez))
        .routes(routes!(readyz))
}

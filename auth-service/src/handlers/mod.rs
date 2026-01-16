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
    (StatusCode::OK, "AuthX.rs").into_response()
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

pub fn build_app_router() -> OpenApiRouter {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(root))
        .routes(routes!(hello_handler))
        .routes(routes!(healthz))
        .routes(routes!(livez))
        .routes(routes!(readyz))
}

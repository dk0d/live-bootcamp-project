pub mod config;
pub mod handlers;
pub mod logging;
pub mod openapi;

use std::time::Duration;

use axum::Router;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::Level;

use utoipa_scalar::{Scalar, Servable};

use self::handlers::build_app_router;

pub async fn build_router(config: &config::Config) -> anyhow::Result<Router> {
    let assets_dir = ServeDir::new("assets");

    let (router, api) = build_app_router().split_for_parts();

    let router = router.merge(Scalar::with_url("/docs", api)).layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &axum::http::Request<_>| {
                tracing::span!(
                    Level::INFO,
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri().path(),
                        status = tracing::field::Empty, // Status filled later
                        latency_us = tracing::field::Empty // Latency filled later
                )
            })
            .on_response(
                |resp: &axum::http::Response<_>, latency: Duration, span: &tracing::Span| {
                    span.record("status", resp.status().as_u16());
                    span.record("latency_us", latency.as_micros());
                },
            ),
    );

    Ok(router)
}

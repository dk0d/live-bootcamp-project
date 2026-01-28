pub mod config;
pub mod domain;
pub mod error;
pub mod logging;
pub mod openapi;
pub mod routes;
pub mod services;
pub mod state;
pub mod utils;

use std::sync::Arc;
use std::time::Duration;

use tokio::net::TcpListener;

use axum::{serve::Serve, Router};

use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::Level;

use utoipa_scalar::{Scalar, Servable};

use crate::routes::build_app_router;

use self::services::user_store::hash_map::HashMapUserUserStore;

#[derive(Debug)]
pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    pub address: String,
}

impl Application {
    /// Build the main application router with middleware and documentation.
    ///
    /// This may be useful when using axum_test to map per test routers.
    ///
    /// Arguments:
    /// - `config`: Application configuration. (though not used currently, extendable later)
    ///
    /// Returns:
    /// - `Router`: The constructed application router.
    pub async fn build_router(config: &config::Config) -> anyhow::Result<Router> {
        let assets_dir =
            ServeDir::new("assets").not_found_service(ServeFile::new("assets/404.html"));

        let user_store = Arc::new(RwLock::new(HashMapUserUserStore::new()));

        let mut allowed_origins = vec![
            format!("http://localhost:{}", config.server.port),
            format!("http://{}", config.server.address()),
        ];
        if let Some(origins) = &config.server.allowed_origins {
            allowed_origins.extend(origins.clone());
        }

        let cors = CorsLayer::new()
            .allow_methods(vec![axum::http::Method::GET, axum::http::Method::POST])
            .allow_headers(vec![
                axum::http::header::ORIGIN,
                axum::http::header::AUTHORIZATION,
                axum::http::header::CONTENT_TYPE,
                axum::http::header::COOKIE,
                axum::http::header::SET_COOKIE,
            ])
            .allow_origin(
                allowed_origins
                    .into_iter()
                    .map(|o| o.parse().unwrap())
                    .collect::<Vec<axum::http::header::HeaderValue>>(),
            )
            .allow_credentials(true);

        let state = state::AppState::new(config, user_store);
        let (router, api) = build_app_router(state).split_for_parts();
        let router = router
            .fallback_service(assets_dir)
            .merge(Scalar::with_url("/docs", api))
            .layer(cors)
            .layer(
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
                        |resp: &axum::http::Response<_>,
                         latency: Duration,
                         span: &tracing::Span| {
                            span.record("status", resp.status().as_u16());
                            span.record("latency_us", latency.as_micros());
                        },
                    ),
            );
        Ok(router)
    }

    /// Main application builder.
    ///
    /// Arguments:
    /// - `config`: Application configuration.
    ///
    /// Returns:
    /// - `Application`: The constructed application instance.
    pub async fn build(config: &config::Config) -> anyhow::Result<Self> {
        let router = Application::build_router(config).await?;
        // Here we should use ip 0.0.0.0 so the service is listening on all the configured network interfaces.
        // This is needed for Docker to work, which we will add later on.
        // See: https://stackoverflow.com/questions/39525820/docker-port-forwarding-not-working
        let address = format!("{}:{}", config.server.host, config.server.port);
        let listener = TcpListener::bind(address.clone()).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);
        Ok(Self { server, address })
    }

    /// Run the application server.
    pub async fn run(self) -> anyhow::Result<()> {
        tracing::info!("Listening on {}", self.address);
        self.server.await?;
        Ok(())
    }
}

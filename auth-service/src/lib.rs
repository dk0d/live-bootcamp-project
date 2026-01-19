pub mod config;
pub mod logging;
pub mod openapi;
pub mod routes;

use std::time::Duration;

use tokio::net::TcpListener;

use axum::{serve::Serve, Router};

use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::Level;

use utoipa_scalar::{Scalar, Servable};

use self::routes::build_app_router;

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
    pub async fn build_router(_config: &config::Config) -> anyhow::Result<Router> {
        let assets_dir =
            ServeDir::new("assets").not_found_service(ServeFile::new("assets/404.html"));
        let (router, api) = build_app_router().split_for_parts();
        let router = router
            .fallback_service(assets_dir)
            .merge(Scalar::with_url("/docs", api))
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

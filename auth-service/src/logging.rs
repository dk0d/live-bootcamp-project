//! Telemetry module for tracing and observability
//!
//! Provides structured logging with optional OpenTelemetry integration.

use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

use crate::config::{Config, ServerEnv};

/// Initialize the tracing/telemetry system
pub fn init(config: &Config) -> anyhow::Result<()> {
    // Build the env filter from RUST_LOG or default
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // Default log levels
        EnvFilter::new("info,tower_http=debug,axum::rejection=trace")
    });

    if config.telemetry.enabled && config.env == ServerEnv::Production {
        // Initialize with OpenTelemetry
        init_with_otel(config, env_filter)?;
    } else {
        // Initialize with just console logging
        init_console_only(config, env_filter);
    }

    Ok(())
}

/// Initialize telemetry with OpenTelemetry export
fn init_with_otel(config: &Config, env_filter: EnvFilter) -> anyhow::Result<()> {
    todo!();
}

/// Initialize telemetry with console logging only
fn init_console_only(config: &Config, env_filter: EnvFilter) {
    match config.env {
        ServerEnv::Development => {
            // Pretty logging for development - simpler setup without OTEL
            tracing_subscriber::fmt()
                .with_env_filter(env_filter)
                .with_span_events(FmtSpan::CLOSE)
                .with_target(true)
                .pretty()
                .init();
        }
        ServerEnv::Production => {
            // JSON logging for production
            tracing_subscriber::registry()
                .with(env_filter)
                .with(
                    fmt::layer()
                        .json()
                        .with_span_events(FmtSpan::CLOSE)
                        .with_current_span(true)
                        .with_target(true),
                )
                .init();
        }
    }
}

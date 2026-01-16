use libauth_service::logging;
use libauth_service::{build_router, config};

use figment::{
    providers::{Env, Format, Toml},
    Figment,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config: config::Config = Figment::new()
        .merge(Toml::file("default.toml"))
        .merge(Env::prefixed("LR__"))
        .extract()?;

    logging::init(&config)?;

    let app = build_router(&config).await?;

    // Here we are using ip 0.0.0.0 so the service is listening on all the configured network interfaces.
    // This is needed for Docker to work, which we will add later on.
    // See: https://stackoverflow.com/questions/39525820/docker-port-forwarding-not-working
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

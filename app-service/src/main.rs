use figment::{
    Figment,
    providers::{Env, Format, Toml},
};
use libapp::{Config, build_app_router, logging};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config: Config = Figment::new()
        .merge(Env::prefixed("LRAPP__"))
        .merge(Toml::file("default.toml"))
        .extract()?;

    logging::init(&config)?;

    let app = build_app_router();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();

    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

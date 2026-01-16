use libauth_service::config;
use libauth_service::{logging, Application};

use figment::{
    providers::{Env, Format, Toml},
    Figment,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let config: config::Config = Figment::new()
        .merge(Toml::file("default.toml"))
        .merge(Env::prefixed("LR__").global())
        .extract()?;
    dbg!(&config);
    logging::init(&config)?;
    let app = Application::build(&config).await?;
    app.run().await?;
    Ok(())
}

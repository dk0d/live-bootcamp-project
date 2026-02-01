use lgr_auth::config;
use lgr_auth::{Application, logging};

use figment::{
    Figment,
    providers::{Env, Format, Toml},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let config: config::Config = Figment::new()
        .merge(Toml::file("default.toml"))
        .merge(Env::prefixed("LR_").split("__"))
        .extract()?;
    Env::prefixed("LR_").iter().for_each(|(k, v)| {
        println!("{}: {}", k, v);
    });
    logging::init(&config)?;
    let app = Application::build(&config).await?;
    app.run().await?;
    Ok(())
}

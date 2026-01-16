// FIXME: Create a workspace and move shared config stuffs to a common crate

#[derive(serde::Deserialize, Default)]
pub struct TelemetryConfig {
    #[serde(default = "default_false")]
    pub enabled: bool,
}

fn default_false() -> bool {
    false
}

#[derive(serde::Deserialize, PartialEq, Eq, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum AppEnv {
    #[default]
    Development,
    Production,
}

#[derive(serde::Deserialize, Default)]
pub struct Config {
    #[serde(default = "default_url")]
    pub url: String,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "TelemetryConfig::default")]
    pub telemetry: TelemetryConfig,

    #[serde(default = "AppEnv::default")]
    pub env: AppEnv,
}

fn default_url() -> String {
    "http://localhost:3000".to_string()
}

fn default_port() -> u16 {
    3000
}

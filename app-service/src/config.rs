// FIXME: Create a workspace and move shared config stuffs to a common crate

#[derive(serde::Deserialize, Default, Clone)]
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

#[derive(serde::Deserialize, Clone)]
pub struct AuthConfig {
    pub url: String,
}

impl Default for AuthConfig {
    fn default() -> Self {
        AuthConfig {
            url: "http://localhost:5170".to_string(),
        }
    }
}

#[derive(serde::Deserialize, Default, Clone)]
pub struct Config {
    #[serde(default = "default_url")]
    pub url: String,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "TelemetryConfig::default")]
    pub telemetry: TelemetryConfig,

    #[serde(default = "AppEnv::default")]
    pub env: AppEnv,

    #[serde(default = "AuthConfig::default")]
    pub auth: AuthConfig,
}

fn default_url() -> String {
    "http://localhost:3000".to_string()
}

fn default_port() -> u16 {
    3000
}

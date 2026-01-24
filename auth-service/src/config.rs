#[derive(serde::Deserialize, Debug, Clone)]
pub struct TelemetryConfig {
    #[serde(default = "default_false")]
    pub enabled: bool,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        TelemetryConfig {
            enabled: default_false(),
        }
    }
}

#[derive(serde::Deserialize, PartialEq, Eq, Clone, Default, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ServerEnv {
    #[default]
    Development,
    Production,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ServerConfig {
    #[serde(default = "default_server_host")]
    pub host: String,

    #[serde(default = "default_server_port")]
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: default_server_host(),
            port: default_server_port(),
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct JwtConfig {
    pub cookie_name: String,
    pub secret: String,
}

impl Default for JwtConfig {
    fn default() -> Self {
        JwtConfig {
            cookie_name: "jwt_auth_token".to_string(),
            secret: "really-long-super-secret-key-for-signing".to_string(),
        }
    }
}

#[derive(serde::Deserialize, Default, Debug, Clone)]
pub struct Config {
    #[serde(default = "ServerConfig::default")]
    pub server: ServerConfig,

    #[serde(default = "TelemetryConfig::default")]
    pub telemetry: TelemetryConfig,

    #[serde(default = "ServerEnv::default")]
    pub env: ServerEnv,

    #[serde(default = "JwtConfig::default")]
    pub jwt: JwtConfig,
}

fn default_server_host() -> String {
    "0.0.0.0".to_string()
}

fn default_server_port() -> u16 {
    3000
}

fn default_false() -> bool {
    false
}

fn default_true() -> bool {
    true
}

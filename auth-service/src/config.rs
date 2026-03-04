use crate::services::email::EmailConfig;

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

    // Extra allowed origins for CORS
    #[serde(default = "default_allowed_origins")]
    pub allowed_origins: Option<Vec<String>>,
}

impl ServerConfig {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: default_server_host(),
            port: default_server_port(),
            allowed_origins: default_allowed_origins(),
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct AppConfig {
    #[serde(default = "default_app_url")]
    pub url: String,

    #[serde(default = "default_auth_redirect_url")]
    pub two_factor_redirect_url: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            url: default_app_url(),
            two_factor_redirect_url: default_auth_redirect_url(),
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum JwtKeySecret {
    Raw { value: String },
    ECDSA { pub_key: String, priv_key: String },
    RSA { pub_key: String, priv_key: String },
}

impl Default for JwtKeySecret {
    fn default() -> Self {
        Self::Raw {
            value: "CHANGE_ME_TO_SOMETHING_GOOD".to_string(),
        }
    }
}

impl JwtKeySecret {
    pub fn alg(&self) -> jsonwebtoken::Algorithm {
        match self {
            JwtKeySecret::Raw { .. } => jsonwebtoken::Algorithm::HS256,
            JwtKeySecret::ECDSA { .. } => jsonwebtoken::Algorithm::ES256,
            JwtKeySecret::RSA { .. } => jsonwebtoken::Algorithm::RS256,
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct JwtConfig {
    pub cookie_name: String,
    pub secret: JwtKeySecret,
}

impl Default for JwtConfig {
    fn default() -> Self {
        JwtConfig {
            cookie_name: "jwt_auth_token".to_string(),
            secret: JwtKeySecret::RSA {
                pub_key: "tests/jwt-test-rsa.pub".to_string(),
                priv_key: "tests/jwt-test-rsa.pem".to_string(),
            }, // secret: JwtKeySecret::Raw {
               //     value: "really-long-super-secret-key-for-signing".to_string(),
               // },
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct DbConfig {
    pub min_connections: u32,
    pub max_connections: u32,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            min_connections: 5,
            max_connections: 10,
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct RedisConfig {
    #[serde(default = "default_redis_host")]
    pub host: Option<String>,

    #[serde(default = "default_redis_port")]
    pub port: Option<String>,

    #[serde(default = "default_redis_ttl")]
    pub ttl_ban: u64,

    #[serde(default = "default_redis_ttl")]
    pub ttl_2fa: u64,
}

fn default_redis_ttl() -> u64 {
    600
}

fn default_redis_host() -> Option<String> {
    Some("127.0.0.1".to_string())
}

fn default_redis_port() -> Option<String> {
    Some("6379".to_string())
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            host: default_redis_host(),
            port: default_redis_port(),
            ttl_ban: default_redis_ttl(),
            ttl_2fa: default_redis_ttl(),
        }
    }
}

#[derive(serde::Deserialize, Default, Debug, Clone)]
pub struct Config {
    #[serde(default = "default_database_url")]
    pub database_url: Option<String>,

    #[serde(default = "DbConfig::default")]
    pub db: DbConfig,

    #[serde(default = "RedisConfig::default")]
    pub redis: RedisConfig,

    #[serde(default = "ServerConfig::default")]
    pub server: ServerConfig,

    #[serde(default = "TelemetryConfig::default")]
    pub telemetry: TelemetryConfig,

    #[serde(default = "ServerEnv::default")]
    pub env: ServerEnv,

    #[serde(default = "JwtConfig::default")]
    pub jwt: JwtConfig,

    #[serde(default = "EmailConfig::default")]
    pub email: EmailConfig,

    #[serde(default = "AppConfig::default")]
    pub app: AppConfig,
}

fn default_database_url() -> Option<String> {
    None
}

fn default_allowed_origins() -> Option<Vec<String>> {
    None
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

// fn default_true() -> bool {
//     true
// }

fn default_auth_redirect_url() -> String {
    "http://localhost:5173/login/2fa".to_string()
}

fn default_app_url() -> String {
    "http://localhost:5173".to_string()
}

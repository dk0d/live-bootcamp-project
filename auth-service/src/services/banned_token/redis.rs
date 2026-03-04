use crate::config::RedisConfig;
use crate::domain::{BannedTokenStore, RedisConnection, make_redis_key};
use crate::error::AuthApiError;
use redis::Commands;
use std::sync::Arc;
use tokio::sync::RwLock;

const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token";

#[derive(Debug)]
pub struct RedisBannedTokenStore {
    config: RedisConfig,
    conn: Arc<RwLock<RedisConnection>>,
}

impl RedisBannedTokenStore {
    pub fn new(config: &RedisConfig) -> Result<Self, AuthApiError> {
        if config.host.is_none() {
            return Err(AuthApiError::Config(
                "Redis Host Not Configured".to_string(),
            ));
        }
        let port = if let Some(p) = config.port.as_ref() {
            format!(":{}", p)
        } else {
            "".to_string()
        };
        let client =
            redis::Client::open(format!("redis://{}{}", &config.host.clone().unwrap(), port))
                .map_err(AuthApiError::Redis)?;
        let conn = RedisConnection(client.get_connection()?);
        Ok(Self {
            config: config.clone(),
            conn: Arc::new(RwLock::new(conn)),
        })
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for RedisBannedTokenStore {
    async fn ban_token(&mut self, token: &str) -> Result<(), crate::error::AuthApiError> {
        let token_key = make_redis_key(BANNED_TOKEN_KEY_PREFIX, token);
        let ttl = self.config.ttl_ban;
        let value = true;
        let mut guard = self.conn.write().await;
        guard
            .0
            .set_ex::<_, _, ()>(&token_key, value, ttl)
            .map_err(AuthApiError::Redis)?;
        Ok(())
    }

    async fn unban_token(&mut self, token: &str) -> Result<(), AuthApiError> {
        let mut guard = self.conn.write().await;
        let key = make_redis_key(BANNED_TOKEN_KEY_PREFIX, token);
        let _ = guard.0.del::<_, bool>(&key);
        Ok(())
    }

    async fn is_token_banned(&self, token: &str) -> bool {
        let key = make_redis_key(BANNED_TOKEN_KEY_PREFIX, token);
        let mut guard = self.conn.write().await;
        guard.0.exists::<_, bool>(&key).unwrap_or(false)
    }
}

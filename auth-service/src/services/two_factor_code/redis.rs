use std::sync::Arc;

use crate::{
    config::RedisConfig,
    domain::{
        Email, LoginAttemptId, RedisConnection, TwoFactorCode, TwoFactorCodeStore, TwoFactorMethod,
        make_redis_key,
    },
    error::AuthApiError,
};
use redis::Commands;
use tokio::sync::RwLock;

const TWO_FA_PREFIX: &str = "2fa";

#[derive(Clone, Debug)]
pub struct RedisTwoFactorStore {
    config: RedisConfig,
    conn: Arc<RwLock<RedisConnection>>,
}

impl RedisTwoFactorStore {
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

#[derive(serde::Deserialize, serde::Serialize)]
struct TwoFactorEntry {
    pub id: LoginAttemptId,
    pub code: TwoFactorCode,
}

#[async_trait::async_trait]
impl TwoFactorCodeStore for RedisTwoFactorStore {
    async fn new_login_attempt(
        &mut self,
        email: &Email,
        _two_factor_method: &TwoFactorMethod,
    ) -> Result<(LoginAttemptId, TwoFactorCode), AuthApiError> {
        let code = TwoFactorCode::new();
        let id = LoginAttemptId::new();
        let key = make_redis_key(TWO_FA_PREFIX, email.as_ref());
        let mut guard = self.conn.write().await;
        let entry = TwoFactorEntry {
            id: id.clone(),
            code: code.clone(),
        };
        guard
            .0
            .set_ex::<_, _, ()>(
                &key,
                &serde_json::to_string(&entry).expect("should be json"),
                self.config.ttl_2fa,
            )
            .map_err(AuthApiError::Redis)?;

        Ok((id, code))
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFactorCode), AuthApiError> {
        // write is needed for &mut self on Connection
        let mut guard = self.conn.write().await;
        let key = make_redis_key(TWO_FA_PREFIX, email.as_ref());
        let value = guard.0.get::<_, String>(&key)?;
        let entry: TwoFactorEntry = serde_json::from_str(&value)
            .map_err(|e| AuthApiError::SerializationError(format!("{e}")))?;
        Ok((entry.id, entry.code))
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), AuthApiError> {
        let mut guard = self.conn.write().await;
        let key = make_redis_key(TWO_FA_PREFIX, email.as_ref());
        let _ = guard.0.del::<_, String>(&key);
        Ok(())
    }
}

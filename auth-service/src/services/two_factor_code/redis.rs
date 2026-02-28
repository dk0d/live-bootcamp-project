use crate::{
    config::RedisConfig,
    domain::{Email, LoginAttemptId, TwoFactorCode, TwoFactorCodeStore, TwoFactorMethod},
    error::AuthApiError,
};

#[derive(Debug, Clone)]
pub struct RedisTwoFactorStore {}

impl RedisTwoFactorStore {
    pub fn new(config: &RedisConfig) -> Self {
        Self {}
    }
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

        todo!()
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFactorCode), AuthApiError> {
        todo!()
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), AuthApiError> {
        todo!()
    }
}

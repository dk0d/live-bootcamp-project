use crate::domain::{Email, LoginAttemptId, Password, TwoFactorCode, TwoFactorMethod, User};
use crate::error::AuthApiError;

#[async_trait::async_trait]
pub trait UserStore: Send + Sync + std::fmt::Debug {
    async fn add_user(&mut self, user: User) -> Result<(), AuthApiError>;
    async fn get_user(&self, email: &Email) -> Result<User, AuthApiError>;
    async fn validate_credentials(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<bool, AuthApiError> {
        match self.get_user(email).await {
            Ok(user) => user
                .password
                .verify_raw_password(password.as_ref())
                .await
                .map(|_| true)
                .map_err(|_| AuthApiError::Unauthorized),
            Err(_) => Err(AuthApiError::UserNotFound),
        }
    }
}

#[async_trait::async_trait]
pub trait BannedTokenStore: Send + Sync + std::fmt::Debug {
    async fn ban_token(&mut self, token: &str) -> Result<(), AuthApiError>;
    async fn unban_token(&mut self, token: &str) -> Result<(), AuthApiError>;
    async fn is_token_banned(&self, token: &str) -> bool;
}

#[async_trait::async_trait]
pub trait TwoFactorCodeStore: Send + Sync + std::fmt::Debug {
    async fn new_login_attempt(
        &mut self,
        email: &Email,
        two_factor_method: &TwoFactorMethod,
    ) -> Result<(LoginAttemptId, TwoFactorCode), AuthApiError>;

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFactorCode), AuthApiError>;

    async fn remove_code(&mut self, email: &Email) -> Result<(), AuthApiError>;

    async fn verify_code(
        &self,
        email: &Email,
        attempt_id: &LoginAttemptId,
        code: &TwoFactorCode,
    ) -> Result<bool, AuthApiError> {
        let (id, stored_code) = self.get_code(email).await?;
        Ok(stored_code == *code && id == *attempt_id)
    }
}

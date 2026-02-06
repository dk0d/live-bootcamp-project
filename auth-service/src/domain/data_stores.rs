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
    ) -> Result<bool, AuthApiError>;
}

pub trait BannedTokenStore: Send + Sync + std::fmt::Debug {
    fn ban_token(&mut self, token: impl ToString) -> Result<(), AuthApiError>;
    fn unban_token(&mut self, token: impl ToString) -> Result<(), AuthApiError>;
    fn is_token_banned(&self, token: impl ToString) -> bool;
}

pub trait TwoFactorCodeStore: Send + Sync + std::fmt::Debug {
    fn new_login_attempt(
        &mut self,
        email: &Email,
        two_factor_method: &TwoFactorMethod,
    ) -> Result<(LoginAttemptId, TwoFactorCode), AuthApiError>;

    fn get_code(&self, email: &Email) -> Result<(LoginAttemptId, TwoFactorCode), AuthApiError>;

    fn remove_code(&mut self, email: &Email) -> Result<(), AuthApiError>;

    fn verify_code(
        &self,
        email: &Email,
        attempt_id: &LoginAttemptId,
        code: &TwoFactorCode,
    ) -> Result<bool, AuthApiError> {
        let (id, stored_code) = self.get_code(email)?;
        Ok(stored_code == *code && id == *attempt_id)
    }
}

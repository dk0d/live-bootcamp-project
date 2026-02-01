use crate::domain::{Email, Password, User};
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

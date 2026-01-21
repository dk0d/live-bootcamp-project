use crate::domain::User;
use crate::services::user_store::UserStoreError;

pub trait UserStore: Send + Sync + std::fmt::Debug {
    fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    fn get_user(&self, email: &str) -> Result<User, UserStoreError>;
    fn validate_credentials(&self, email: &str, password: &str) -> Result<bool, UserStoreError>;
}

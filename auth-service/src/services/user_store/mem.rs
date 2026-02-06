use std::collections::HashMap;

use crate::domain::{Email, Password, User, UserStore};
use crate::error::AuthApiError;
use crate::utils::auth::hash_password;

#[derive(Debug, Clone)]
pub struct InMemoryUserStore {
    users: HashMap<Email, User>,
}

impl InMemoryUserStore {
    pub fn new() -> Self {
        InMemoryUserStore {
            users: HashMap::new(),
        }
    }
}

impl Default for InMemoryUserStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl UserStore for InMemoryUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), AuthApiError> {
        if self.users.contains_key(&user.email) {
            return Err(AuthApiError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, AuthApiError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(AuthApiError::UserNotFound),
        }
    }

    async fn validate_credentials(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<bool, AuthApiError> {
        match self.users.get(email) {
            Some(user) => match hash_password(password.as_ref()) {
                Ok(hashed_password) => {
                    if user.password.as_ref() == hashed_password {
                        Ok(true)
                    } else {
                        Err(AuthApiError::InvalidCredentials)
                    }
                }
                Err(e) => Err(AuthApiError::UnexpectedError(e.to_string())),
            },
            None => Err(AuthApiError::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::TwoFactorMethod;

    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = InMemoryUserStore::new();
        let user = User {
            email: "me@you.com".try_into().unwrap(),
            password: Password::parse("hashed_password").unwrap().into(),
            two_factor: TwoFactorMethod::None,
        };
        let res = store.add_user(user).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = InMemoryUserStore::new();
        let user = User {
            email: "me@you.com".try_into().unwrap(),
            password: Password::parse("hashed_password").unwrap().into(),
            two_factor: TwoFactorMethod::None,
        };
        _ = store.add_user(user).await;
        assert_eq!(store.users.len(), 1);
        assert!(
            store
                .get_user(&Email::parse("me@you.com").unwrap())
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = InMemoryUserStore::new();
        let user = User {
            email: "me@you.com".try_into().unwrap(),
            password: Password::parse("password").unwrap().into(),
            two_factor: TwoFactorMethod::None,
        };
        _ = store.add_user(user).await;
        assert_eq!(store.users.len(), 1);
        assert!(
            store
                .validate_credentials(
                    &Email::parse("me@you.com").unwrap(),
                    &Password::parse("password").unwrap()
                )
                .await
                .is_ok()
        );
    }
}

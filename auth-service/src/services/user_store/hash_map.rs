use std::collections::HashMap;

use crate::domain::User;
use crate::services::user_store::traits::UserStore;
use crate::services::user_store::UserStoreError;
use crate::utils::crypto::hash_password;

#[derive(Debug, Clone)]
pub struct HashMapUserUserStore {
    users: HashMap<String, User>,
}

impl HashMapUserUserStore {
    pub fn new() -> Self {
        HashMapUserUserStore {
            users: HashMap::new(),
        }
    }
}

impl Default for HashMapUserUserStore {
    fn default() -> Self {
        Self::new()
    }
}

impl UserStore for HashMapUserUserStore {
    fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    fn validate_credentials(&self, email: &str, password: &str) -> Result<bool, UserStoreError> {
        match self.users.get(email) {
            Some(user) => match hash_password(password) {
                Ok(hashed_password) => {
                    if user.hashed_password == hashed_password {
                        Ok(true)
                    } else {
                        Err(UserStoreError::InvalidCredentials)
                    }
                }
                Err(e) => Err(UserStoreError::UnexpectedError(e.to_string())),
            },
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashMapUserUserStore::new();
        let user = User {
            email: "me@you.com".to_string(),
            hashed_password: "hashed_password".to_string(),
            requires_2fa: false,
        };
        let res = store.add_user(user);
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashMapUserUserStore::new();
        let user = User {
            email: "me@you.com".to_string(),
            hashed_password: "hashed_password".to_string(),
            requires_2fa: false,
        };
        _ = store.add_user(user);
        assert_eq!(store.users.len(), 1);
        assert!(store.get_user("me@you.com").is_ok());
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashMapUserUserStore::new();
        let user = User {
            email: "me@you.com".to_string(),
            hashed_password: "hashed_password".to_string(),
            requires_2fa: false,
        };
        _ = store.add_user(user);
        assert_eq!(store.users.len(), 1);
        assert!(store.validate_credentials("me@you.com", "password").is_ok());
    }
}

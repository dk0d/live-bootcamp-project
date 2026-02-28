use std::collections::HashMap;

use crate::domain::{Email, LoginAttemptId, TwoFactorCode, TwoFactorCodeStore, TwoFactorMethod};
use crate::error::AuthApiError;

#[derive(Debug, Default)]
pub struct InMemoryTwoFactorCodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFactorCode)>,
}

#[async_trait::async_trait]
impl TwoFactorCodeStore for InMemoryTwoFactorCodeStore {
    async fn new_login_attempt(
        &mut self,
        email: &Email,
        _two_factor_method: &TwoFactorMethod,
    ) -> Result<(LoginAttemptId, TwoFactorCode), AuthApiError> {
        let code = TwoFactorCode::new();
        let id = LoginAttemptId::new();
        self.codes.insert(email.clone(), (id.clone(), code.clone()));
        Ok((id, code))
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFactorCode), AuthApiError> {
        self.codes
            .get(email)
            .cloned()
            .ok_or(AuthApiError::TwoFactorCodeNotFound)
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), AuthApiError> {
        self.codes
            .remove(email)
            .map(|_| ())
            .ok_or(AuthApiError::TwoFactorCodeNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::TwoFactorCodeStore;

    #[tokio::test]
    async fn test_2fa_store_create_and_get_code() {
        let mut store = InMemoryTwoFactorCodeStore::default();
        let email = Email::parse("user@test.com").expect("valid email");
        let (login_attempt_id, _code) = store
            .new_login_attempt(&email, &TwoFactorMethod::Email)
            .await
            .expect("create login attempt");
        let (retrieved_id, _retrieved_code) = store.get_code(&email).await.expect("get code");
        assert_eq!(login_attempt_id, retrieved_id);
    }

    #[tokio::test]
    async fn test_2fa_store_remove_code() {
        let mut store = InMemoryTwoFactorCodeStore::default();
        let email = Email::parse("user@test.com").expect("valid email");
        let (_login_attempt_id, _code) = store
            .new_login_attempt(&email, &TwoFactorMethod::Email)
            .await
            .expect("create login attempt");
        store.remove_code(&email).await.expect("remove code");
        let result = store.get_code(&email).await;
        assert!(matches!(result, Err(AuthApiError::TwoFactorCodeNotFound)));
    }

    #[tokio::test]
    async fn test_2fa_store_verify_code() {
        let mut store = InMemoryTwoFactorCodeStore::default();
        let email = Email::parse("user@test.com").expect("valid email");
        let (login_attempt_id, code) = store
            .new_login_attempt(&email, &TwoFactorMethod::Email)
            .await
            .expect("create login attempt");
        let res = store
            .verify_code(&email, &login_attempt_id, &code)
            .await
            .expect("verify code");
        assert!(res);
    }

    #[tokio::test]
    async fn test_2fa_store_verify_code_fails() {
        let mut store = InMemoryTwoFactorCodeStore::default();
        let email = Email::parse("user@test.com").expect("valid email");
        let (login_attempt_id, code) = store
            .new_login_attempt(&email, &TwoFactorMethod::Email)
            .await
            .expect("create login attempt");

        let wrong_code = TwoFactorCode::new();
        let res = store
            .verify_code(&email, &login_attempt_id, &wrong_code)
            .await
            .expect("verify code");
        assert!(!res);

        let wrong_id = LoginAttemptId::new();
        let res = store
            .verify_code(&email, &wrong_id, &code)
            .await
            .expect("verify code");
        assert!(!res);

        let wrong_email = Email::parse("other@test.com").expect("valid email");
        let res = store
            .verify_code(&wrong_email, &login_attempt_id, &code)
            .await;
        assert!(matches!(res, Err(AuthApiError::TwoFactorCodeNotFound)));
    }
}

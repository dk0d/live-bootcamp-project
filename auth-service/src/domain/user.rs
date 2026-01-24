use serde::{Deserialize, Serialize};

use crate::error::AuthApiError;
use crate::utils::auth::hash_password;

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn parse(email: &str) -> Result<Self, AuthApiError> {
        if !email.contains('@') || !email.contains('.') {
            return Err(AuthApiError::InvalidEmail(email.to_string()));
        }
        let (local_part, domain_part) = email.split_once('@').unwrap();
        let (host, tld) = domain_part.split_once('.').unwrap();
        if local_part.is_empty() || tld.len() < 2 || host.is_empty() {
            return Err(AuthApiError::InvalidEmail(email.to_string()));
        }
        Ok(Email(email.to_string()))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for Email {
    type Error = AuthApiError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Email::parse(value)
    }
}

impl TryFrom<String> for Email {
    type Error = AuthApiError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Email::parse(&value)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    pub fn parse(password: &str) -> Result<Self, AuthApiError> {
        if password.len() < 8 {
            return Err(AuthApiError::PasswordTooShort(password.len()));
        }
        Ok(Password(password.to_string()))
    }
}

impl TryFrom<&str> for Password {
    type Error = AuthApiError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Password::parse(value)
    }
}

impl TryFrom<String> for Password {
    type Error = AuthApiError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Password::parse(&value)
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn parse(password: &str) -> Result<Self, AuthApiError> {
        let password = Password::parse(password)?;
        let hashed = hash_password(password.as_ref())?;
        Ok(HashedPassword(hashed))
    }
}

impl From<Password> for HashedPassword {
    fn from(password: Password) -> Self {
        let hashed = hash_password(&password.0).expect("Failed to hash password");
        HashedPassword(hashed)
    }
}

impl AsRef<str> for HashedPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub email: Email,
    pub password: HashedPassword,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
        Self {
            email,
            password: password.into(),
            requires_2fa,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_invalid() {
        let cases = vec!["plainaddress", "@no-local-part.com", "Outlook Contact"];

        for case in cases {
            let result = Email::parse(case);
            assert!(result.is_err(), "Expected error for case: {}", case);
        }
    }

    #[test]
    fn test_email_valid() {
        let cases = vec!["there@go.com", "hello@joy.net", "what@me.org"];
        for case in cases {
            let result = Email::parse(case);
            assert!(result.is_ok(), "Expected success for case: {}", case);
        }
    }

    #[test]
    fn test_password_too_short() {
        let short_password = "short";
        let result = Password::parse(short_password);
        assert!(result.is_err());
    }

    #[test]
    fn test_password_valid() {
        let valid_password = "longenoughpassword";
        let result = Password::parse(valid_password);
        assert!(result.is_ok());
    }
}

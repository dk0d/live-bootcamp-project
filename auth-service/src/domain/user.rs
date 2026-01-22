use serde::{Deserialize, Serialize};

use crate::utils::crypto::hash_password;

#[derive(Debug, thiserror::Error, Clone)]
pub enum DataError {
    #[error("Invalid format: {0}")]
    InvalidData(String),

    #[error("Password too short: {0}")]
    PasswordTooShort(usize),

    /// Error for invalid email format
    #[error("Invalid email: {0}")]
    InvalidEmail(String),

    #[error("Missing field: {0}")]
    MissingField(String),
}

impl DataError {
    pub fn status_code(&self) -> axum::http::StatusCode {
        match self {
            DataError::InvalidData(_) => axum::http::StatusCode::BAD_REQUEST,
            DataError::PasswordTooShort(_) => axum::http::StatusCode::BAD_REQUEST,
            DataError::InvalidEmail(_) => axum::http::StatusCode::BAD_REQUEST,
            DataError::MissingField(_) => axum::http::StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn parse(email: &str) -> Result<Self, DataError> {
        if !email.contains('@') || !email.contains('.') {
            return Err(DataError::InvalidEmail(email.to_string()));
        }
        let (local_part, domain_part) = email.split_at(email.find('@').unwrap());
        if local_part.is_empty() || domain_part.len() < 2 {
            return Err(DataError::InvalidEmail(email.to_string()));
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
    type Error = DataError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Email::parse(value)
    }
}

impl TryFrom<String> for Email {
    type Error = DataError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Email::parse(&value)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    pub fn parse(password: &str) -> Result<Self, DataError> {
        if password.len() < 8 {
            return Err(DataError::PasswordTooShort(password.len()));
        }
        let p = hash_password(password)
            .map_err(|e| DataError::InvalidData(format!("Failed to hash password: {}", e)))?;
        Ok(Password(p))
    }
}

impl TryFrom<&str> for Password {
    type Error = DataError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Password::parse(value)
    }
}

impl TryFrom<String> for Password {
    type Error = DataError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Password::parse(&value)
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub email: Email,
    pub hashed_password: Password,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: Email, hashed_password: Password, requires_2fa: bool) -> Self {
        Self {
            email,
            hashed_password,
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
        if let Err(e) = result {
            match e {
                DataError::PasswordTooShort(len) => assert_eq!(len, short_password.len()),
                _ => panic!("Expected PasswordTooShort error"),
            }
        }
    }

    #[test]
    fn test_password_valid() {
        let valid_password = "longenoughpassword";
        let result = Password::parse(valid_password);
        assert!(result.is_ok());
    }
}

use crate::domain::TwoFactorMethod;
use crate::error::AuthApiError;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::HashedPassword;
use super::db::UserRow;

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq, ToSchema)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub email: Email,
    pub password: HashedPassword,
    pub two_factor: TwoFactorMethod,
}

impl From<UserRow> for User {
    fn from(value: UserRow) -> Self {
        Self {
            // NOTE: feels dangerous to have these expects here
            email: Email::parse(&value.email).expect("valid email"),
            password: HashedPassword::parse_password_hash(value.password_hash)
                .expect("valid hash from db"),
            two_factor: value.two_factor.try_into().unwrap_or_default(),
        }
    }
}

impl From<User> for UserRow {
    fn from(value: User) -> Self {
        Self {
            email: value.email.as_ref().to_owned(),
            password_hash: value.password.as_ref().to_owned(),
            two_factor: value.two_factor.to_string(),
        }
    }
}

impl User {
    pub fn new(email: Email, password: HashedPassword, two_factor: TwoFactorMethod) -> Self {
        Self {
            email,
            password,
            two_factor,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn email_invalid() {
        let cases = vec!["plainaddress", "@no-local-part.com", "Outlook Contact"];

        for case in cases {
            let result = Email::parse(case);
            assert!(result.is_err(), "Expected error for case: {}", case);
        }
    }

    #[test]
    fn email_valid() {
        let cases = vec!["there@go.com", "hello@joy.net", "what@me.org"];
        for case in cases {
            let result = Email::parse(case);
            assert!(result.is_ok(), "Expected success for case: {}", case);
        }
    }
}

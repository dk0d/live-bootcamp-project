use crate::error::AuthApiError;
use utoipa::ToSchema;
use uuid::Uuid;

use rand::Rng;
use rand::distr::Uniform;

#[derive(serde::Deserialize, serde::Serialize, Debug, ToSchema, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TwoFactorMethod {
    Email,

    #[default]
    None,
}

fn gen_code() -> Result<String, AuthApiError> {
    let mut rng = rand::rng();
    let distribution = Uniform::new(0, 10).map_err(|_| AuthApiError::TwoFactorCodeGenFailed)?;
    let code = (0..6)
        .map(|_| rng.sample(&distribution).to_string())
        .collect();
    Ok(code)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, ToSchema, serde::Serialize, serde::Deserialize)]
pub struct TwoFactorCode(String);

impl TwoFactorCode {
    pub fn new() -> Self {
        let code = gen_code().expect("Failed to generate two factor code");
        TwoFactorCode(code)
    }
}

impl Default for TwoFactorCode {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<String> for TwoFactorCode {
    type Error = AuthApiError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 6 || !value.chars().all(|c| c.is_ascii_digit()) {
            return Err(AuthApiError::InvalidData(
                "Two factor code must be 6 digits".to_string(),
            ));
        }
        Ok(TwoFactorCode(value))
    }
}

impl TryFrom<&str> for TwoFactorCode {
    type Error = AuthApiError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 6 || !value.chars().all(|c| c.is_ascii_digit()) {
            return Err(AuthApiError::InvalidData(
                "Two factor code must be 6 digits".to_string(),
            ));
        }
        Ok(TwoFactorCode(value.to_string()))
    }
}

impl AsRef<str> for TwoFactorCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, ToSchema, serde::Serialize, serde::Deserialize)]
pub struct LoginAttemptId(Uuid);

impl LoginAttemptId {
    pub fn new() -> Self {
        LoginAttemptId(Uuid::new_v4())
    }
}
impl Default for LoginAttemptId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<String> for LoginAttemptId {
    type Error = AuthApiError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let uuid = Uuid::parse_str(&value).map_err(|_| AuthApiError::InvalidLoginAttemptId)?;
        Ok(LoginAttemptId(uuid))
    }
}

impl TryFrom<&str> for LoginAttemptId {
    type Error = AuthApiError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let uuid = Uuid::parse_str(value).map_err(|_| AuthApiError::InvalidLoginAttemptId)?;
        Ok(LoginAttemptId(uuid))
    }
}

impl AsRef<Uuid> for LoginAttemptId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_two_factor_code_try_from() {
        let valid_code = "123456".to_string();
        let invalid_code = "12345a".to_string();

        let code = TwoFactorCode::try_from(valid_code.clone()).unwrap();
        assert_eq!(code.as_ref(), valid_code);

        let err = TwoFactorCode::try_from(invalid_code).unwrap_err();
        assert!(matches!(err, AuthApiError::InvalidData(_)));
    }

    #[test]
    fn test_login_attempt_id_try_from() {
        let valid_uuid = Uuid::new_v4().to_string();
        let invalid_uuid = "invalid-uuid".to_string();

        let attempt_id = LoginAttemptId::try_from(valid_uuid.clone()).unwrap();
        assert_eq!(attempt_id.as_ref().to_string(), valid_uuid);

        let err = LoginAttemptId::try_from(invalid_uuid).unwrap_err();
        assert!(matches!(err, AuthApiError::InvalidLoginAttemptId));
    }

    #[test]
    fn test_login_attempt_id_serialize() {
        let attempt_id = LoginAttemptId::new();
        let serialized = serde_json::to_string(&attempt_id).unwrap();
        let expected = format!("\"{}\"", attempt_id.as_ref());
        assert_eq!(serialized, expected);
    }
}

use axum_extra::extract::cookie::{Cookie, SameSite};

use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation};
use serde::Deserialize;

use crate::domain::Email;
use crate::error::AuthApiError;

pub fn hash_password(password: &str) -> Result<String, AuthApiError> {
    // FIXME: Replace with real hashing logic
    let hashed = format!("hashed_{}", password);
    Ok(hashed)
}

#[derive(Debug, thiserror::Error)]
pub enum GenerateTokenError {
    #[error("JWT Encoding Error: {0}")]
    Encoding(jsonwebtoken::errors::Error),

    #[error("JWT Encoding Error: {0}")]
    Decoding(jsonwebtoken::errors::Error),

    #[error("JWT Validation Error: {0}")]
    Validation(jsonwebtoken::errors::Error),

    #[error("Unexpected Error: {0}")]
    UnexpectedError(String),
}

#[derive(serde::Serialize, Deserialize, Debug)]
pub struct Claims {
    sub: String,
    exp: usize,
}

fn create_auth_cookie(name: &str, token: String) -> Cookie<'static> {
    let cookie = Cookie::build((name.to_string(), token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();
    cookie
}

fn generate_auth_token(email: &Email, secret: &str) -> Result<String, GenerateTokenError> {
    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;
    let claims = Claims {
        sub: email.as_ref().to_string(),
        exp,
    };
    generate_auth_token_with_claims::<Claims>(&claims, secret)
}

fn generate_auth_token_with_claims<C>(
    claims: &C,
    secret: &str,
) -> Result<String, GenerateTokenError>
where
    C: serde::Serialize,
{
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(GenerateTokenError::Encoding)
}

fn create_token(claims: &Claims, secret: &str) -> Result<String, GenerateTokenError> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(GenerateTokenError::Encoding)
}

pub async fn validate_token<C>(token: &str, secret: &str) -> Result<C, GenerateTokenError>
where
    C: serde::de::DeserializeOwned,
{
    jsonwebtoken::decode::<C>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(GenerateTokenError::Decoding)
    .map(|data| data.claims)
}

pub async fn validate_token_validator<C>(
    token: &str,
    secret: &str,
    validator: &Validation,
) -> Result<C, GenerateTokenError>
where
    C: serde::de::DeserializeOwned,
{
    jsonwebtoken::decode::<C>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        validator,
    )
    .map_err(GenerateTokenError::Decoding)
    .map(|data| data.claims)
}

pub fn generate_auth_cookie(
    email: &Email,
    name: &str,
    secret: &str,
) -> Result<Cookie<'static>, GenerateTokenError> {
    let token = generate_auth_token(email, secret)?;
    Ok(create_auth_cookie(name, token))
}

#[cfg(test)]
mod tests {

    use chrono::Utc;

    use super::*;
    const JWT_COOKIE_NAME: &str = "auth_token";
    const JWT_SECRET: &str = "your_secret_key";

    #[tokio::test]
    async fn test_auth_generate_cookie() {
        let email = Email::parse("test@example.com").unwrap();
        let cookie = generate_auth_cookie(&email, JWT_COOKIE_NAME, JWT_SECRET).unwrap();
        assert_eq!(cookie.name(), JWT_COOKIE_NAME.to_string());
        assert_eq!(cookie.value().split('.').count(), 3);
        assert_eq!(cookie.path(), Some("/"));
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.same_site(), Some(SameSite::Lax));
    }

    #[tokio::test]
    async fn test_auth_create_cookie() {
        let token = "test_token".to_owned();
        let cookie = create_auth_cookie(JWT_COOKIE_NAME, token.clone());
        assert_eq!(cookie.name(), JWT_COOKIE_NAME);
        assert_eq!(cookie.value(), token);
        assert_eq!(cookie.path(), Some("/"));
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.same_site(), Some(SameSite::Lax));
    }

    #[tokio::test]
    async fn test_auth_generate_token() {
        let email = Email::parse("test@example.com").unwrap();
        let result = generate_auth_token(&email, JWT_SECRET).unwrap();
        assert_eq!(result.split('.').count(), 3);
    }

    #[tokio::test]
    async fn test_auth_validate_token_valid() {
        let email = Email::parse("test@example.com").unwrap();
        let token = generate_auth_token(&email, JWT_SECRET).expect("valid token");
        let result: Claims = validate_token(&token, JWT_SECRET)
            .await
            .expect("valid token");
        assert_eq!(result.sub, "test@example.com");

        let exp = Utc::now()
            .checked_add_signed(chrono::Duration::try_minutes(9).expect("valid duration"))
            .expect("valid timestamp")
            .timestamp();

        assert!(result.exp > exp as usize);
    }

    #[tokio::test]
    async fn test_auth_validate_token_invalid() {
        let token = "invalid_token".to_owned();
        let result = validate_token::<Claims>(&token, JWT_SECRET).await;
        assert!(result.is_err());
    }
}

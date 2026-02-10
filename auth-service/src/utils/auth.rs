use axum_extra::extract::cookie::{Cookie, SameSite};

use base64::Engine;
use base64::engine::GeneralPurpose;
use jsonwebtoken::{Algorithm, Header, Validation, encode};
use serde::Deserialize;
use tokio::sync::OnceCell;

use crate::config::{JwtConfig, JwtKeySecret};
use crate::domain::{Email, LoginAttemptId};
use crate::error::AuthApiError;
use crate::state::AppState;
use base64::engine::general_purpose::STANDARD;
use jsonwebtoken::jwk::{JwkSet, KeyAlgorithm};

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

/// Claims for 2FA tokens, which include the login attempt ID and email
///
/// This is used in the 2FA redirect_url {url}?payload={jwt}
///
/// and the landing site can validate the token and grab the email and login attempt ID
/// to complete the 2FA flow
///
/// The exp claim is used to ensure the token is only valid for a short period of time (e.g. 15 minutes)
#[derive(serde::Serialize, Deserialize, Debug)]
pub struct TwoFAClaims {
    sub: LoginAttemptId,
    exp: usize,
    email: Email,
}

static KEYS: OnceCell<JwkSet> = OnceCell::const_new();

fn get_decoding_key(secret: &JwtKeySecret) -> (jsonwebtoken::DecodingKey, jsonwebtoken::Algorithm) {
    match &secret {
        JwtKeySecret::ECDSA { pub_key, .. } => {
            let key = std::fs::read_to_string(pub_key).expect("Failed to read Pub PEM key");
            (
                jsonwebtoken::DecodingKey::from_ec_pem(key.as_bytes()).expect("valid key"),
                jsonwebtoken::Algorithm::ES256,
            )
        }
        JwtKeySecret::Raw { value: secret } => (
            jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
            jsonwebtoken::Algorithm::HS256,
        ),
    }
}

fn get_encoding_key(secret: &JwtKeySecret) -> (jsonwebtoken::EncodingKey, jsonwebtoken::Algorithm) {
    match &secret {
        JwtKeySecret::ECDSA { priv_key, .. } => {
            let key = std::fs::read_to_string(priv_key).expect("Failed to read Priv PEM key");
            (
                jsonwebtoken::EncodingKey::from_ec_pem(key.as_bytes()).expect("valid key"),
                jsonwebtoken::Algorithm::ES256,
            )
        }
        JwtKeySecret::Raw { value: secret } => (
            jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
            jsonwebtoken::Algorithm::HS256,
        ),
    }
}

pub async fn get_jwks(state: &AppState) -> &'static JwkSet {
    KEYS.get_or_init(|| async {
        // let decode_key = jsonwebtoken::DecodingKey::from_secret(state.config.jwt.secret.as_bytes());
        let (encode_key, alg) = get_encoding_key(&state.config.jwt.secret);
        // let encode_key = jsonwebtoken::EncodingKey::from_ec_pem(secret.as_bytes());
        let mut key =
            jsonwebtoken::jwk::Jwk::from_encoding_key(&encode_key, alg).expect("Valid Keys");
        key.common.key_algorithm = match alg {
            Algorithm::ES256 => Some(KeyAlgorithm::ES256),
            Algorithm::HS256 => Some(KeyAlgorithm::HS256),
            _ => panic!("Failure to match keys"),
        };
        jsonwebtoken::jwk::JwkSet { keys: vec![key] }
    })
    .await
}

fn create_auth_cookie(name: &str, token: String) -> Cookie<'static> {
    Cookie::build((name.to_string(), token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build()
}

pub fn get_jwt_header(secret: &JwtKeySecret) -> jsonwebtoken::Header {
    let mut header = jsonwebtoken::Header::new(secret.alg());
    header.typ = Some("jwt".to_string());
    header
}

pub fn generate_auth_token(
    email: &Email,
    secret: &JwtKeySecret,
) -> Result<String, GenerateTokenError> {
    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: email.as_ref().to_string(),
        exp,
    };

    let header = get_jwt_header(secret);

    generate_auth_token_with_claims::<Claims>(&header, &claims, secret)
}

pub fn generate_2fa_token(
    id: &LoginAttemptId,
    email: &Email,
    secret: &JwtKeySecret,
) -> Result<String, GenerateTokenError> {
    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::minutes(5))
        .expect("valid timestamp")
        .timestamp() as usize;
    let claims = TwoFAClaims {
        sub: id.clone(),
        exp,
        email: email.clone(),
    };
    let header = get_jwt_header(secret);
    let token = generate_auth_token_with_claims::<TwoFAClaims>(&header, &claims, secret)?;
    let mut buf = String::new();
    let engine = GeneralPurpose::new(
        &base64::alphabet::URL_SAFE,
        base64::engine::general_purpose::PAD,
    );
    engine.encode_string(&token, &mut buf);
    Ok(buf)
}

fn generate_auth_token_with_claims<C>(
    header: &Header,
    claims: &C,
    secret: &JwtKeySecret,
) -> Result<String, GenerateTokenError>
where
    C: serde::Serialize,
{
    let (encoding_key, _) = get_encoding_key(secret);
    encode(header, &claims, &encoding_key).map_err(GenerateTokenError::Encoding)
}

// fn create_token(claims: &Claims, secret: &str) -> Result<String, GenerateTokenError> {
//     encode(
//         &Header::default(),
//         claims,
//         &EncodingKey::from_secret(secret.as_ref()),
//     )
//     .map_err(GenerateTokenError::Encoding)
// }

pub async fn validate_token<C>(token: &str, config: &JwtConfig) -> Result<C, GenerateTokenError>
where
    C: serde::de::DeserializeOwned,
{
    let (key, alg) = get_decoding_key(&config.secret);
    jsonwebtoken::decode::<C>(token, &key, &Validation::new(alg))
        .map_err(GenerateTokenError::Decoding)
        .map(|data| data.claims)
}

pub fn generate_auth_cookie_raw(
    email: &Email,
    name: &str,
    secret: &JwtKeySecret,
) -> Result<Cookie<'static>, GenerateTokenError> {
    let token = generate_auth_token(email, secret)?;
    Ok(create_auth_cookie(name, token))
}

pub fn generate_auth_cookie(
    email: &Email,
    config: &JwtConfig,
) -> Result<Cookie<'static>, GenerateTokenError> {
    let token = generate_auth_token(email, &config.secret)?;
    Ok(create_auth_cookie(&config.cookie_name, token))
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
        let secret = JwtKeySecret::Raw {
            value: JWT_SECRET.to_string(),
        };
        let cookie = generate_auth_cookie_raw(&email, JWT_COOKIE_NAME, &secret).unwrap();
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
        let secret = JwtKeySecret::Raw {
            value: JWT_SECRET.to_string(),
        };
        let result = generate_auth_token(&email, &secret).unwrap();
        assert_eq!(result.split('.').count(), 3);
    }

    #[tokio::test]
    async fn test_auth_validate_token_valid() {
        let config = JwtConfig {
            secret: JwtKeySecret::Raw {
                value: JWT_SECRET.to_string(),
            },
            cookie_name: JWT_COOKIE_NAME.to_string(),
        };
        let email = Email::parse("test@example.com").unwrap();
        let token = generate_auth_token(&email, &config.secret).expect("valid token");
        let result: Claims = validate_token(&token, &config).await.expect("valid token");
        assert_eq!(result.sub, "test@example.com");

        let exp = Utc::now()
            .checked_add_signed(chrono::Duration::try_minutes(9).expect("valid duration"))
            .expect("valid timestamp")
            .timestamp();

        assert!(result.exp > exp as usize);
    }

    #[tokio::test]
    async fn test_auth_validate_token_valid_ecdsa() {
        let config = JwtConfig {
            secret: JwtKeySecret::ECDSA {
                pub_key: "tests/jwt-test.pub".to_string(),
                priv_key: "tests/jwt-test.pem".to_string(),
            },
            cookie_name: JWT_COOKIE_NAME.to_string(),
        };
        let email = Email::parse("test@example.com").unwrap();
        let token = generate_auth_token(&email, &config.secret).expect("valid token");
        let result: Claims = validate_token(&token, &config).await.expect("valid token");
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
        let config = JwtConfig {
            secret: JwtKeySecret::Raw {
                value: JWT_SECRET.to_string(),
            },
            cookie_name: JWT_COOKIE_NAME.to_string(),
        };
        let result = validate_token::<Claims>(&token, &config).await;
        assert!(result.is_err());
    }
}

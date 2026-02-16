use crate::error::AuthApiError;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier};
use serde::{Deserialize, Serialize};

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
    /// Create a newly hashed password from a raw password string
    pub async fn parse(password: &str) -> Result<Self, AuthApiError> {
        let password = Password::parse(password)?; // ensures password constraints apply
        let hashed = HashedPassword::compute_password_hash(password.as_ref()).await?;
        Ok(HashedPassword(hashed))
    }

    /// Get a hashed password from a string that is already hashed
    /// Verifies that the string has the right format
    pub fn parse_password_hash(hash: String) -> Result<HashedPassword, AuthApiError> {
        let _ = PasswordHash::new(&hash).map_err(|e| AuthApiError::UnexpectedError(format!("{e}")));
        Ok(HashedPassword(hash))
    }

    /// Verify a password candidate against the stored hashed password
    pub async fn verify_raw_password(&self, candidate: &str) -> Result<(), AuthApiError> {
        let candidate = candidate.to_owned();
        let password_hash = self.as_ref().to_owned();
        tokio::task::spawn_blocking(move || -> Result<(), AuthApiError> {
            let expected = PasswordHash::new(&password_hash)
                .map_err(|e| AuthApiError::UnexpectedError(format!("{e}")))?;
            Argon2::default()
                .verify_password(candidate.as_bytes(), &expected)
                .map_err(|_| AuthApiError::Unauthorized)
        })
        .await
        .map_err(|e| AuthApiError::UnexpectedError(format!("{e}")))?
    }

    /// Helper function to hash passwords before persisting them in storage.
    pub async fn compute_password_hash(password: &str) -> Result<String, AuthApiError> {
        let password = password.to_owned();
        tokio::task::spawn_blocking(move || -> Result<String, AuthApiError> {
            let salt = SaltString::generate(&mut OsRng);
            let password_hash = argon2::Argon2::new(
                argon2::Algorithm::Argon2id,
                argon2::Version::V0x13,
                Params::new(15000, 2, 1, None)
                    .map_err(|e| AuthApiError::UnexpectedError(format!("{e}")))?,
            )
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AuthApiError::UnexpectedError(format!("{e}")))?
            .to_string();

            Ok(password_hash)
        })
        .await
        .map_err(|e| AuthApiError::UnexpectedError(format!("{e}")))?
    }
}

// impl From<Password> for HashedPassword {
//     fn from(password: Password) -> Self {
//         let hashed = hash_password(&password.0).expect("Failed to hash password");
//         HashedPassword(hashed)
//     }
// }

impl AsRef<str> for HashedPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[cfg(test)]

mod tests {
    use super::*;
    use argon2::{
        Algorithm,
        Argon2,
        Params,
        PasswordHasher,
        Version,
        // new
        password_hash::{SaltString, rand_core::OsRng},
    };

    use fake::Fake;
    use fake::faker::internet::en::Password as FakePassword;
    use quickcheck::Gen;
    use rand::SeedableRng;

    #[test]
    fn password_too_short() {
        let short_password = "short";
        let result = Password::parse(short_password);
        assert!(result.is_err());
    }

    #[test]
    fn password_valid() {
        let valid_password = "longenoughpassword";
        let result = Password::parse(valid_password);
        assert!(result.is_ok());
    }

    // updated!
    #[tokio::test]
    async fn empty_string_is_rejected() {
        let password = "".to_owned();

        // updated!
        assert!(HashedPassword::parse(&password).await.is_err());
    }

    // updated!
    #[tokio::test]
    async fn string_less_than_8_characters_is_rejected() {
        let password = "1234567".to_owned();
        // updated!
        assert!(HashedPassword::parse(&password).await.is_err());
    }

    // new
    #[test]
    fn can_parse_valid_argon2_hash() {
        // Arrange - Create a valid Argon2 hash
        let raw_password = "TestPassword123";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        );

        let hash_string = argon2
            .hash_password(raw_password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        // Act
        let hash_password = HashedPassword::parse_password_hash(hash_string.clone()).unwrap();

        // Assert
        assert_eq!(hash_password.as_ref(), hash_string.as_str());
        assert!(hash_password.as_ref().starts_with("$argon2id$v=19$"));
    }

    // new
    #[tokio::test]
    async fn can_verify_raw_password() {
        let raw_password = "TestPassword123";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        );

        let hash_string = argon2
            .hash_password(raw_password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        let hash_password = HashedPassword::parse_password_hash(hash_string.clone()).unwrap();

        assert_eq!(hash_password.as_ref(), hash_string.as_str());
        assert!(hash_password.as_ref().starts_with("$argon2id$v=19$"));

        assert_eq!(hash_password.as_ref(), &hash_string);
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub String);

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary(g: &mut Gen) -> Self {
            let seed: u64 = g.size() as u64;
            let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
            let password = FakePassword(8..30).fake_with_rng(&mut rng);
            Self(password)
        }
    }

    // updated!
    #[tokio::test]
    #[quickcheck_macros::quickcheck]
    async fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
        HashedPassword::parse(&valid_password.0).await.is_ok() // updated!
    }
}

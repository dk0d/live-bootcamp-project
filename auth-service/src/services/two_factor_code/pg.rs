use sqlx::{PgPool, Row, postgres::PgPoolOptions};

use crate::{
    domain::{Email, LoginAttemptId, TwoFactorCode, TwoFactorCodeStore, TwoFactorMethod},
    error::AuthApiError,
};

#[derive(Debug, Clone)]
pub struct PostgresTwoFactorStore {
    pool: PgPool,
}

impl PostgresTwoFactorStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn new_opts(opts: PgPoolOptions, url: &str) -> Result<Self, AuthApiError> {
        let pool = opts.connect(url).await.map_err(AuthApiError::Db)?;
        Ok(Self { pool })
    }
}

#[async_trait::async_trait]
impl TwoFactorCodeStore for PostgresTwoFactorStore {
    async fn new_login_attempt(
        &mut self,
        email: &Email,
        _two_factor_method: &TwoFactorMethod,
    ) -> Result<(LoginAttemptId, TwoFactorCode), AuthApiError> {
        let code = TwoFactorCode::new();
        let id = LoginAttemptId::new();
        let added = sqlx::query(r#"INSERT INTO two_factor (email, id, code) VALUES ($1, $2, $3);"#)
            .bind(email.as_ref())
            .bind(id.as_ref().to_string())
            .bind(code.as_ref())
            .execute(&self.pool)
            .await
            .map_err(AuthApiError::Db)?;

        if added.rows_affected() != 1 {
            return Err(AuthApiError::TwoFactorCodeGenFailedToSave);
        }

        Ok((id, code))
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFactorCode), AuthApiError> {
        let result = sqlx::query(
            r#"
        SELECT email, id, two_factor_code FROM two_factor WHERE email = $1;
        "#,
        )
        .bind(email.as_ref())
        .fetch_one(&self.pool)
        .await
        .map_err(AuthApiError::Db)?;

        let code = TwoFactorCode::try_from(result.get::<String, _>("code"))?;
        let id = LoginAttemptId::try_from(result.get::<String, _>("id"))?;

        Ok((id, code))
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), AuthApiError> {
        let _ = sqlx::query(
            r#"
        DELETE FROM two_factor WHERE email = $1;
        "#,
        )
        .bind(email.as_ref())
        .fetch_one(&self.pool)
        .await
        .map_err(AuthApiError::Db)?;
        Ok(())
    }
}

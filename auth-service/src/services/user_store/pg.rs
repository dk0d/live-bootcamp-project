use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::{
    domain::{Email, User, UserRow, data_stores::UserStore},
    error::AuthApiError,
};

#[derive(Debug, Clone)]
pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn new_opts(opts: PgPoolOptions, url: &str) -> Result<Self, AuthApiError> {
        let pool = opts.connect(url).await.map_err(AuthApiError::Db)?;
        Ok(Self { pool })
    }
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), AuthApiError> {
        let row: UserRow = user.into();
        let result = sqlx::query!(
            r#"
        INSERT into "public"."user" 
            (email, password_hash, two_factor)
        values 
            ($1, $2, $3);
        "#,
            row.email,
            row.password_hash,
            row.two_factor,
        )
        .execute(&self.pool)
        .await
        .map_err(AuthApiError::Db)?;

        if result.rows_affected() < 1 {
            return Err(AuthApiError::UnexpectedError(
                "DB did not error, but user not added".to_string(),
            ));
        }

        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, AuthApiError> {
        let user_row = sqlx::query_as!(
            UserRow,
            r#"SELECT email, password_hash, two_factor from "public"."user" where email = $1;"#,
            email.as_ref(),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AuthApiError::Db)?;
        Ok(user_row.into())
    }
}

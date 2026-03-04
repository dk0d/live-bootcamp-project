use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use crate::config::Config;
use crate::error::AuthApiError;

/// Database connection wrapper
#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Connect to the database with the given settings
    pub async fn connect(config: &Config) -> anyhow::Result<Self> {
        let url = config.database_url.as_ref().ok_or(AuthApiError::Config(
            "Must have URL for db connection".to_string(),
        ))?;

        let pool = PgPoolOptions::new()
            .max_connections(config.db.max_connections)
            .min_connections(config.db.min_connections)
            .connect(url)
            .await?;

        Ok(Self { pool })
    }

    /// Get a reference to the connection pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Run a health check query
    pub async fn health_check(&self) -> anyhow::Result<()> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }
}

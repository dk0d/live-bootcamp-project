use crate::config::Config;
use crate::domain::{BannedTokenStore, EmailClient, TwoFactorCodeStore, UserStore};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type UserStoreType = Arc<RwLock<dyn UserStore>>;
pub type BannedTokenStoreType = Arc<RwLock<dyn BannedTokenStore>>;
pub type TwoFactorCodeStoreType = Arc<RwLock<dyn TwoFactorCodeStore>>;
pub type EmailClientType = Arc<RwLock<dyn EmailClient>>;
pub type PoolType = Arc<RwLock<PgPool>>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_tokens: BannedTokenStoreType,
    pub two_factor: TwoFactorCodeStoreType,
    pub email_client: EmailClientType,
    pub config: Config,
}

impl AppState {
    pub fn new(
        config: &Config,
        user_store: UserStoreType,
        banned_tokens: BannedTokenStoreType,
        two_factor: TwoFactorCodeStoreType,
        email_client: EmailClientType,
    ) -> Self {
        Self {
            config: config.clone(),
            banned_tokens,
            user_store,
            two_factor,
            email_client,
        }
    }
}

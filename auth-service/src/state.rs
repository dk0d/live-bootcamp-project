use crate::config::Config;
use crate::services::banned_token::mem::InMemoryBannedTokenStore;
use crate::services::user_store::InMemoryUserStore;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type UserStoreType = Arc<RwLock<InMemoryUserStore>>;
pub type BannedTokenStoreType = Arc<RwLock<InMemoryBannedTokenStore>>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_tokens: BannedTokenStoreType,
    pub config: Config,
}

impl AppState {
    pub fn new(
        config: &Config,
        user_store: UserStoreType,
        banned_tokens: BannedTokenStoreType,
    ) -> Self {
        Self {
            config: config.clone(),
            banned_tokens,
            user_store,
        }
    }
}

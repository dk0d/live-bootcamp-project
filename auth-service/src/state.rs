use crate::config::Config;
use crate::services::banned_token::mem::InMemoryBannedTokenStore;
use crate::services::email::Emailer;
use crate::services::two_factor_code::mem::InMemoryTwoFactorCodeStore;
use crate::services::user_store::InMemoryUserStore;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type UserStoreType = Arc<RwLock<InMemoryUserStore>>;
pub type BannedTokenStoreType = Arc<RwLock<InMemoryBannedTokenStore>>;
pub type TwoFactorCodeStoreType = Arc<RwLock<InMemoryTwoFactorCodeStore>>;
pub type EmailClientType = Arc<RwLock<Emailer>>;

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

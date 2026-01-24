use crate::config::Config;
use crate::services::user_store::HashMapUserUserStore;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type UserStoreType = Arc<RwLock<HashMapUserUserStore>>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub config: Config,
}

impl AppState {
    pub fn new(config: &Config, user_store: UserStoreType) -> Self {
        Self {
            config: config.clone(),
            user_store,
        }
    }
}

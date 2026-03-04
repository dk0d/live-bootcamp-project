use std::collections::HashSet;

use crate::domain::BannedTokenStore;
use crate::error::AuthApiError;

#[derive(Debug, Clone, Default)]
pub struct InMemoryBannedTokenStore {
    // TODO: Make hashmap of <Email, HashSet<Token>> to support multiple tokens per user
    // reduce collision chances (and/or token checksum)?
    // better to just add token id to claims and store banned token ids?
    tokens: HashSet<String>,
}

impl InMemoryBannedTokenStore {
    pub fn new() -> Self {
        Self {
            tokens: HashSet::new(),
        }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for InMemoryBannedTokenStore {
    async fn ban_token(&mut self, token: &str) -> Result<(), AuthApiError> {
        self.tokens.insert(token.to_string());
        Ok(())
    }

    async fn unban_token(&mut self, token: &str) -> Result<(), AuthApiError> {
        self.tokens.remove(token);
        Ok(())
    }

    async fn is_token_banned(&self, token: &str) -> bool {
        self.tokens.contains(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ban_unban_token() {
        let mut store = InMemoryBannedTokenStore::new();
        let token = "test_token";

        assert!(!store.is_token_banned(token).await);

        store.ban_token(token).await.unwrap();
        assert!(store.is_token_banned(token).await);

        store.unban_token(token).await.unwrap();
        assert!(!store.is_token_banned(token).await);
    }
}

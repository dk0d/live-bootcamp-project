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

impl BannedTokenStore for InMemoryBannedTokenStore {
    fn ban_token(&mut self, token: impl ToString) -> Result<(), AuthApiError> {
        self.tokens.insert(token.to_string());
        Ok(())
    }

    fn unban_token(&mut self, token: impl ToString) -> Result<(), AuthApiError> {
        self.tokens.remove(&token.to_string());
        Ok(())
    }

    fn is_token_banned(&self, token: impl ToString) -> bool {
        self.tokens.contains(&token.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ban_unban_token() {
        let mut store = InMemoryBannedTokenStore::new();
        let token = "test_token";

        assert!(!store.is_token_banned(token));

        store.ban_token(token).unwrap();
        assert!(store.is_token_banned(token));

        store.unban_token(token).unwrap();
        assert!(!store.is_token_banned(token));
    }
}

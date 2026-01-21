use serde::{Deserialize, Serialize};

use crate::routes::SignupRequest;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct User {
    pub email: String,
    pub hashed_password: String,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: String, hashed_password: String, requires_2fa: bool) -> Self {
        Self {
            email,
            hashed_password,
            requires_2fa,
        }
    }
}

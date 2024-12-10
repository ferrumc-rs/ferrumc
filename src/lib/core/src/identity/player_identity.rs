use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerIdentity {
    pub username: String,
    pub uuid: u128,
}

impl PlayerIdentity {
    pub fn new(username: String, uuid: u128) -> Self {
        Self { username, uuid }
    }
}

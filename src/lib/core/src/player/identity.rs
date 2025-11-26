use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlayerIdentityData {
    pub username: String,
    pub uuid: Uuid,
    pub short_uuid: i32,
}

impl PlayerIdentityData {
    pub fn new(username: String, uuid: u128) -> Self {
        Self {
            username,
            uuid: Uuid::from_u128(uuid),
            // Note: This truncates the UUID to 32 bits, which is standard
            // for Minecraft entity IDs derived from UUIDs.
            short_uuid: uuid as i32,
        }
    }
}

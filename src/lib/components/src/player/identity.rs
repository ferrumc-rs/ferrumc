use bevy_ecs::prelude::Component;
use ferrumc_core::player::identity::PlayerIdentityData;
use std::ops::{Deref, DerefMut};
use typename::TypeName;

#[derive(TypeName, Component, Debug, Default, Clone)]
pub struct PlayerIdentity(pub PlayerIdentityData);

impl PlayerIdentity {
    // Wrapper constructor for convenience
    pub fn new(username: String, uuid: u128) -> Self {
        Self(PlayerIdentityData::new(username, uuid))
    }
}

impl Deref for PlayerIdentity {
    type Target = PlayerIdentityData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PlayerIdentity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// 2. Conversion helpers (Optional but useful)
impl From<PlayerIdentityData> for PlayerIdentity {
    fn from(data: PlayerIdentityData) -> Self {
        Self(data)
    }
}

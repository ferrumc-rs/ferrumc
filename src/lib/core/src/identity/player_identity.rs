use bevy_ecs::prelude::Component;
use typename::TypeName;

#[derive(TypeName, Debug, Component, Default)]
pub struct PlayerIdentity {
    pub username: String,
    pub uuid: u128,
    pub short_uuid: i32,
}

impl PlayerIdentity {
    pub fn new(username: String, uuid: u128) -> Self {
        Self {
            username,
            uuid,
            short_uuid: uuid as i32,
        }
    }
}

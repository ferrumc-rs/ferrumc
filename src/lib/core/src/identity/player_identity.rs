use bevy_ecs::prelude::Component;
use typename::TypeName;

#[derive(TypeName, Debug, Component, Default, Clone)]
pub struct PlayerIdentity {
    pub username: String,
    pub uuid: uuid::Uuid,
    pub short_uuid: i32,
}

impl PlayerIdentity {
    pub fn new(username: String, uuid: u128) -> Self {
        Self {
            username,
            uuid: uuid::Uuid::from_u128(uuid),
            short_uuid: uuid as i32,
        }
    }
}

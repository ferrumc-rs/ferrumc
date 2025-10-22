use bevy_ecs::{entity::Entity, event::Event};

use crate::{data::player::PlayerData, identity::player_identity::PlayerIdentity};

#[derive(Event)]
pub struct PlayerDisconnectEvent {
    pub entity: Entity,
    pub identity: PlayerIdentity,
    pub data: PlayerData,
}

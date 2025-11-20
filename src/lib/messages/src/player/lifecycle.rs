use bevy_ecs::prelude::{Entity, Event};
use ferrumc_core::player::identity::PlayerIdentityData;

/// Fired by the `new_connection` system when a player joins
///
/// Fired by: `new_connection`.
/// Listened for by: `system_messages` to broadcast join and leave messages
#[derive(Event, Clone)]
#[allow(unused)]
pub struct PlayerJoinEvent(pub PlayerIdentityData);

/// Fired by the `connection_killer` system when a player joins
///
/// Fired by: `connection_killer`.
/// Listened for by: `system_messages` to broadcast join and leave messages
#[derive(Event, Clone)]
#[allow(unused)]
pub struct PlayerLeaveEvent(pub PlayerIdentityData);


#[derive(Event)]
pub struct ConnectionKillEvent {
    pub entity: Entity,
    pub reason: Option<String>,
}

use bevy_ecs::prelude::*;
use ferrumc_core::identity::player_identity::PlayerIdentity;

/// Fired by the `new_connection` system when a player joins
///
/// Fired by: `new_connection`.
/// Listened for by: `system_messages` to broadcast join and leave messages
#[derive(Message, Clone)]
#[allow(unused)]
pub struct PlayerJoined(pub PlayerIdentity);

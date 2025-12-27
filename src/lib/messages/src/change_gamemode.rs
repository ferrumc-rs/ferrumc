use bevy_ecs::prelude::{Entity, Message};
use ferrumc_components::player::gamemode::GameMode;

/// Fired when a player's gamemode should be changed.
/// This can be triggered by a command or a packet.
#[derive(Message)]
pub struct PlayerGameModeChanged {
    pub player: Entity,
    pub new_mode: GameMode,
}

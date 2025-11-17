use bevy_ecs::prelude::{Entity, Event};
use ferrumc_components::player::gamemode::GameMode;

/// Fired when a player's gamemode should be changed.
/// This can be triggered by a command or a packet.
#[derive(Event)]
pub struct ChangeGameModeEvent {
    pub player: Entity,
    pub new_mode: GameMode,
}

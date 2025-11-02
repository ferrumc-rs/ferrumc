use bevy_ecs::prelude::{Component, Entity};

/// Component that tracks which players have already received this entity's spawn packet
///
/// This prevents sending duplicate spawn packets to the same player.
#[derive(Debug, Clone, Component, Default)]
pub struct SyncedToPlayers {
    pub player_entities: Vec<Entity>,
}

use bevy_ecs::prelude::{Entity, Event};

/// Fired when a player gains experience points (e.g., from an orb).
///
/// Fired by: Orb collection system, command handler.
/// Listened for by: An `experience_system` to update the `Experience` component.
#[derive(Event)]
#[allow(unused)]
pub struct PlayerXPGainEvent {
    pub player: Entity,
    pub amount: u32,
}

/// Fired by the `experience_system` when a player's level changes.
///
/// Fired by: `experience_system`.
/// Listened for by: `sound_system` (to play the *ding*), UI systems.
#[derive(Event)]
#[allow(unused)]
pub struct PlayerLevelUpEvent {
    pub player: Entity,
    pub new_level: u32,
}

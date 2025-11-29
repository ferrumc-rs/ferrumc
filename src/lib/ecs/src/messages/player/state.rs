use bevy_ecs::prelude::{Entity, Event};
use ferrumc_core::player::gamemode::GameMode;

/// Fired when a player's gamemode should be changed.
/// This can be triggered by a command or a packet.
#[derive(Event)]
pub struct ChangeGameModeEvent {
    pub player: Entity,
    pub new_mode: GameMode,
}

/// Fired when a player should take damage.
///
/// * Fired by: Physics (fall damage), Hunger System (starvation), Combat.
/// * Listened for by: A `health_system` that will decrease the `Health` component.
#[derive(Event)]
#[allow(unused)]
pub struct PlayerDamageEvent {
    pub player: Entity,
    pub amount: f32,
    // TODO: add a `DamageSource` enum here later
}

/// Fired by the `health_system` when a player's health reaches <= 0.
///
/// * Fired by: `health_system`.
/// * Listened for by: `respawn_system`, `player_leave_system` (to broadcast death msg).
#[derive(Event)]
#[allow(unused)]
pub struct PlayerDeathEvent {
    pub player: Entity,
}

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

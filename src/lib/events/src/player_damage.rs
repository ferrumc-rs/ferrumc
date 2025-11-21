use bevy_ecs::prelude::{Entity, Event};

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

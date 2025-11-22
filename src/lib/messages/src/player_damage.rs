use bevy_ecs::prelude::{Entity, Message};

/// Fired when a player should take damage.
///
/// * Fired by: Physics (fall damage), Hunger System (starvation), Combat.
/// * Listened for by: A `health_system` that will decrease the `Health` component.
#[derive(Message)]
#[allow(unused)]
pub struct PlayerDamaged {
    pub player: Entity,
    pub amount: f32,
    // TODO: add a `DamageSource` enum here later
}

/// Fired by the `health_system` when a player's health reaches <= 0.
///
/// * Fired by: `health_system`.
/// * Listened for by: `respawn_system`, `player_leave_system` (to broadcast death msg).
#[derive(Message)]
#[allow(unused)]
pub struct PlayerDied {
    pub player: Entity,
}

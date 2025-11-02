use bevy_ecs::prelude::*;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_entities::components::*;

const GRAVITY: f64 = -0.08; // Blocks per tick^2
const TERMINAL_VELOCITY: f64 = -3.92; // Max fall speed

/// System that apply basic physics to entity
pub fn entity_physics_system(
    mut query: Query<(&mut Position, &mut Velocity, &OnGround), With<EntityType>>,
) {
    for (mut pos, mut vel, on_ground) in query.iter_mut() {
        // Apply gravity if not on ground
        if !on_ground.0 {
            vel.y = (vel.y + GRAVITY).max(TERMINAL_VELOCITY);
        } else {
            // Reset velocity Y if on ground
            if vel.y < 0.0 {
                vel.y = 0.0;
            }
        }

        pos.x += vel.x;
        pos.y += vel.y;
        pos.z += vel.z;

        if on_ground.0 {
            vel.x *= 0.6;
            vel.z *= 0.6;
        } else {
            vel.x *= 0.98;
            vel.z *= 0.98;
        }
    }
}

pub fn entity_age_system(mut query: Query<&mut Age, With<EntityType>>) {
    for mut age in query.iter_mut() {
        age.tick();
    }
}

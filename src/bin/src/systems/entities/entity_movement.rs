use bevy_ecs::prelude::*;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_entities::collision::{check_collision, BoundingBox};
use ferrumc_entities::components::*;
use ferrumc_state::GlobalStateResource;

const GRAVITY: f64 = -0.08; // Blocks per tick^2
const TERMINAL_VELOCITY: f64 = -3.92; // Max fall speed

/// System that apply basic physics to entity
pub fn entity_physics_system(
    mut query: Query<(&mut Position, &mut Velocity, &OnGround), With<EntityType>>,
    state: Res<GlobalStateResource>,
) {
    // TODO: Make this configurable per entity type
    let bbox = BoundingBox::PIG;

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

        // Try to move in all three axes, checking collision at the final position
        let new_x = pos.x + vel.x;
        let new_y = pos.y + vel.y;
        let new_z = pos.z + vel.z;

        // Check collision at the new position (considering all movement)
        if !check_collision(&state.0, new_x, new_y, new_z, &bbox) {
            // No collision, move freely
            pos.x = new_x;
            pos.y = new_y;
            pos.z = new_z;
        } else {
            // Collision detected, try each axis separately

            // Try Y movement first (jumping/falling)
            if !check_collision(&state.0, pos.x, new_y, pos.z, &bbox) {
                pos.y = new_y;
            } else {
                vel.y = 0.0;
            }

            // Try X movement with updated Y position
            if !check_collision(&state.0, new_x, pos.y, pos.z, &bbox) {
                pos.x = new_x;
            } else {
                vel.x = 0.0;
            }

            // Try Z movement with updated X and Y positions
            if !check_collision(&state.0, pos.x, pos.y, new_z, &bbox) {
                pos.z = new_z;
            } else {
                vel.z = 0.0;
            }
        }

        if on_ground.0 {
            // Less friction on ground for better movement (was 0.6)
            vel.x *= 0.85;
            vel.z *= 0.85;
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

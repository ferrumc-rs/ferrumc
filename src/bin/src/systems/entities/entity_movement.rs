use bevy_ecs::prelude::*;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_entities::collision::{check_collision, is_in_water, BoundingBox};
use ferrumc_entities::components::*;
use ferrumc_state::GlobalStateResource;

const GRAVITY: f64 = -0.08; // Blocks per tick^2
const TERMINAL_VELOCITY: f64 = -3.92; // Max fall speed
const WATER_BUOYANCY: f64 = 0.09; // Upward force in water (slightly stronger than gravity for gentle floating)
const WATER_DRAG: f64 = 0.8; // Water friction multiplier

/// System that apply basic physics to entity
pub fn entity_physics_system(
    mut query: Query<(&mut Position, &mut Velocity, &OnGround), With<EntityType>>,
    state: Res<GlobalStateResource>,
) {
    // TODO: Make this configurable per entity type
    let bbox = BoundingBox::PIG;

    for (mut pos, mut vel, on_ground) in query.iter_mut() {
        // Check if entity is in water
        let in_water = is_in_water(&state.0, pos.x, pos.y, pos.z, &bbox);

        // Apply gravity and buoyancy
        if in_water {
            // In water: buoyancy force is slightly stronger than gravity for gentle floating
            vel.y += GRAVITY + WATER_BUOYANCY;
            // Net force: -0.08 + 0.09 = +0.01 (upward), causing gentle floating
        } else if !on_ground.0 {
            // In air: normal gravity
            vel.y = (vel.y + GRAVITY).max(TERMINAL_VELOCITY);
        } else {
            // On ground: reset downward velocity
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
            *pos = Position::new(new_x, new_y, new_z);
        } else {
            // Collision detected, try each axis separately
            let mut final_x = pos.x;
            let mut final_y = pos.y;
            let mut final_z = pos.z;

            // Try Y movement first (jumping/falling)
            if !check_collision(&state.0, pos.x, new_y, pos.z, &bbox) {
                final_y = new_y;
            } else {
                vel.y = 0.0;
            }

            // Try X movement with updated Y position
            if !check_collision(&state.0, new_x, final_y, pos.z, &bbox) {
                final_x = new_x;
            } else {
                vel.x = 0.0;
            }

            // Try Z movement with updated X and Y positions
            if !check_collision(&state.0, final_x, final_y, new_z, &bbox) {
                final_z = new_z;
            } else {
                vel.z = 0.0;
            }

            *pos = Position::new(final_x, final_y, final_z);
        }

        // Apply friction based on environment
        if in_water {
            // Water drag - slows movement significantly
            vel.x *= WATER_DRAG;
            vel.z *= WATER_DRAG;
            vel.y *= 0.95; // Vertical water drag
        } else if on_ground.0 {
            // Ground friction
            vel.x *= 0.85;
            vel.z *= 0.85;
        } else {
            // Air resistance
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

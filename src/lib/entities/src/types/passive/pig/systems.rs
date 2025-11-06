use bevy_ecs::prelude::*;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_state::GlobalStateResource;
use rand::Rng;

use crate::GameEntity;
use crate::components::Velocity;
use crate::types::passive::pig::PigData;

/// System that ticks pig entities to update their AI/behavior
pub fn pig_tick_system(
    mut pigs: Query<(
        &mut PigData,
        &mut Velocity,
        &mut Rotation,
        &Position,
        &OnGround,
    )>,
    state: Res<GlobalStateResource>,
    mut commands: Commands,
) {
    for (mut pig_data, mut velocity, mut rotation, position, on_ground) in pigs.iter_mut() {
        // Call the entity's tick method for entity-specific behavior
        pig_data.tick(&state.0, &mut commands);

        // Basic AI: Random wandering when on ground
        if on_ground.0 {
            let mut rng = rand::rng();

            // Check for obstacle first - if blocked, try to jump or change direction
            if should_avoid_obstacle(&state.0, position, velocity.x, velocity.z) {
                // 50% chance to try jumping over obstacle, 50% chance to turn around
                if rng.random_bool(0.5) && velocity.y.abs() < 0.01 {
                    velocity.y = 0.42; // Jump to try to get over obstacle
                } else {
                    // Pick a new random direction when hitting a wall
                    let angle = rng.random_range(0.0..std::f64::consts::TAU);
                    velocity.x = angle.cos() * 0.25;
                    velocity.z = angle.sin() * 0.25;
                    rotation.yaw = (-velocity.x.atan2(velocity.z).to_degrees()) as f32;
                }
            } else {
                // Only 1% chance to change direction when not blocked (less rotation)
                if rng.random_bool(0.01) {
                    let angle = rng.random_range(0.0..std::f64::consts::TAU);
                    velocity.x = angle.cos() * 0.25;
                    velocity.z = angle.sin() * 0.25;
                    rotation.yaw = (-velocity.x.atan2(velocity.z).to_degrees()) as f32;
                }
            }
        }
    }
}

/// Check if there's an obstacle in the movement direction
fn should_avoid_obstacle(
    state: &ferrumc_state::GlobalState,
    pos: &Position,
    vel_x: f64,
    vel_z: f64,
) -> bool {
    // Pig hitbox is approximately 0.9 x 0.9 x 0.9 blocks
    // Check multiple points around the entity's bounding box

    let check_distance = 0.6; // Look slightly ahead
    let next_x = pos.x + vel_x.signum() * check_distance;
    let next_z = pos.z + vel_z.signum() * check_distance;

    // Check at feet level and head level (entity is ~0.9 blocks tall)
    let check_positions = [
        // Feet level
        (
            next_x.floor() as i32,
            pos.y.floor() as i32,
            next_z.floor() as i32,
        ),
        // Head level
        (
            next_x.floor() as i32,
            (pos.y + 0.5).floor() as i32,
            next_z.floor() as i32,
        ),
    ];

    for (check_x, check_y, check_z) in check_positions {
        if let Ok(block_state) =
            state
                .world
                .get_block_and_fetch(check_x, check_y, check_z, "overworld")
        {
            // If there's a solid block, obstacle detected
            if block_state.0 != 0 {
                return true;
            }
        }
    }

    false
}

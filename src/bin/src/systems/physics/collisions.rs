use bevy_ecs::prelude::{DetectChanges, Query, Ref, Res};
use bevy_math::{IVec3, Vec3A};
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::velocity::Velocity;
use ferrumc_entities::PhysicalProperties;
use ferrumc_macros::match_block;
use ferrumc_state::{GlobalState, GlobalStateResource};
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::pos::{ChunkBlockPos, ChunkPos};

pub fn handle(
    query: Query<(
        &mut Velocity,
        Ref<Position>,
        &PhysicalProperties,
        &mut OnGround,
    )>,
    state: Res<GlobalStateResource>,
) {
    for (mut vel, pos, physical, mut grounded) in query {
        if pos.is_changed() || vel.is_changed() {
            grounded.0 = false;

            // Clamp Y axis first for proper ground detection
            let clamped_vel_y = clamp_axis_velocity(&state.0, &pos, &physical.bounding_box, vel.y as f64, Axis::Y);
            if clamped_vel_y != vel.y as f64 {
                if vel.y < 0.0 {
                    grounded.0 = true;
                }
                vel.y = clamped_vel_y as f32;
            }

            // Clamp X axis, accounting for Y movement
            let future_y = pos.coords.y + clamped_vel_y;
            let clamped_vel_x = clamp_axis_velocity_with_offset(
                &state.0,
                &pos,
                &physical.bounding_box,
                vel.x as f64,
                Axis::X,
                0.0,
                future_y - pos.coords.y,
                0.0
            );
            if clamped_vel_x != vel.x as f64 {
                vel.x = clamped_vel_x as f32;
            }

            // Clamp Z axis, accounting for Y and X movement
            let future_x = pos.coords.x + clamped_vel_x;
            let clamped_vel_z = clamp_axis_velocity_with_offset(
                &state.0,
                &pos,
                &physical.bounding_box,
                vel.z as f64,
                Axis::Z,
                future_x - pos.coords.x,
                future_y - pos.coords.y,
                0.0
            );
            if clamped_vel_z != vel.z as f64 {
                vel.z = clamped_vel_z as f32;
            }
        }
    }
}

enum Axis {
    X,
    Y,
    Z,
}

/// Clamp velocity on a single axis to prevent collision.
/// offset_x/y/z represent movement applied on other axes before this one.
fn clamp_axis_velocity_with_offset(
    state: &GlobalState,
    pos: &Ref<Position>,
    bounding_box: &bevy_math::bounding::Aabb3d,
    mut velocity: f64,
    axis: Axis,
    offset_x: f64,
    offset_y: f64,
    offset_z: f64,
) -> f64 {
    if velocity.abs() < 0.0001 {
        return velocity;
    }

    let current_pos = Vec3A::new(
        (pos.coords.x + offset_x) as f32,
        (pos.coords.y + offset_y) as f32,
        (pos.coords.z + offset_z) as f32,
    );

    let min = bounding_box.min + current_pos;
    let max = bounding_box.max + current_pos;

    let (check_min, check_max) = match axis {
        Axis::X => {
            let x_min = if velocity > 0.0 { min.x } else { min.x + velocity as f32 };
            let x_max = if velocity > 0.0 { max.x + velocity as f32 } else { max.x };
            (
                Vec3A::new(x_min, min.y, min.z),
                Vec3A::new(x_max, max.y, max.z),
            )
        }
        Axis::Y => {
            let y_min = if velocity > 0.0 { min.y } else { min.y + velocity as f32 };
            let y_max = if velocity > 0.0 { max.y + velocity as f32 } else { max.y };
            (
                Vec3A::new(min.x, y_min, min.z),
                Vec3A::new(max.x, y_max, max.z),
            )
        }
        Axis::Z => {
            let z_min = if velocity > 0.0 { min.z } else { min.z + velocity as f32 };
            let z_max = if velocity > 0.0 { max.z + velocity as f32 } else { max.z };
            (
                Vec3A::new(min.x, min.y, z_min),
                Vec3A::new(max.x, max.y, z_max),
            )
        }
    };

    for x in check_min.x.floor() as i32..=check_max.x.floor() as i32 {
        for y in check_min.y.floor() as i32..=check_max.y.floor() as i32 {
            for z in check_min.z.floor() as i32..=check_max.z.floor() as i32 {
                let block_pos = IVec3::new(x, y, z);
                if !is_solid_block(state, block_pos) {
                    continue;
                }

                velocity = match axis {
                    Axis::X => {
                        if velocity > 0.0 {
                            let block_left = x as f64;
                            let entity_right = max.x as f64;
                            velocity.min(block_left - entity_right)
                        } else {
                            let block_right = (x + 1) as f64;
                            let entity_left = min.x as f64;
                            velocity.max(block_right - entity_left)
                        }
                    }
                    Axis::Y => {
                        if velocity > 0.0 {
                            let block_bottom = y as f64;
                            let entity_top = max.y as f64;
                            velocity.min(block_bottom - entity_top)
                        } else {
                            let block_top = (y + 1) as f64;
                            let entity_bottom = min.y as f64;
                            velocity.max(block_top - entity_bottom)
                        }
                    }
                    Axis::Z => {
                        if velocity > 0.0 {
                            let block_near = z as f64;
                            let entity_far = max.z as f64;
                            velocity.min(block_near - entity_far)
                        } else {
                            let block_far = (z + 1) as f64;
                            let entity_near = min.z as f64;
                            velocity.max(block_far - entity_near)
                        }
                    }
                };

                if velocity.abs() < 0.0001 {
                    return 0.0;
                }
            }
        }
    }

    velocity
}

fn clamp_axis_velocity(
    state: &GlobalState,
    pos: &Ref<Position>,
    bounding_box: &bevy_math::bounding::Aabb3d,
    velocity: f64,
    axis: Axis,
) -> f64 {
    clamp_axis_velocity_with_offset(state, pos, bounding_box, velocity, axis, 0.0, 0.0, 0.0)
}

#[allow(dead_code)]
fn check_collision(state: &GlobalState, pos: Vec3A, bounding_box: &bevy_math::bounding::Aabb3d) -> bool {
    let min_block_pos = bounding_box.min + pos;
    let max_block_pos = bounding_box.max + pos;

    for x in min_block_pos.x.floor() as i32..=max_block_pos.x.floor() as i32 {
        for y in min_block_pos.y.floor() as i32..=max_block_pos.y.floor() as i32 {
            for z in min_block_pos.z.floor() as i32..=max_block_pos.z.floor() as i32 {
                let block_pos = IVec3::new(x, y, z);
                if is_solid_block(state, block_pos) {
                    return true;
                }
            }
        }
    }
    false
}

pub fn is_solid_block(state: &GlobalState, pos: IVec3) -> bool {
    state
        .world
        .load_chunk(ChunkPos::from(pos.as_dvec3()), "overworld")
        .unwrap_or(
            state
                .terrain_generator
                .generate_chunk(ChunkPos::from(pos.as_dvec3()))
                .expect("Failed to generate chunk")
                .into(),
        )
        .get_block(ChunkBlockPos::from(pos))
        .map(|block_state| {
            !match_block!("air", block_state)
                && !match_block!("void_air", block_state)
                && !match_block!("cave_air", block_state)
        })
        .unwrap_or(false)
}

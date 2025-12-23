use bevy_ecs::message::MessageWriter;
use bevy_ecs::prelude::{DetectChanges, Entity, Query, Res};
use bevy_math::bounding::{Aabb3d, BoundingVolume};
use bevy_math::{IVec3, Vec3A};
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::velocity::Velocity;
use ferrumc_entities::PhysicalProperties;
use ferrumc_macros::match_block;
use ferrumc_messages::entity_update::SendEntityUpdate;
use ferrumc_state::{GlobalState, GlobalStateResource};
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::pos::{ChunkBlockPos, ChunkPos};
use tracing::debug;

pub fn handle(
    query: Query<(
        Entity,
        &mut Velocity,
        &mut Position,
        &PhysicalProperties,
        &mut OnGround,
    )>,
    mut writer: MessageWriter<SendEntityUpdate>,
    state: Res<GlobalStateResource>,
) {
    for (eid, mut vel, mut pos, physical, mut grounded) in query {
        if pos.is_changed() || vel.is_changed() {
            // Figure out where the entity is going to be next tick
            let next_pos = pos.coords.as_vec3a() + **vel;
            let mut collided = false;
            let mut hit_blocks = vec![];

            // Merge the current and next bounding boxes to get the full area the entity will occupy
            // This helps catch fast-moving entities that might skip through thin blocks
            // At really high speeds this will create a very large bounding box, so further optimizations may be needed
            let current_hitbox = Aabb3d {
                min: physical.bounding_box.min + pos.coords.as_vec3a(),
                max: physical.bounding_box.max + pos.coords.as_vec3a(),
            };

            let next_hitbox = Aabb3d {
                min: physical.bounding_box.min + next_pos,
                max: physical.bounding_box.max + next_pos,
            };

            let merged_hitbox = current_hitbox.merge(&next_hitbox);

            // Get the block positions that the entity's bounding box will occupy
            let min_block_pos = merged_hitbox.min;
            let max_block_pos = merged_hitbox.max;

            // Check each block in the bounding box for solidity
            for x in min_block_pos.x.floor() as i32..=max_block_pos.x.floor() as i32 {
                for y in min_block_pos.y.floor() as i32..=max_block_pos.y.floor() as i32 {
                    for z in min_block_pos.z.floor() as i32..=max_block_pos.z.floor() as i32 {
                        let block_pos = IVec3::new(x, y, z);
                        if is_solid_block(&state.0, block_pos) {
                            collided = true;
                            hit_blocks.push(block_pos);
                            if is_solid_block(&state.0, IVec3::new(x, y - 1, z)) && vel.y <= 0.0 {
                                grounded.0 = true;
                            }
                        }
                    }
                }
            }
            // If a collision is detected, stop the entity's movement
            if collided {
                vel.vec = Vec3A::ZERO;
                // Find the closest hit block to the entity's next position
                hit_blocks.sort_by(|a, b| {
                    let dist_a = (a.as_vec3a() - next_pos).length_squared();
                    let dist_b = (b.as_vec3a() - next_pos).length_squared();
                    dist_a.partial_cmp(&dist_b).unwrap()
                });
                let first_hit = hit_blocks.first().expect("At least one hit block expected");
                debug!(
                    "Entity collided at block position: {:?} going {}",
                    &hit_blocks, vel.vec
                );

                let block_aabb = Aabb3d {
                    min: first_hit.as_vec3a(),
                    max: (first_hit + IVec3::ONE).as_vec3a(),
                };

                let translated_bounding_box = Aabb3d {
                    min: physical.bounding_box.min + pos.coords.as_vec3a(),
                    max: physical.bounding_box.max + pos.coords.as_vec3a(),
                };

                // Get the closest point on the entity's bounding box to the block's AABB
                let entity_collide_point = translated_bounding_box
                    .closest_point(block_aabb.center().as_dvec3().as_vec3a());

                if entity_collide_point == block_aabb.center().as_dvec3().as_vec3a() {
                    continue;
                }

                // Then we get the closest point on the block's AABB to the entity's collide point
                let block_collide_point = block_aabb.closest_point(entity_collide_point);

                if block_collide_point == entity_collide_point {
                    continue;
                }

                // The difference between these two points tells us how far apart the 2 colliding objects are
                let collision_difference = entity_collide_point - block_collide_point;

                // We use this to nudge the entity out of the block along the smallest axis
                pos.coords -= collision_difference.as_dvec3();
            }

            writer.write(SendEntityUpdate(eid));
        }
    }
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

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
use tracing::debug;

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
            // Figure out where the entity is going to be next tick
            let next_pos = (pos.coords + vel.as_dvec3()).as_vec3a();
            let mut collided = false;
            let mut hit_block = IVec3::ZERO;

            // Get the block positions that the entity's bounding box will occupy
            let min_block_pos = physical.bounding_box.min + next_pos;
            let max_block_pos = physical.bounding_box.max + next_pos;

            // Check each block in the bounding box for solidity
            for x in min_block_pos.x.floor() as i32..=max_block_pos.x.floor() as i32 {
                for y in min_block_pos.y.floor() as i32..=max_block_pos.y.floor() as i32 {
                    for z in min_block_pos.z.floor() as i32..=max_block_pos.z.floor() as i32 {
                        let block_pos = IVec3::new(x, y, z);
                        if is_solid_block(&state.0, block_pos) {
                            collided = true;
                            hit_block = block_pos;
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
                debug!("Entity collided at block position: {:?}", debug(&hit_block));

                //TODO: Clip the entity to the nearest non-colliding position
            }
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

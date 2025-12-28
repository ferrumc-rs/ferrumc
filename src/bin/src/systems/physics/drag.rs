use bevy_ecs::prelude::{DetectChanges, Query, Res, With};
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::velocity::Velocity;
use ferrumc_entities::markers::HasWaterDrag;
use ferrumc_macros::match_block;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::pos::{ChunkBlockPos, ChunkPos};

pub fn handle(
    mut query: Query<(&mut Velocity, &mut Position), With<HasWaterDrag>>,
    state: Res<GlobalStateResource>,
) {
    for (mut vel, pos) in query.iter_mut() {
        if pos.is_changed() || vel.is_changed() {
            let chunk_pos = ChunkPos::from(pos.coords);
            let chunk = state.0.world.load_chunk(chunk_pos, "overworld").unwrap_or({
                state
                    .0
                    .world
                    .insert_chunk(
                        chunk_pos,
                        "overworld",
                        state
                            .0
                            .terrain_generator
                            .generate_chunk(chunk_pos)
                            .expect("Could not generate chunk"),
                    )
                    .expect("Failed to save generated chunk");
                state
                    .0
                    .world
                    .load_chunk(chunk_pos, "overworld")
                    .expect("Failed to load newly generated chunk")
            });
            let is_in_water = chunk
                .get_block(ChunkBlockPos::from(pos.coords.as_ivec3()))
                .map(|block| match_block!("water", block))
                .unwrap_or(false);
            if is_in_water {
                vel.y *= 0.98;
                vel.x *= 0.91;
                vel.z *= 0.91;
            }
        }
    }
}

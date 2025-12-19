use bevy_ecs::prelude::{DetectChanges, Query, Ref, Res};
use bevy_math::IVec3;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::velocity::Velocity;
use ferrumc_entities::PhysicalProperties;
use ferrumc_macros::match_block;
use ferrumc_state::{GlobalState, GlobalStateResource};
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::pos::{ChunkBlockPos, ChunkPos};

pub fn handle(
    query: Query<(&mut Velocity, Ref<Position>, &PhysicalProperties)>,
    state: Res<GlobalStateResource>,
) {
    for (mut vel, pos, physical) in query {
        // If velocity and position haven't changed, skip collision checks
        if !vel.is_changed() && !pos.is_changed() {
            continue;
        }
        let aabb = physical.bounding_box;
        let (min, max) = (aabb.min.as_dvec3(), aabb.max.as_dvec3());

        let to_block = |x: f64, y: f64, z: f64| {
            IVec3::new(x.floor() as i32, y.floor() as i32, z.floor() as i32)
        };

        let check_positions = [
            // Feet level
            to_block(pos.x + min.x, pos.y + min.y, pos.z + min.z),
            to_block(pos.x + max.x, pos.y + min.y, pos.z + min.z),
            to_block(pos.x + min.x, pos.y + min.y, pos.z + max.z),
            to_block(pos.x + max.x, pos.y + min.y, pos.z + max.z),
            // Head level
            to_block(pos.x + min.x, pos.y + max.y, pos.z + min.z),
            to_block(pos.x + max.x, pos.y + max.y, pos.z + min.z),
            to_block(pos.x + min.x, pos.y + max.y, pos.z + max.z),
            to_block(pos.x + max.x, pos.y + max.y, pos.z + max.z),
        ];

        if check_positions
            .iter()
            .any(|pos| is_solid_block(&state.0, *pos))
        {
            vel.y = 0.0;
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

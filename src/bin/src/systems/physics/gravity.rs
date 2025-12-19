use bevy_ecs::prelude::{Query, Res, With};
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::velocity::Velocity;
use ferrumc_entities::markers::HasGravity;
use ferrumc_macros::block;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::pos::{ChunkBlockPos, ChunkPos};

// Just apply gravity to a mob's velocity. Application of velocity is handled elsewhere.
pub(crate) fn handle(
    mut entities: Query<(&mut Velocity, &Position, &OnGround), With<HasGravity>>,
    state: Res<GlobalStateResource>,
) {
    for (mut vel, pos, grounded) in entities.iter_mut() {
        if grounded.0 {
            continue;
        }
        let int_pos = pos.floor().as_ivec3();
        if state
            .0
            .world
            .load_chunk(ChunkPos::from(int_pos), "overworld")
            .unwrap_or(
                state
                    .0
                    .terrain_generator
                    .generate_chunk(ChunkPos::from(int_pos))
                    .expect("Failed to generate chunk")
                    .into(),
            )
            .get_block(ChunkBlockPos::from(int_pos))
            .is_ok_and(|block| {
                !(block == block!("air")
                    || block == block!("void_air")
                    || block == block!("cave_air"))
            })
        {
            **vel -= ferrumc_physics::GRAVITY_ACCELERATION.as_dvec3();
            vel.y = vel.y.max(ferrumc_physics::TERMINAL_VELOCITY_Y);
        }
    }
}

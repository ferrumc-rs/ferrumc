use bevy_ecs::prelude::{Query, Res, With};
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::velocity::Velocity;
use ferrumc_entities::markers::HasGravity;
use ferrumc_macros::block;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_state_id::BlockStateId;

// Just apply gravity to a mob's velocity. Application of velocity is handled elsewhere.
fn handle(
    mut entities: Query<(&mut Velocity, &Position), With<HasGravity>>,
    state: Res<GlobalStateResource>,
) {
    for (mut vel, pos) in entities.iter_mut() {
        let int_pos = pos.floor().as_ivec3();
        if state
            .0
            .world
            .get_block_and_fetch(int_pos.x, int_pos.y - 1, int_pos.z, "overworld")
            .is_ok_and(|block| {
                !(block == block!("air")
                    || block == block!("void_air")
                    || block == block!("cave_air"))
            })
        {
            **vel - ferrumc_physics::GRAVITY_ACCELERATION.as_dvec3();
            vel.y = (**vel).y.max(ferrumc_physics::TERMINAL_VELOCITY_Y);
        }
    }
}

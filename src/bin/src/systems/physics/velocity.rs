use bevy_ecs::prelude::Query;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::velocity::Velocity;

pub fn handle(mut query: Query<(&Velocity, &mut Position)>) {
    for (vel, mut pos) in query.iter_mut() {
        // TODO: Collision detection should be handled here
        pos.coords += **vel;
    }
}

use bevy_ecs::prelude::{Query, With};
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::velocity::Velocity;
use ferrumc_entities::markers::HasGravity;
use ferrumc_entities::PhysicalProperties;
use ferrumc_physics::GRAVITY_ACCELERATION;

// Just apply gravity to a mob's velocity. Application of velocity is handled elsewhere.
pub(crate) fn handle(
    mut entities: Query<
        (&mut Velocity, &mut Position, &OnGround, &PhysicalProperties),
        With<HasGravity>,
    >,
) {
    for (mut vel, mut pos, grounded, physical) in entities.iter_mut() {
        if grounded.0 {
            continue;
        }
        // Apply gravity
        vel.vec += GRAVITY_ACCELERATION;
    }
}

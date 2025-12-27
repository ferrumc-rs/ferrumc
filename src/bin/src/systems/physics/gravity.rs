use bevy_ecs::prelude::{Query, With};
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::velocity::Velocity;
use ferrumc_entities::markers::HasGravity;
use ferrumc_physics::GRAVITY_ACCELERATION;

// Just apply gravity to a mob's velocity. Application of velocity is handled elsewhere.
pub(crate) fn handle(mut entities: Query<(&mut Velocity, &OnGround), With<HasGravity>>) {
    for (mut vel, grounded) in entities.iter_mut() {
        if grounded.0 {
            continue;
        }
        // Apply gravity
        vel.vec += GRAVITY_ACCELERATION;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::prelude::*;
    use bevy_math::Vec3A;
    use ferrumc_core::transform::grounded::OnGround;
    use ferrumc_core::transform::velocity::Velocity;
    use ferrumc_entities::markers::HasGravity;

    #[test]
    fn test_gravity_application() {
        let mut world = World::new();
        let entity = world
            .spawn((Velocity { vec: Vec3A::ZERO }, OnGround(false), HasGravity))
            .id();

        let mut schedule = Schedule::default();
        schedule.add_systems(handle);

        // Run the gravity system
        schedule.run(&mut world);

        let vel = world.get::<Velocity>(entity).unwrap();
        assert!(
            vel.vec.y < 0.0,
            "Velocity Y should be negative after gravity application"
        );
    }

    #[test]
    fn test_no_gravity_when_grounded() {
        let mut world = World::new();
        let entity = world
            .spawn((Velocity { vec: Vec3A::ZERO }, OnGround(true), HasGravity))
            .id();

        let mut schedule = Schedule::default();
        schedule.add_systems(handle);

        // Run the gravity system
        schedule.run(&mut world);

        let vel = world.get::<Velocity>(entity).unwrap();
        assert_eq!(
            vel.vec.y, 0.0,
            "Velocity Y should remain zero when grounded"
        );
    }
}

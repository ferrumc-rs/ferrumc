use bevy_ecs::prelude::{Query, Res, With, Without};
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::velocity::Velocity;
use ferrumc_entities::markers::{HasGravity, HasWaterDrag};
use ferrumc_macros::match_block;
use ferrumc_physics::GRAVITY_ACCELERATION;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::pos::{ChunkBlockPos, ChunkPos};
use ferrumc_world::block_state_id::BlockStateId;

// Just apply gravity to a mob's velocity. Application of velocity is handled elsewhere.
pub(crate) fn handle(
    mut entities: Query<(&mut Velocity, &OnGround, &Position), (With<HasGravity>, Without<HasWaterDrag>)>,
    mut water_entities: Query<(&mut Velocity, &OnGround, &Position), (With<HasGravity>, With<HasWaterDrag>)>,
    state: Res<GlobalStateResource>,
) {
    // Apply full gravity to non-water entities
    for (mut vel, grounded, _) in entities.iter_mut() {
        if grounded.0 {
            continue;
        }
        // Apply gravity
        vel.vec += GRAVITY_ACCELERATION;
    }

    // For water entities, only apply gravity if NOT in water
    // If in water, the drag system will handle the reduced gravity
    for (mut vel, grounded, pos) in water_entities.iter_mut() {
        if grounded.0 {
            continue;
        }

        let chunk_pos = ChunkPos::from(pos.coords);
        let chunk =
            ferrumc_utils::world::load_or_generate_mut(&state.0, chunk_pos, "overworld")
               .expect("Failed to load or generate chunk");

        let feet_pos = pos.coords.as_ivec3();

        let is_in_water = match_block!(
            "water",
            chunk.get_block(ChunkBlockPos::from(feet_pos))
        );

        // Only apply full gravity if NOT in water
        if !is_in_water {
            vel.vec += GRAVITY_ACCELERATION;
        }
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

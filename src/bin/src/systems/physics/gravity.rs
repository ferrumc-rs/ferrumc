use bevy_ecs::prelude::{Has, Query, Res, With};
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::velocity::Velocity;
use ferrumc_entities::markers::{HasGravity, HasWaterDrag};
use ferrumc_macros::match_block;
use ferrumc_physics::GRAVITY_ACCELERATION;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::pos::{ChunkBlockPos, ChunkPos};

type EntityQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static mut Velocity,
        &'static OnGround,
        &'static Position,
        Has<HasWaterDrag>,
    ),
    With<HasGravity>,
>;

// Just apply gravity to a mob's velocity. Application of velocity is handled elsewhere.
pub(crate) fn handle(mut entities: EntityQuery, state: Res<GlobalStateResource>) {
    for (mut vel, grounded, pos, is_water) in entities.iter_mut() {
        if grounded.0 {
            continue;
        }

        if is_water {
            let chunk_pos = ChunkPos::from(pos.coords);
            let chunk =
                ferrumc_utils::world::load_or_generate_mut(&state.0, chunk_pos, "overworld")
                    .expect("Failed to load or generate chunk");

            let feet_pos = pos.coords.as_ivec3();

            let is_in_water = match_block!("water", chunk.get_block(ChunkBlockPos::from(feet_pos)));

            // Only apply full gravity if NOT in water
            if !is_in_water {
                vel.vec += GRAVITY_ACCELERATION;
            }
        } else {
            // Apply gravity
            vel.vec += GRAVITY_ACCELERATION;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::prelude::*;
    use bevy_math::DVec3;
    use bevy_math::Vec3A;
    use ferrumc_core::transform::grounded::OnGround;
    use ferrumc_core::transform::velocity::Velocity;
    use ferrumc_entities::markers::HasGravity;
    use ferrumc_macros::block;
    use ferrumc_state::create_test_state;

    /// Creates a chunk with water blocks at the specified positions
    /// This helper function is used to set up test scenarios where entities are in water
    fn create_chunk_with_water(state: &GlobalStateResource, chunk_pos: ChunkPos) {
        // Load or generate the chunk
        let mut chunk =
            ferrumc_utils::world::load_or_generate_mut(&state.0, chunk_pos, "overworld")
                .expect("Failed to load or generate chunk");

        chunk.fill(block!("water", { level: 0 }));
    }

    #[test]
    fn test_gravity_application() {
        let mut world = World::new();
        let (state, _temp_dir) = create_test_state();
        world.insert_resource(state);

        let entity = world
            .spawn((
                Velocity { vec: Vec3A::ZERO },
                OnGround(false),
                Position {
                    coords: DVec3::new(0.0, 100.0, 0.0),
                },
                HasGravity,
            ))
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
        let (state, _temp_dir) = create_test_state();
        world.insert_resource(state);

        let entity = world
            .spawn((
                Velocity { vec: Vec3A::ZERO },
                OnGround(true),
                Position {
                    coords: DVec3::new(0.0, 100.0, 0.0),
                },
                HasGravity,
            ))
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

    #[test]
    fn test_water_entity_gravity_not_in_water() {
        let mut world = World::new();
        let (state, _temp_dir) = create_test_state();
        world.insert_resource(state);

        let entity = world
            .spawn((
                Velocity { vec: Vec3A::ZERO },
                OnGround(false),
                Position {
                    coords: DVec3::new(0.0, 100.0, 0.0),
                },
                HasGravity,
                HasWaterDrag,
            ))
            .id();

        let mut schedule = Schedule::default();
        schedule.add_systems(handle);

        // Run the gravity system
        schedule.run(&mut world);

        let vel = world.get::<Velocity>(entity).unwrap();
        assert!(
            vel.vec.y < 0.0,
            "Water entity should have gravity applied when not in water"
        );
    }

    #[test]
    fn test_water_entity_no_gravity_when_grounded() {
        let mut world = World::new();
        let (state, _temp_dir) = create_test_state();
        world.insert_resource(state);

        let entity = world
            .spawn((
                Velocity { vec: Vec3A::ZERO },
                OnGround(true),
                Position {
                    coords: DVec3::new(0.0, 100.0, 0.0),
                },
                HasGravity,
                HasWaterDrag,
            ))
            .id();

        let mut schedule = Schedule::default();
        schedule.add_systems(handle);

        // Run the gravity system
        schedule.run(&mut world);

        let vel = world.get::<Velocity>(entity).unwrap();
        assert_eq!(
            vel.vec.y, 0.0,
            "Water entity should not have gravity when grounded"
        );
    }

    #[test]
    fn test_water_entity_in_water_no_gravity() {
        let mut world = World::new();
        let (state, _temp_dir) = create_test_state();

        // Create a chunk with water blocks
        let chunk_pos = ChunkPos::new(0, 0);
        create_chunk_with_water(&state, chunk_pos);

        world.insert_resource(state);

        // Spawn entity at Y=65 (in water)
        let entity = world
            .spawn((
                Velocity { vec: Vec3A::ZERO },
                OnGround(false),
                Position {
                    coords: DVec3::new(0.0, 65.0, 0.0),
                },
                HasGravity,
                HasWaterDrag,
            ))
            .id();

        let mut schedule = Schedule::default();
        schedule.add_systems(handle);

        // Run the gravity system
        schedule.run(&mut world);

        let vel = world.get::<Velocity>(entity).unwrap();
        assert_eq!(
            vel.vec.y, 0.0,
            "Water entity should not have gravity applied when in water (drag system handles it)"
        );
    }
}

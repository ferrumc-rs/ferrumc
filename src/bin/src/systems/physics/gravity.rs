use bevy_ecs::prelude::{Query, Res, With, Without};
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::velocity::Velocity;
use ferrumc_entities::markers::{HasGravity, HasWaterDrag};
use ferrumc_macros::match_block;
use ferrumc_physics::GRAVITY_ACCELERATION;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::pos::{ChunkBlockPos, ChunkPos};

type RegularEntityQuery<'w, 's> = Query<
    'w,
    's,
    (&'static mut Velocity, &'static OnGround, &'static Position),
    (With<HasGravity>, Without<HasWaterDrag>),
>;

type WaterEntityQuery<'w, 's> = Query<
    'w,
    's,
    (&'static mut Velocity, &'static OnGround, &'static Position),
    (With<HasGravity>, With<HasWaterDrag>),
>;

// Just apply gravity to a mob's velocity. Application of velocity is handled elsewhere.
pub(crate) fn handle(
    mut entities: RegularEntityQuery,
    mut water_entities: WaterEntityQuery,
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
        let chunk = ferrumc_utils::world::load_or_generate_mut(&state.0, chunk_pos, "overworld")
            .expect("Failed to load or generate chunk");

        let feet_pos = pos.coords.as_ivec3();

        let is_in_water = match_block!("water", chunk.get_block(ChunkBlockPos::from(feet_pos)));

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
    use bevy_math::DVec3;
    use bevy_math::Vec3A;
    use ferrumc_core::transform::grounded::OnGround;
    use ferrumc_core::transform::velocity::Velocity;
    use ferrumc_entities::markers::HasGravity;
    use ferrumc_macros::block;
    use ferrumc_state::player_cache::PlayerCache;
    use ferrumc_state::player_list::PlayerList;
    use ferrumc_state::ServerState;
    use ferrumc_threadpool::ThreadPool;
    use ferrumc_world::pos::ChunkBlockPos;
    use ferrumc_world::World as FerrumcWorld;
    use ferrumc_world_gen::WorldGenerator;
    use std::sync::Arc;
    use std::time::Instant;
    use tempfile::TempDir;

    /// Creates a minimal GlobalStateResource for testing with a temporary database
    fn create_test_state() -> (GlobalStateResource, TempDir) {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let db_path = temp_dir.path().to_path_buf();

        let server_state = ServerState {
            world: FerrumcWorld::new(&db_path),
            terrain_generator: WorldGenerator::new(0),
            shut_down: false.into(),
            players: PlayerList::default(),
            player_cache: PlayerCache::default(),
            thread_pool: ThreadPool::new(),
            start_time: Instant::now(),
        };

        let global_state = Arc::new(server_state);
        (GlobalStateResource(global_state), temp_dir)
    }

    /// Creates a chunk with water blocks at the specified positions
    /// This helper function is used to set up test scenarios where entities are in water
    fn create_chunk_with_water(
        state: &GlobalStateResource,
        chunk_pos: ChunkPos,
        water_y_range: std::ops::Range<i16>,
    ) {
        // Load or generate the chunk
        let mut chunk =
            ferrumc_utils::world::load_or_generate_mut(&state.0, chunk_pos, "overworld")
                .expect("Failed to load or generate chunk");

        // Fill the specified Y range with water blocks (level=0 is full water)
        for x in 0..16u8 {
            for z in 0..16u8 {
                for y in water_y_range.clone() {
                    chunk.set_block(ChunkBlockPos::new(x, y, z), block!("water", { level: 0 }));
                }
            }
        }
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

        // Create a chunk with water blocks at Y levels 60-70
        let chunk_pos = ChunkPos::new(0, 0);
        create_chunk_with_water(&state, chunk_pos, 60..70);

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

use bevy_ecs::prelude::Query;
use bevy_math::Vec3A;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::velocity::Velocity;

pub fn handle(mut query: Query<(&Velocity, &mut Position)>) {
    for (vel, mut pos) in query.iter_mut() {
        if **vel == Vec3A::ZERO {
            continue;
        }
        pos.coords += vel.as_dvec3();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::message::MessageRegistry;
    use bevy_ecs::prelude::*;
    use bevy_math::Vec3A;
    use ferrumc_core::transform::position::Position;
    use ferrumc_core::transform::velocity::Velocity;
    use ferrumc_messages::entity_update::SendEntityUpdate;

    #[test]
    fn test_velocity_updates_position() {
        let mut world = World::new();
        let entity = world
            .spawn((
                Velocity {
                    vec: Vec3A::new(1.0, 2.0, 3.0),
                },
                Position {
                    coords: Vec3A::ZERO.as_dvec3(),
                },
            ))
            .id();
        MessageRegistry::register_message::<SendEntityUpdate>(&mut world);

        let mut schedule = Schedule::default();
        schedule.add_systems(handle);

        // Run the velocity system
        schedule.run(&mut world);

        let pos = world.get::<Position>(entity).unwrap();
        assert_eq!(
            pos.coords,
            Vec3A::new(1.0, 2.0, 3.0).as_dvec3(),
            "Position should be updated based on velocity"
        );
    }

    #[test]
    fn test_no_update_when_unchanged() {
        let mut world = World::new();
        let entity = world
            .spawn((
                Velocity { vec: Vec3A::ZERO },
                Position {
                    coords: Vec3A::ZERO.as_dvec3(),
                },
            ))
            .id();

        MessageRegistry::register_message::<SendEntityUpdate>(&mut world);

        let mut schedule = Schedule::default();
        schedule.add_systems(handle);

        // Run the velocity system
        schedule.run(&mut world);

        assert!(
            world.get::<Position>(entity).is_some(),
            "Entity should exist"
        );

        assert_eq!(
            world.get::<Position>(entity).unwrap().coords,
            Vec3A::ZERO.as_dvec3(),
            "Position should remain unchanged"
        );

        let reader = world.get_resource::<Messages<SendEntityUpdate>>().unwrap();
        let mut cursor = reader.get_cursor();
        let mut messages = vec![];
        for msg in cursor.read(reader) {
            messages.push(msg);
        }
        assert_eq!(
            messages.len(),
            0,
            "No SendEntityUpdate message should be sent when unchanged"
        );
    }

    #[test]
    fn test_multiple_velocity_steps() {
        let mut world = World::new();
        let entity = world
            .spawn((
                Velocity {
                    vec: Vec3A::new(0.5, 0.0, 0.0),
                },
                Position {
                    coords: Vec3A::ZERO.as_dvec3(),
                },
            ))
            .id();

        let mut schedule = Schedule::default();
        schedule.add_systems(handle);

        // Run the velocity system multiple times
        for _ in 0..4 {
            schedule.run(&mut world);
        }

        let pos = world.get::<Position>(entity).unwrap();
        assert_eq!(
            pos.coords,
            Vec3A::new(2.0, 0.0, 0.0).as_dvec3(),
            "Position should be updated correctly after multiple steps"
        );
    }

    #[test]
    fn test_multiple_entities() {
        let mut world = World::new();
        let entity1 = world
            .spawn((
                Velocity {
                    vec: Vec3A::new(1.0, 0.0, 0.0),
                },
                Position {
                    coords: Vec3A::ZERO.as_dvec3(),
                },
            ))
            .id();
        let entity2 = world
            .spawn((
                Velocity {
                    vec: Vec3A::new(0.0, 1.0, 0.0),
                },
                Position {
                    coords: Vec3A::ZERO.as_dvec3(),
                },
            ))
            .id();

        let mut schedule = Schedule::default();
        schedule.add_systems(handle);

        // Run the velocity system
        schedule.run(&mut world);

        let pos1 = world.get::<Position>(entity1).unwrap();
        let pos2 = world.get::<Position>(entity2).unwrap();
        assert_eq!(
            pos1.coords,
            Vec3A::new(1.0, 0.0, 0.0).as_dvec3(),
            "Entity 1 position should be updated correctly"
        );
        assert_eq!(
            pos2.coords,
            Vec3A::new(0.0, 1.0, 0.0).as_dvec3(),
            "Entity 2 position should be updated correctly"
        );
    }
}

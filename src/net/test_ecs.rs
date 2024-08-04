mod tests {
    #[allow(unused_imports)]
    use crate::{
        ecs::world::World, utils::encoding::position::Position, utils::encoding::velocity::Velocity,
    };

    #[tokio::test]
    async fn test_ecs() {
        let mut world = World::new();

        let entity1 = world
            .create_entity()
            .with(Position::new(0, 1, 0))
            .with(Velocity::new(1, 1, 1))
            .build();

        world
            .create_entity()
            .with(Position::new(1, 2, 1))
            .with(Velocity::new(2, 2, 2));

        world.create_entity().with(Position::new(2, 3, 2));

        // Query example
        for (entity_id, (mut position, velocity)) in
            world.query::<(&mut Position, &Velocity)>().iter().await
        {
            println!(
                "[Entity: {}] Adding {:?} to {:?}",
                entity_id, velocity, position
            );
            position.x += velocity.x;
            position.y += velocity.y;
            position.z += velocity.z;
        }

        println!("{}", "*".repeat(50));

        // Log all the entities with position
        for (entity_id, position) in world.query::<&Position>().iter().await {
            println!("[Entity: {}] Position: {:?}", entity_id, position);
        }

        // delete entity
        world
            .delete_entity(entity1)
            .expect("Failed to delete entity1");
    }
}

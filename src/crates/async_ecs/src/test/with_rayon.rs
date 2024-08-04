#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rayon::prelude::*;
    use tokio::runtime::Runtime;
    use tokio::sync::RwLock;

    use crate::component::{DynamicComponent, Position, Velocity};
    use crate::world::World;

    // New components for our test scenario
    #[derive(Debug, Clone)]
    struct Health {
        current: f32,
        max: f32,
    }
    impl DynamicComponent for Health {}

    #[derive(Debug, Clone)]
    struct DamageOverTime {
        damage_per_second: f32,
    }
    impl DynamicComponent for DamageOverTime {}

    #[derive(Debug, Clone)]
    struct Healer {
        heal_per_second: f32,
    }
    impl DynamicComponent for Healer {}

    // Helper function to create a large number of entities
    async fn create_test_entities(world: Arc<RwLock<World>>, count: usize) {
        let mut world = world.write().await;
        for i in 0..count {
            world.create_entity()
                .with(Position { x: i as f32, y: i as f32 })
                .with(Velocity { x: 1.0, y: 1.0 })
                .with(Health { current: 100.0, max: 100.0 })
                .with(if i % 2 == 0 {
                    DamageOverTime { damage_per_second: 5.0 }
                } else {
                    DamageOverTime { damage_per_second: 0.0 }
                })
                .with(if i % 3 == 0 {
                    Healer { heal_per_second: 2.0 }
                } else {
                    Healer { heal_per_second: 0.0 }
                })
                .build();
        }
    }

    // System 1: Movement system
    async fn movement_system(world: Arc<RwLock<World>>) {
        let world = world.read().await;
        let mut query = world.query::<(&mut Position, &Velocity)>();

        let mut positions = Vec::new();
        while let Some((_, (position, velocity))) = query.next().await {
            positions.push((position.x, position.y, velocity.x, velocity.y));
        }

        positions.par_iter_mut().for_each(|(px, py, vx, vy)| {
            *px += *vx;
            *py += *vy;
            println!("Entity moved to: ({}, {})", px, py);
        });

        // Update the actual components
        let mut query = world.query::<&mut Position>();
        let mut i = 0;
        while let Some((_, mut position)) = query.next().await {
            position.x = positions[i].0;
            position.y = positions[i].1;
            i += 1;
        }
    }

    // System 2: Damage over time system
    async fn damage_system(world: Arc<RwLock<World>>) {
        let world = world.read().await;
        let mut query = world.query::<(&mut Health, &DamageOverTime)>();

        let mut healths = Vec::new();
        while let Some((_, (health, dot))) = query.next().await {
            healths.push((health.current, health.max, dot.damage_per_second));
        }

        healths.par_iter_mut().for_each(|(current, max, damage)| {
            *current = (*current - *damage).max(0.0).min(*max);
        });

        // Update the actual components
        let mut query = world.query::<&mut Health>();
        let mut i = 0;
        while let Some((_, mut health)) = query.next().await {
            health.current = healths[i].0;
            i += 1;
        }
    }

    // System 3: Healing system
    async fn healing_system(world: Arc<RwLock<World>>) {
        let world = world.read().await;
        let mut query = world.query::<(&mut Health, &Healer)>();

        let mut healths = Vec::new();
        while let Some((_, (health, healer))) = query.next().await {
            healths.push((health.current, health.max, healer.heal_per_second));
        }

        healths.par_iter_mut().for_each(|(current, max, heal)| {
            *current = (*current + *heal).max(0.0).min(*max);
        });

        // Update the actual components
        let mut query = world.query::<&mut Health>();
        let mut i = 0;
        while let Some((_, mut health)) = query.next().await {
            health.current = healths[i].0;
            i += 1;
        }
    }


    #[test]
    fn test_parallel_systems() {
        let rt = Runtime::new().unwrap();

        rt.block_on(async {
            let world = Arc::new(RwLock::new(World::new()));

            // Create a large number of entities
            create_test_entities(world.clone(), 10000).await;

            // Run systems in parallel
            let world_clone1 = Arc::clone(&world);
            let world_clone2 = Arc::clone(&world);
            let world_clone3 = Arc::clone(&world);

            let handle1 = tokio::spawn(async move { movement_system(world_clone1).await });
            let handle2 = tokio::spawn(async move { damage_system(world_clone2).await });
            let handle3 = tokio::spawn(async move { healing_system(world_clone3).await });

            // Wait for all systems to complete
            let _ = tokio::try_join!(handle1, handle2, handle3);

            // Verify results
            let world = world.read().await;
            let mut query = world.query::<(&Position, &Health)>();

            while let Some((_, (position, health))) = query.next().await {
                assert!(position.x > 0.0 && position.y > 0.0, "Entity should have moved");
                assert!(health.current > 0.0 && health.current <= health.max, "Health should be within valid range");
            }
        });
    }
}
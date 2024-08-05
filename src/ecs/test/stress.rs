#[cfg(test)]
mod tests {
    use crate::net::GET_WORLD;
    use crate::utils::encoding::position::Position;
    use crate::utils::encoding::velocity::Velocity;

    #[tokio::test]
    async fn stress_test_small() {
        let world = GET_WORLD();

        // Create 1000 entities
        for _ in 0..1000 {
            world.create_entity().await
                .with(Position { x: 0, y: 0, z: 0 })
                .with(Velocity { x: 1, y: 1, z: 1 })
                .build();
        }

        let mut query = world.query::<(&mut Position, &Velocity)>();

        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));

        loop {
            let start = std::time::Instant::now();

            while let Some((_, (mut pos, vel))) = query.next().await {
                pos.x += vel.x;
                pos.y += vel.y;
                pos.z += vel.z;
            }
            
            let elapsed = start.elapsed();
            println!("Time taken to update 1000 entities: {:?}", elapsed);
            
            interval.tick().await;
        }
    }
}
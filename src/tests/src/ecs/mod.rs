use ferrumc_core::transform::position::Position;
use ferrumc_ecs::Universe;

#[tokio::test]
#[ignore]
async fn test_1mil_entities() {
    let world = Universe::new();

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    let start = std::time::Instant::now();
    for _ in 0..1_000_000 {
        world
            .builder()
            .with(Position::from((0.0, 0.0, 0.0)))
            .unwrap()
            .with(20f32)
            .unwrap()
            .build();
    }
    println!("Time to create 1mil entities: {:?}", start.elapsed());

    tokio::time::sleep(std::time::Duration::from_secs(15)).await;
}

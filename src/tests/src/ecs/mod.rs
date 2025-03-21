use ferrumc_core::transform::position::Position;
use ferrumc_ecs::Universe;

#[test]
#[ignore]
fn test_1mil_entities() {
    let world = Universe::new();

    thread::sleep(std::time::Duration::from_secs(5));

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

    thread::sleep(std::time::Duration::from_secs(15));
}

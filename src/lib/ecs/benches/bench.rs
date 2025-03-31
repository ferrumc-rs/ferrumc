use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ferrumc_ecs::Universe;

#[allow(dead_code)]
struct Position {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Velocity {
    x: f32,
    y: f32,
}

fn create_entity(universe: &Universe) {
    // entity is 0 here;
    universe
        .builder()
        .with(Position { x: 0.0, y: 0.0 })
        .unwrap()
        .build();
}

fn get_position_immut(universe: &Universe) {
    let position = universe.get::<Position>(0).unwrap();
    assert_eq!(position.x, 0.0);
    assert_eq!(position.y, 0.0);
}

fn get_position_mut(universe: &Universe) {
    let position = universe.get_mut::<Position>(0).unwrap();
    assert_eq!(position.x, 0.0);
    assert_eq!(position.y, 0.0);
}

fn _create_1000_entities_with_pos_and_vel(universe: &Universe) {
    for i in 0..1000 {
        let builder = universe
            .builder()
            .with(Position {
                x: i as f32,
                y: i as f32,
            })
            .unwrap();
        if i % 2 == 0 {
            builder
                .with(Velocity {
                    x: i as f32,
                    y: i as f32,
                })
                .unwrap();
        }
    }
}

fn query_10k_entities(universe: &Universe) {
    let query = universe.query::<(&Position, &Velocity)>();
    for (_, (position, velocity)) in query {
        assert_eq!(position.x, velocity.x);
        assert_eq!(position.y, velocity.y);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut world = Universe::new();
    c.benchmark_group("entity")
        .bench_function("create_entity", |b| {
            b.iter(|| {
                create_entity(black_box(&world));
            });
            // Create a new world after benches is done.
            world = Universe::new();
            world
                .builder()
                .with(Position { x: 0.0, y: 0.0 })
                .unwrap()
                .build();
        })
        .bench_function("get immut", |b| {
            b.iter(|| {
                get_position_immut(black_box(&world));
            });
        })
        .bench_function("get mut", |b| {
            b.iter(|| {
                get_position_mut(black_box(&world));
            });
        })
        .bench_function("query 10k entities", |b| {
            let universe = Universe::new();
            _create_1000_entities_with_pos_and_vel(&universe);
            b.iter(|| {
                query_10k_entities(black_box(&world));
            });
        });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

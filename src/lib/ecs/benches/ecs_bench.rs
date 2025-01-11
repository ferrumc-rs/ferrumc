use std::sync::Arc;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::SliceRandom;
use rand::Rng;
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

async fn create_entity(universe: &Universe) {
    // entity is 0 here;
    universe
        .builder()
        .with(Position { x: 0.0, y: 0.0 })
        .await
        .unwrap()
        .build();
}

async fn get_position_immut(universe: &Universe) {
    let position = universe.get::<Position>(0).await.unwrap();
    assert_eq!(position.x, 0.0);
    assert_eq!(position.y, 0.0);
}

async fn get_position_mut(universe: &Universe) {
    let position = universe.get_mut::<Position>(0).await.unwrap();
    assert_eq!(position.x, 0.0);
    assert_eq!(position.y, 0.0);
}

async fn query_over_tasks(universe: Arc<Universe>) {
    let mut joinset = tokio::task::JoinSet::new();
    let test_fn = async |universe: Arc<Universe>| {
        let mut entities = universe.query::<&Position>().await.into_entities();
        entities.shuffle( &mut rand::rng());
        for eid in entities {
            let mut pos = universe.get_mut::<Position>(eid).await.expect("Failed to get position");
            pos.x = rand::rng().random();
            pos.y = rand::rng().random();
        }
    };
    for _ in (1..500) {
        let universe = universe.clone();
        joinset.spawn(async move{
            test_fn(universe.clone()).await;
        });
    }
    
    joinset.join_all().await;
}

async fn _create_1000_entities_with_pos_and_vel(universe: &Universe) {
    for i in 0..1000 {
        let builder = universe
            .builder()
            .with(Position {
                x: i as f32,
                y: i as f32,
            })
            .await
            .unwrap();
        if i % 2 == 0 {
            builder
                .with(Velocity {
                    x: i as f32,
                    y: i as f32,
                })
                .await
                .unwrap();
        }
    }
}

async fn query_10k_entities(universe: &Universe) {
    let mut query = universe.query::<(&Position, &Velocity)>().await;
    while let Some((_, (position, velocity))) = query.next().await {
        assert_eq!(position.x, velocity.x);
        assert_eq!(position.y, velocity.y);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut world = Universe::new();
    let rt = tokio::runtime::Runtime::new().unwrap();
    c.benchmark_group("entity")
        .bench_function("create_entity", |b| {
            b.to_async(tokio::runtime::Runtime::new().unwrap())
                .iter(|| async { create_entity(&world).await });
            // Create a new world after bench is done.
            world = Universe::new();
            rt.block_on(async {
                world
                    .builder()
                    .with(Position { x: 0.0, y: 0.0 })
                    .await
                    .unwrap()
                    .build();
            });
        })
        .bench_function("get immut", |b| {
            b.to_async(tokio::runtime::Runtime::new().unwrap())
                .iter(async || {
                    get_position_immut(black_box(&world)).await;
                });
        })
        .bench_function("get mut", |b| {
            b.to_async(tokio::runtime::Runtime::new().unwrap())
                .iter(async || {
                    get_position_mut(black_box(&world)).await;
                });
        })
        .bench_function("query 10k entities", |b| {
            let universe = Universe::new();
            rt.block_on(async {
                _create_1000_entities_with_pos_and_vel(&universe).await;
            });
            b.to_async(tokio::runtime::Runtime::new().unwrap())
                .iter(|| async { query_10k_entities(&universe).await });
        })
        .bench_function("query over tasks", |b| {
            let universe = Arc::new(Universe::new());
            rt.block_on(async {
                _create_1000_entities_with_pos_and_vel(&universe).await;
            });
            b.to_async(tokio::runtime::Runtime::new().unwrap())
                .iter(|| async { query_over_tasks(universe.clone()).await });
        });
    ;
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use std::hint::black_box;
use std::thread::sleep;
use crate::components::ComponentStorage;
use crate::entities::EntityManager;
use crate::query::Query;

#[derive(Debug)]
struct Position {
    x: u32,
    y: u32,
}

#[test]
fn test_basic() {
    let entity_manager = EntityManager::new();
    let component_storage = ComponentStorage::new();

    sleep(std::time::Duration::from_secs(5));
    
    let start = std::time::Instant::now();
    for x in 0..100_000 {
        entity_manager
            .builder(&component_storage)
            .with(Position { x, y: x*2 })
            .build();
    }
    println!("Inserting 100_000 entities took {:?}", start.elapsed());
    
    sleep(std::time::Duration::from_secs(5));

    let start = std::time::Instant::now();

    let query = Query::<&Position>::new(&component_storage);

    println!("Creating query took {:?}", start.elapsed());

    let start = std::time::Instant::now();

    let mut x_sum: u64 = 0;
    query.into_iter().for_each(|position| {
        x_sum += position.x as u64;
        black_box(position);
    });
    
    assert_eq!({ x_sum }, 100_000_u64 * 99_999_u64 / 2u64);

    println!("Iterating over 100_000 entities took {:?}", start.elapsed());
    
    sleep(std::time::Duration::from_secs(10));
}
#![allow(dead_code)]

use crate::world::EntityAllocator;


mod world;
mod components;
mod error;
#[test]
fn entry() {
    main();
}

fn main() {
    let mut allocator = EntityAllocator::new();

    let entity1 = allocator.allocate();
    let entity2 = allocator.allocate();

    println!("Created entities: {:?} and {:?}", entity1, entity2);

    // Simulate entity destruction
    let entity1 = allocator.deallocate(entity1).expect("Failed to deallocate entity");
    // Reallocate
    let entity3 = allocator.allocate();

    println!("Reallocated entity: {:?}", entity3);
    println!("Is entity3 equal to entity1? {}", entity3 == entity1);
}
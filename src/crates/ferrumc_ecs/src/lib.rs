#![allow(dead_code)]

use crate::components::{ComponentStorage, Position, Velocity};
use crate::world::{EntityAllocator};

mod world;
mod components;
mod error;
mod macros;
pub mod dsa;

#[test]
fn entry() {
    main();
}

fn main() {
    /*    let mut allocator = EntityAllocator::new();

        let entity1 = allocator.allocate();
        let entity2 = allocator.allocate();

        println!("Created entities: {:?} and {:?}", entity1, entity2);

        // Simulate entity destruction
        let entity1 = allocator.deallocate(entity1).expect("Failed to deallocate entity");
        // Reallocate
        let entity3 = allocator.allocate();

        println!("Reallocated entity: {:?}", entity3);
        println!("Is entity3 equal to entity1? {}", entity3 == entity1);*/

    let mut storage = ComponentStorage::new();
    let mut allocator = EntityAllocator::new();

    let entity1 = allocator.allocate();

    storage.insert(&entity1, Position::new(1.0, 2.0, 3.0));
    storage.insert(&entity1, Velocity::new(4.0, 5.0, 6.0));


    let position = storage.get::<Position>(&entity1).unwrap();
    let velocity = storage.get::<Velocity>(&entity1).unwrap();

    println!("Position: {:?}", position);
    println!("Velocity: {:?}", velocity);


}
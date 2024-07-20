#![allow(dead_code)]

use crate::components::{ComponentStorage, Position};
use crate::world::{Entity, EntityAllocator};

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

    storage.insert(entity1, Position { x: 1.0, y: 2.0, z: 3.0 });





}
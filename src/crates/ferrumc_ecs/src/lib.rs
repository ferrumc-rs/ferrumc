#![allow(dead_code)]

use crate::components::{ComponentStorage, Position, Velocity};
use crate::world::{EntityAllocator};

mod world;
mod components;
mod error;
mod macros;
pub mod dsa;
mod query;

#[test]
fn entry() {
    main();
}

fn main() {
    let mut storage = ComponentStorage::new();
    let mut allocator = EntityAllocator::new();

    let entity1 = allocator.allocate(&mut storage)
        .with(Position::new(1.0, 2.0, 3.0))
        .with(Velocity::new(2.0, 3.0, 4.0))
        .build();

    let position = storage.get::<Position>(&entity1).unwrap();
    let velocity = storage.get::<Velocity>(&entity1).unwrap();

    println!("Position: {:?}", position);
    println!("Velocity: {:?}", velocity);

    let position_storage = storage.get_storage::<Position>().unwrap();
    for (idx, position) in position_storage.iter() {
        println!("Entity: {}, Position: {:?}", idx, position);
    }

}
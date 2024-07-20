#![allow(dead_code)]


pub mod world;
pub mod components;
mod error;
mod macros;
pub mod dsa;
mod query;/*
#[test]
fn entry() {
    main();
}

fn main() {
    let mut world = World::new();

    let entity = world.create_entity()
        .with(Position::new(0.0, 1.0, 0.0))
        .with(Velocity::new(1.0, 1.0, 1.0))
        .build();

    // Query example
    for (entity_id, (pos, vel)) in world.query::<(Position, Velocity)>().iter() {
        println!("Entity {}: Position {:?}, Velocity {:?}", entity_id, pos, vel);
    }

    // Mutable query example
    for (_, (pos, vel)) in world.query_mut::<(Position, Velocity)>().iter_mut() {
        pos.add_velocity(vel);
    }

    // Delete entity
    world.delete_entity(entity).expect("Failed to delete entity");
}*/
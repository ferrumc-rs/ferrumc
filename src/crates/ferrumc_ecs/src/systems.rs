use crate::components::{Position, Velocity};
use crate::world::World;


pub fn movement_system(world: &World) {
    let positions = world.get_all_components::<Position>().unwrap();
    let velocities = world.get_all_components::<Velocity>().unwrap();

    for (&entity, position) in positions {
        if let Some(velocity) = velocities.get(&entity) {
            let mut position = position.borrow_mut();
            let velocity = velocity.borrow();

            println!("Moving entity {}", entity);
            position.x += velocity.x;
            position.y += velocity.y;
        }
    }
}
use crate::component::{Position, Velocity};
use crate::world::World;

mod entity;
mod component;
mod helpers;
mod error;
mod query;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod test;
mod world;


#[tokio::main]
async fn main() {
    let mut world = World::new();

    let entity = world.create_entity()
        .with(Position { x: 0.0, y: 0.0 })
        .build();
    
    let position = world.get_component_storage().get::<Position>(entity).await.unwrap();
    world.get_component_storage().remove::<Position>(entity);
    println!("{:?}", position);
    
    /*world.create_entity()
        .with(Position { x: 0.0, y: 0.0 })
        .with(Velocity { x: 10.0, y: 0.0 })
        .build();

    // No velocity
    world.create_entity()
        .with(Position { x: 1.0, y: 1.0 })
        .build();

    world.create_entity()
        .with(Position { x: 2.0, y: 2.0 })
        .with(Velocity { x: 10.0, y: 10.0 })
        .build();

    let mut query = world.query::<(&mut Position, &Velocity)>();

    while let Some((entity_id, (mut pos, vel))) = query.next().await {
        pos.x += vel.x;
        pos.y += vel.y;
        world.get_component_storage().remove::<Velocity>(entity_id);
    }

    let mut query = world.query::<&Position>();

    while let Some((id, pos)) = query.next().await {
        println!("Entity {}: {:?}", id, *pos);
    }

    let mut query = world.query::<&Velocity>();

    while let Some((id, vel)) = query.next().await {
        println!("Entity {}: {:?}", id, *vel);
    }
*/
}
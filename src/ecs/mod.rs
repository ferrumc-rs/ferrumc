#[cfg(test)]
use std::sync::OnceLock;

pub mod component;
pub mod entity;
pub mod error;
pub mod helpers;
pub mod query;
#[cfg(test)]
pub mod test;
#[cfg(test)]
pub mod tests;
pub mod world;

#[cfg(test)]
#[allow(non_snake_case)]
fn GET_WORLD() -> &'static world::World {
    static WORLD: OnceLock<world::World> = OnceLock::new();
    WORLD.get_or_init(world::World::new)
}

#[cfg(test)]
mod more_tests {
    use crate::ecs::world::World;
    use crate::utils::encoding::position::Position;

    #[tokio::test]
    async fn main() {
        let world = World::new();

        let entity = world
            .create_entity()
            .await
            .with(Position { x: 0, z: 0, y: 0 })
            .build();

        {
            world
                .get_component_storage()
                .get::<Position>(entity)
                .await
                .unwrap();
        }

        if let Err(e) = world.get_component_storage().remove::<Position>(entity) {
            println!("Error: {:?}", e);
        } else {
            println!("Removed Position successfully");
        }

        // println!("{:?}", position);

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
}

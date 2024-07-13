use crate::world::{Entity, World};
use crate::components::{Position, Velocity, Player};

pub fn movement_system(world: &mut World) {
    let mut positions_to_update: Vec<(Entity, Velocity)> = Vec::new();

    for &entity in world.entities() {
        if let (Some(_), Some(vel)) = (
            world.get_component::<Position>(entity),
            world.get_component::<Velocity>(entity),
        ) {
            positions_to_update.push((entity, *vel));
        }
    }

    for (entity, velocity) in positions_to_update {
        if let Some(pos) = world.get_component_mut::<Position>(entity) {
            pos.x += velocity.x;
            pos.y += velocity.y;
            pos.z += velocity.z;
        }
    }
}

pub fn player_health_system(world: &mut World) {
    let mut players_to_remove: Vec<Entity> = Vec::new();

    for &entity in world.entities() {
        if let Some(player) = world.get_component::<Player>(entity) {
            if player.health <= 0 {
                players_to_remove.push(entity);
            }
            println!("Player {} has {} health remaining.",
                     player.name, player.health);
        }
    }

    for entity in players_to_remove {
        println!("Player {} has been removed from the game.",
                 world.get_component::<Player>(entity).unwrap().name);
        // In a real implementation, you'd remove the entity and its components here
    }
}
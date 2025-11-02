use crate::components::EntityType;
use bevy_ecs::prelude::Entity;
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// Request to spawn an entity via command
#[derive(Debug)]
pub struct SpawnRequest {
    pub entity_type: EntityType,
    pub player_entity: Entity, // Entity of the player who issued the command
}

/// Global queue for spawn requests from commands
/// Uses static with interior mutability to avoid Res issues with command macro
static GLOBAL_SPAWN_QUEUE: Lazy<Mutex<Vec<SpawnRequest>>> = Lazy::new(|| Mutex::new(Vec::new()));

/// Add a spawn request to the global queue
pub fn request_spawn(entity_type: EntityType, player_entity: Entity) {
    if let Ok(mut queue) = GLOBAL_SPAWN_QUEUE.lock() {
        queue.push(SpawnRequest {
            entity_type,
            player_entity,
        });
    }
}

/// Drain all pending spawn requests from the global queue
pub fn drain_spawn_requests() -> Vec<SpawnRequest> {
    if let Ok(mut queue) = GLOBAL_SPAWN_QUEUE.lock() {
        queue.drain(..).collect()
    } else {
        Vec::new()
    }
}

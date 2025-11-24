use bevy_ecs::prelude::Entity;
use crossbeam_queue::SegQueue;
use once_cell::sync::Lazy;

/// Type of entity to spawn
#[derive(Debug, Clone, Copy)]
pub enum EntityType {
    Pig,
}

/// A spawn request from a command
#[derive(Debug)]
pub struct SpawnRequest {
    pub entity_type: EntityType,
    pub player_entity: Entity,
}

/// Global lock-free queue for spawn requests from commands
static SPAWN_QUEUE: Lazy<SegQueue<SpawnRequest>> = Lazy::new(SegQueue::new);

/// Add a spawn request to the queue (called from commands)
pub fn request_spawn(entity_type: EntityType, player_entity: Entity) {
    SPAWN_QUEUE.push(SpawnRequest {
        entity_type,
        player_entity,
    });
}

/// Pop a spawn request from the queue (called from system)
pub fn pop_spawn_request() -> Option<SpawnRequest> {
    SPAWN_QUEUE.pop()
}

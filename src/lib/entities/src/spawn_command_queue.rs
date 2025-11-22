use crate::components::EntityType;
use bevy_ecs::prelude::Entity;
use crossbeam_queue::SegQueue;
use once_cell::sync::Lazy;

/// Request to spawn an entity via command
#[derive(Debug)]
pub struct SpawnRequest {
    pub entity_type: EntityType,
    pub player_entity: Entity, // Entity of the player who issued the command
}

/// Global lock-free queue for spawn requests from commands
/// Uses SegQueue for better performance than Mutex<Vec>
static GLOBAL_SPAWN_QUEUE: Lazy<SegQueue<SpawnRequest>> = Lazy::new(SegQueue::new);

/// Add a spawn request to the global queue (lock-free operation)
pub fn request_spawn(entity_type: EntityType, player_entity: Entity) {
    GLOBAL_SPAWN_QUEUE.push(SpawnRequest {
        entity_type,
        player_entity,
    });
}

/// Pop a single spawn request from the global queue
/// Returns None if the queue is empty
pub fn pop_spawn_request() -> Option<SpawnRequest> {
    GLOBAL_SPAWN_QUEUE.pop()
}

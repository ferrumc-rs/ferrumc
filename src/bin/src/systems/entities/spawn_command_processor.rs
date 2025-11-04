use bevy_ecs::prelude::*;
use ferrumc_core::transform::position::Position;
use ferrumc_entities::{pop_spawn_request, SpawnEntityEvent};
use tracing::{debug, warn};

/// System that processes spawn commands from the queue and sends spawn events
pub fn spawn_command_processor_system(
    query: Query<&Position>,
    mut spawn_events: EventWriter<SpawnEntityEvent>,
) {
    // Process all pending spawn requests from the lock-free queue
    while let Some(request) = pop_spawn_request() {
        // Get player position
        if let Ok(pos) = query.get(request.player_entity) {
            // Spawn entity slightly in front of the player (2 blocks away)
            let spawn_pos = Position::new(pos.x + 2.0, pos.y, pos.z + 2.0);

            debug!(
                "Processing spawn command: {:?} at ({:.2}, {:.2}, {:.2})",
                request.entity_type, spawn_pos.x, spawn_pos.y, spawn_pos.z
            );

            spawn_events.write(SpawnEntityEvent {
                entity_type: request.entity_type,
                position: spawn_pos,
            });
        } else {
            warn!(
                "Failed to get position for entity {:?}",
                request.player_entity
            );
        }
    }
}

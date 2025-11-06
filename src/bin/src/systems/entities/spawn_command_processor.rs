use bevy_ecs::prelude::*;
use ferrumc_core::transform::position::Position;
use ferrumc_entities::{pop_spawn_request, SpawnEntityEvent};
use ferrumc_state::GlobalStateResource;
use tracing::warn;

/// System that processes spawn commands from the queue and sends spawn events
pub fn spawn_command_processor_system(
    query: Query<&Position>,
    mut spawn_events: EventWriter<SpawnEntityEvent>,
    _state: Res<GlobalStateResource>,
) {
    // Process all pending spawn requests from the lock-free queue
    while let Some(request) = pop_spawn_request() {
        // Get player position
        if let Ok(pos) = query.get(request.player_entity) {
            // Spawn entity 2 blocks in front of the player at same Y level
            let spawn_pos = Position::new(pos.x + 2.0, pos.y, pos.z + 2.0);

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

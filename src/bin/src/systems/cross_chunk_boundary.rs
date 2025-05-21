use bevy_ecs::prelude::EventReader;
use ferrumc_core::chunks::cross_chunk_boundary_event::CrossChunkBoundaryEvent;
use tracing::debug;

pub fn cross_chunk_boundary(mut events: EventReader<CrossChunkBoundaryEvent>) {
    if events.is_empty() {
        return;
    }
    for event in events.read() {
        // Handle the cross chunk boundary event
        debug!(
            "Player {:?} crossed chunk boundary from {:?} to {:?}",
            event.player, event.old_chunk, event.new_chunk
        );
    }
}
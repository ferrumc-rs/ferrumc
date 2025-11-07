use bevy_ecs::prelude::*;
use ferrumc_entities::{EntityNetworkIdIndex, SpawnEntityEvent};
use ferrumc_state::GlobalStateResource;
use std::sync::atomic::Ordering;
use tracing::info;

/// System that listen spawn event and create entity
pub fn entity_spawner_system(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEntityEvent>,
    mut entity_index: ResMut<EntityNetworkIdIndex>,
    _global_state: Res<GlobalStateResource>,
) {
    for event in spawn_events.read() {
        // Generate new entity ID
        let entity_id = generate_entity_id();

        // Delegate spawning to EntityType
        if let Some(entity) = event
            .entity_type
            .spawn(&mut commands, entity_id, &event.position)
        {
            // Add to network ID index for O(1) lookup
            entity_index.insert(entity_id as i32, entity);

            info!(
                "Spawned {:?} with ID {} at ({:.2}, {:.2}, {:.2})",
                event.entity_type, entity_id, event.position.x, event.position.y, event.position.z
            );
        }
    }
}

// TODO: Implement true ID generator (for now using atomic counter)
// Using i64 to reduce collision risk on large servers with many entities
static NEXT_ENTITY_ID: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(1000);

fn generate_entity_id() -> i64 {
    NEXT_ENTITY_ID.fetch_add(1, Ordering::Relaxed)
}

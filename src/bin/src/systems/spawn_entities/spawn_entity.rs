use bevy_ecs::prelude::*;
use ferrumc_entities::events::SpawnEntityEvent;
use ferrumc_entities::factory::EntityFactory;

pub fn handle_spawn_entity(
    mut commands: Commands,
    mut events: EventReader<SpawnEntityEvent>,
) {
    for ev in events.read() {
        if let Some(entity_id) = EntityFactory::spawn_entity(
            &mut commands,
            ev.entity_kind,
            ev.position.clone(),
        ) {
            tracing::info!("Spawned entity {:?} of kind {:?} at {:?}", 
                entity_id, ev.entity_kind, ev.position);
        }
    }
}

use bevy_ecs::prelude::*;
use ferrumc_core::transform::position::Position;
use ferrumc_entities::components::SyncedToPlayers;
use ferrumc_entities::types::passive::pig::PigBundle;
use ferrumc_entities::SpawnEntityEvent;
use ferrumc_state::GlobalStateResource;
use tracing::info;

/// System that listen spawn event and create entity
pub fn entity_spawner_system(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEntityEvent>,
    _global_state: Res<GlobalStateResource>,
) {
    for event in spawn_events.read() {
        // Generate new entity ID
        let entity_id = generate_entity_id();

        match event.entity_type {
            ferrumc_entities::components::EntityType::Pig => {
                let pig = PigBundle::new(
                    entity_id,
                    Position::new(event.position.x, event.position.y, event.position.z),
                );
                commands.spawn((pig, SyncedToPlayers::default()));
                info!("Spawned pig at {:?}", event.position);
            }
            _ => {
                tracing::warn!("Entity type {:?} not yet implemented", event.entity_type);
            }
        }
    }
}

// TODO: Implémente true ID generator
static mut NEXT_ENTITY_ID: i32 = 1000;
fn generate_entity_id() -> i32 {
    unsafe {
        let id = NEXT_ENTITY_ID;
        NEXT_ENTITY_ID += 1;
        id
    }
}

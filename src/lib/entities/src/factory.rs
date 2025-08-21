use bevy_ecs::prelude::*;
use ferrumc_core::entities::entity_kind::EntityKind;
use ferrumc_core::transform::position::Position;
use crate::bundles::*;
use crate::spawner::SpawnBundleExt;

/// Entity factory for spawning different types of entities
#[derive(Default)]
pub struct EntityFactory;

impl EntityFactory {
    /// Spawn an entity of the given kind at the specified position
    pub fn spawn_entity(
        commands: &mut Commands,
        entity_kind: EntityKind,
        position: Position,
    ) -> Option<Entity> {
        match entity_kind.get_id() {
            ZOMBIE_ID => {
                Some(commands.spawn(ZombieBundle::default().with_position(position)).id())
            }
            // Add more entity types here as they're implemented
            _ => {
                None
            }
        }
    }
}

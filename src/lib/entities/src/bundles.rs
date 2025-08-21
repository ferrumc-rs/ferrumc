use crate::components::{AiComponent, Hostile, Movable, Zombie};
use crate::spawner::SpawnBundleExt;
use bevy_ecs::bundle::Bundle;
use ferrumc_core::collisions::bounding_box::BoundingBox;
use ferrumc_core::entities::entity_kind::EntityKind;
use ferrumc_core::entities::health::Health;
use ferrumc_core::transform::Transform;
use ferrumc_core::transform::position::Position;
use ferrumc_macros::get_registry_entry;

#[derive(Bundle)]
pub struct ZombieBundle {
    pub zombie: Zombie,
    pub entity_kind: EntityKind,
    pub transform: Transform,
    pub health: Health,
    pub bounding_box: BoundingBox,
    pub ai: AiComponent,
    pub movable: Movable,
    pub hostile: Hostile,
}

pub const ZOMBIE_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:zombie");
impl Default for ZombieBundle {
    fn default() -> Self {
        ZombieBundle {
            zombie: Zombie,
            entity_kind: EntityKind::new(ZOMBIE_ID),
            transform: Transform::new((0.0, 64.0, 0.0), (0.0, 0.0)),
            health: Health::new_max(20.0),
            bounding_box: BoundingBox::new((0.3, 0.9, 0.3)),
            ai: AiComponent::default(),
            movable: Movable { speed: 0.25 }, // Slow zombie movement
            hostile: Hostile { damage: 2.0, range: 1.5 }, // Zombie specific stats
        }
    }
}

impl SpawnBundleExt for ZombieBundle {
    fn with_position(mut self, position: Position) -> Self {
        self.transform.position = position;
        self
    }
}

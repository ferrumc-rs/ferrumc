use bevy_ecs::bundle::Bundle;
use glam::vec3;
use ferrumc_core::collisions::bounding_box::BoundingBox;
use ferrumc_core::entities::health::Health;
use ferrumc_core::transform::Transform;
use crate::components::Zombie;

#[derive(Bundle)]
pub struct ZombieBundle {
    pub zombie: Zombie,
    pub transform: Transform,
    pub health: Health,
    pub bounding_box: BoundingBox
}

impl Default for ZombieBundle { 
    fn default() -> Self {
        ZombieBundle {
            zombie: Zombie,
            transform: Transform::new(
                (0.0, 64.0, 0.0),
                (0.0, 0.0)
            ),
            health: Health::new_max(20.0),
            bounding_box: BoundingBox::new((0.3, 0.9, 0.3)),
        }
    }
}
use bevy_ecs::prelude::Component;
use glam::Vec3;

#[derive(Component)]
pub struct BoundingBox {
    pub half_extents: Vec3, // (0.3, 0.9, 0.3) for zombie-ish size
}

impl BoundingBox {
    pub fn new(half_extents: impl Into<Vec3>) -> Self {
        BoundingBox {
            half_extents: half_extents.into(),
        }
    }
}

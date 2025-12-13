use bevy_ecs::prelude::Component;

/// Marker components for entities
#[derive(Component)]
pub struct HasGravity;

#[derive(Component)]
pub struct HasWaterDrag;

// Entity types
mod entity_types {
    use super::Component;
    #[derive(Component)]
    pub struct Pig;
}

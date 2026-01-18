use bevy_ecs::prelude::Component;

/// Marker components for entities
#[derive(Component)]
pub struct HasGravity;

#[derive(Component)]
pub struct HasWaterDrag;

#[derive(Component)]
pub struct HasCollisions;

// Entity types
pub mod entity_types {
    use super::Component;

    #[derive(Component)]
    pub struct Allay;
    #[derive(Component)]
    pub struct Armadillo;
    #[derive(Component)]
    pub struct Axolotl;
    #[derive(Component)]
    pub struct Cow;
    #[derive(Component)]
    pub struct Pig;
}

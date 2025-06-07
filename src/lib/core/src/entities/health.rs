use bevy_ecs::prelude::Component;
use typename::TypeName;

#[derive(TypeName, Debug, Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(current: f32, max: f32) -> Self {
        Self { current, max }
    }
    /// New health with the same current and max value
    pub fn new_max(max: f32) -> Self {
        Self {
            current: max,
            max,
        }
    }
}

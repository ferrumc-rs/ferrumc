use bevy_ecs::prelude::Component;

/// The client's requested view distance (2-32).
#[derive(Component, Debug, Clone, Copy)]
pub struct ViewDistance(pub u8);

impl Default for ViewDistance {
    fn default() -> Self {
        Self(8) // A sensible default
    }
}

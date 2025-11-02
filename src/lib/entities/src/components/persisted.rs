use bevy_ecs::prelude::Component;

/// Marker component for entities that should be persisted to the database
#[derive(Debug, Clone, Copy, Component, Default)]
pub struct Persisted;

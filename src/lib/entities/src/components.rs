use bevy_ecs::prelude::*;

/// Marker type for zombie
#[derive(Component)]
pub struct Zombie;

/// Generic AI component for entities that can have AI behavior
#[derive(Component)]
pub struct AiComponent {
    pub target: Option<Entity>,
    pub state: AiState,
}

#[derive(Debug, Clone, Copy)]
pub enum AiState {
    Idle,
    Chasing,
    Attacking,
    Fleeing,
}

impl Default for AiComponent {
    fn default() -> Self {
        Self {
            target: None,
            state: AiState::Idle,
        }
    }
}

/// Component for entities that can move
#[derive(Component)]
pub struct Movable {
    pub speed: f32,
}

impl Default for Movable {
    fn default() -> Self {
        Self { speed: 1.0 }
    }
}

/// Component for hostile entities
#[derive(Component)]
pub struct Hostile {
    pub damage: f32,
    pub range: f32,
}

impl Default for Hostile {
    fn default() -> Self {
        Self {
            damage: 1.0,
            range: 2.0,
        }
    }
}

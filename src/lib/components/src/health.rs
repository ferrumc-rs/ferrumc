use bevy_ecs::prelude::Component;
use bitcode_derive::{Decode, Encode};

#[derive(Component, Debug, Clone, Copy, Decode, Encode)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 20.0,
            max: 20.0,
        }
    }
}

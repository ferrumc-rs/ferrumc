use bevy_ecs::prelude::Component;
use bitcode_derive::{Decode, Encode};

#[derive(Component, Debug, Clone, Copy, Decode, Encode)]
pub struct Hunger {
    /// 0-20 (half-shanks)
    pub level: u8,
    /// 0.0-5.0 (for regeneration)
    pub saturation: f32,
    /// 0.0-4.0 (accumulates before saturation/hunger drain)
    pub exhaustion: f32,
}

impl Default for Hunger {
    fn default() -> Self {
        Self {
            level: 20,
            saturation: 5.0,
            exhaustion: 0.0,
        }
    }
}

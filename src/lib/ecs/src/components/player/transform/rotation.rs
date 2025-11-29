use bevy_ecs::prelude::Component;
use ferrumc_core::player::transform::rotation::RotationData;
use std::ops::{Deref, DerefMut};
use typename::TypeName;

#[derive(TypeName, Component, Clone, Copy, Debug, Default)]
pub struct Rotation(pub RotationData);

impl Rotation {
    pub fn new(yaw: f32, pitch: f32) -> Self {
        Self(RotationData::new(yaw, pitch))
    }
}

// 1. Deref Magic: This makes `Rotation` behave like `RotationData`.
// You can call .rotate_yaw() on the component directly.
impl Deref for Rotation {
    type Target = RotationData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Rotation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// 2. Converters
impl From<(f32, f32)> for Rotation {
    fn from((yaw, pitch): (f32, f32)) -> Self {
        Self::new(yaw, pitch)
    }
}

impl From<Rotation> for (f32, f32) {
    fn from(rotation: Rotation) -> (f32, f32) {
        (rotation.yaw, rotation.pitch)
    }
}

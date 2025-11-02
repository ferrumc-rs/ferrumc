use bevy_ecs::prelude::Component;
use typename::TypeName;

/// velocity of an entity (m/s)
#[derive(Debug, Clone, Component, TypeName)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Velocity {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self::zero()
    }
}

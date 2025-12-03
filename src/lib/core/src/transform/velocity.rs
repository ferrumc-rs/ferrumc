use bevy_ecs::prelude::Component;
use bevy_math::DVec3;
use std::ops::{Deref, DerefMut};
use typename::TypeName;

/// Velocity component representing the rate of change of position.
///
/// Measured in blocks per tick (at 60 TPS).
/// Positive Y is upward.
#[derive(TypeName, Debug, Component, Clone, Copy)]
pub struct Velocity {
    pub vec: DVec3,
}

impl Velocity {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            vec: DVec3::new(x, y, z),
        }
    }

    pub fn zero() -> Self {
        Self { vec: DVec3::ZERO }
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self::zero()
    }
}

impl Deref for Velocity {
    type Target = DVec3;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}

impl From<DVec3> for Velocity {
    fn from(vec: DVec3) -> Self {
        Self { vec }
    }
}

impl From<Velocity> for DVec3 {
    fn from(velocity: Velocity) -> Self {
        velocity.vec
    }
}

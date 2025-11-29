use bevy_ecs::prelude::Component;
use bevy_math::DVec3; // We keep bevy_math here, in the engine layer
use ferrumc_core::player::transform::position::PositionData;
use ferrumc_protocol::codec::net_types::network_position::NetworkPosition;
use std::ops::{Deref, DerefMut};

/// The Bevy Component wrapper.
#[derive(Component, Clone, Debug, Default)]
pub struct Position(pub PositionData);

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(PositionData::new(x, y, z))
    }

    /// Helper to get a DVec3 for math operations
    pub fn as_dvec3(&self) -> DVec3 {
        DVec3::new(self.0.x, self.0.y, self.0.z)
    }
}

// Implement Deref so you can do `pos.x` directly (skipping the .0)
impl Deref for Position {
    type Target = PositionData;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Network Conversions (because core shouldn't know about NetCodec)
impl From<NetworkPosition> for Position {
    fn from(pos: NetworkPosition) -> Self {
        Self::new(pos.x as f64, pos.y as f64, pos.z as f64)
    }
}

// DVec3 Conversions (physics math)
impl From<DVec3> for Position {
    fn from(vec: DVec3) -> Self {
        Self::new(vec.x, vec.y, vec.z)
    }
}

impl From<&DVec3> for Position {
    fn from(vec: &DVec3) -> Self {
        Self::new(vec.x, vec.y, vec.z)
    }
}

impl From<Position> for DVec3 {
    fn from(pos: Position) -> Self {
        DVec3::new(pos.x, pos.y, pos.z)
    }
}

// so .into() works for tuples
impl From<(f64, f64, f64)> for Position {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self::new(x, y, z)
    }
}

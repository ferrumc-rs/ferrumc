use bevy_ecs::prelude::Component;
use bitcode::{Decode, Encode};
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use typename::TypeName;

#[derive(TypeName, Component, Serialize, Deserialize, Encode, Decode)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<NetworkPosition> for Position {
    fn from(pos: NetworkPosition) -> Self {
        Self::new(pos.x as f64, pos.y as f64, pos.z as f64)
    }
}

// Helper functions:
impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

// Implementations:
impl Default for Position {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl From<(f64, f64, f64)> for Position {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self::new(x, y, z)
    }
}

impl From<&(f64, f64, f64)> for Position {
    fn from((x, y, z): &(f64, f64, f64)) -> Self {
        Self::new(*x, *y, *z)
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Position {{ x: {:.2}, y: {:.2}, z: {:.2} }}",
            self.x, self.y, self.z
        )
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

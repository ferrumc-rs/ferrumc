use bevy_ecs::prelude::Component;
use bevy_math::DVec3;
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use std::{
    fmt::{Debug, Display, Formatter},
    ops::Deref,
};
use typename::TypeName;

#[derive(TypeName, Component)]
pub struct Position {
    pub coords: DVec3,
}

impl From<NetworkPosition> for Position {
    fn from(pos: NetworkPosition) -> Self {
        Self::new(pos.x as f64, pos.y as f64, pos.z as f64)
    }
}

// Helper functions:
impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            coords: DVec3::new(x, y, z),
        }
    }
}

impl Deref for Position {
    type Target = DVec3;

    fn deref(&self) -> &Self::Target {
        &self.coords
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

impl From<&Position> for DVec3 {
    fn from(pos: &Position) -> Self {
        DVec3::new(pos.x, pos.y, pos.z)
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

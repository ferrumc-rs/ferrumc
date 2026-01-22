use bevy_ecs::prelude::Component;
use bevy_math::{DVec3, IVec2};
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use std::ops::DerefMut;
use std::{
    fmt::{Debug, Display, Formatter},
    ops::Deref,
};
use typename::TypeName;

#[derive(TypeName, Component, Clone, Copy)]
pub struct Position {
    pub coords: DVec3,
}

impl From<NetworkPosition> for Position {
    fn from(pos: NetworkPosition) -> Self {
        Self::new(pos.x as f64, pos.y as f64, pos.z as f64)
    }
}

impl From<Position> for NetworkPosition {
    fn from(pos: Position) -> Self {
        NetworkPosition {
            x: pos.x.floor() as i32,
            y: pos.y.floor() as i16,
            z: pos.z.floor() as i32,
        }
    }
}

// Helper functions:
impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            coords: DVec3::new(x, y, z),
        }
    }

    /// Returns a new position offset forward by the given distance based on rotation.
    ///
    /// This calculates a position in front of the current position using the yaw angle.
    /// Useful for spawning entities in front of a player or calculating look direction.
    ///
    /// # Arguments
    ///
    /// * `rotation` - The rotation (yaw/pitch) to use for direction
    /// * `distance` - How many blocks forward to offset
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ferrumc_core::transform::{Position, Rotation};
    ///
    /// let pos = Position::new(0.0, 64.0, 0.0);
    /// let rot = Rotation::new(0.0, 0.0); // Looking north
    /// let forward = pos.offset_forward(&rot, 2.0);
    /// // forward is 2 blocks in front based on yaw
    /// ```
    pub fn offset_forward(&self, rotation: &super::rotation::Rotation, distance: f64) -> Self {
        let yaw_radians = rotation.yaw.to_radians();
        Self::new(
            self.x - (yaw_radians.sin() as f64 * distance),
            self.y,
            self.z + (yaw_radians.cos() as f64 * distance),
        )
    }

    /// Returns the x, y, z coordinates as a tuple for easy spreading.
    ///
    /// This is useful for struct initialization where you need to spread
    /// position coordinates across multiple fields.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let pos = Position::new(10.0, 64.0, 20.0);
    /// let (x, y, z) = pos.xyz();
    /// // Now you can use x, y, z individually
    /// ```
    pub fn xyz(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn chunk(&self) -> IVec2 {
        IVec2::new(self.x.div_euclid(16.0) as _, self.z.div_euclid(16.0) as _)
    }
}

impl Deref for Position {
    type Target = DVec3;

    fn deref(&self) -> &Self::Target {
        &self.coords
    }
}

impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.coords
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

impl From<Position> for (f64, f64, f64) {
    fn from(pos: Position) -> Self {
        (pos.x, pos.y, pos.z)
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

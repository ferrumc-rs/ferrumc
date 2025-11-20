use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

/// pure data representation of a position.
#[derive(Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct PositionData {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PositionData {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Debug for PositionData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Position {{ x: {:.2}, y: {:.2}, z: {:.2} }}",
            self.x, self.y, self.z
        )
    }
}

impl Display for PositionData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

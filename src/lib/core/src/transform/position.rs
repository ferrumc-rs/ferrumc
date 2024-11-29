use std::fmt::{Debug, Display, Formatter};

pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64
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
        write!(f, "Position {{ x: {:.2}, y: {:.2}, z: {:.2} }}", self.x, self.y, self.z)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}
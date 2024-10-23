
#[derive(Debug)]
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
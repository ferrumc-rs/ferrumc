use std::fmt::Debug;

pub struct Rotation {
    pub yaw: f32,
    pub pitch: f32,
}

impl Rotation {
    pub fn new(yaw: f32, pitch: f32) -> Self {
        Self { yaw, pitch }
    }
}

impl Default for Rotation {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl From<(f32, f32)> for Rotation {
    fn from((yaw, pitch): (f32, f32)) -> Self {
        Self::new(yaw, pitch)
    }
}

impl Debug for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rotation {{ yaw: {:.2}, pitch: {:.2} }}", self.yaw, self.pitch)
    }
}
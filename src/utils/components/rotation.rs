use ferrumc_macros::{Component, Constructor, Getter};

#[derive(Debug, Component, Getter, Constructor, Clone)]
pub struct Rotation {
    pub yaw: f32,
    pub pitch: f32,
}

impl Rotation {
    pub fn add_yaw(&mut self, yaw: f32) {
        self.yaw += yaw;
    }

    pub fn add_pitch(&mut self, pitch: f32) {
        self.pitch += pitch;
    }
}

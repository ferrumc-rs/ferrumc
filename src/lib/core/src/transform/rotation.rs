use bevy_ecs::prelude::Component;
use std::fmt::Debug;
use typename::TypeName;

#[derive(TypeName, Component, Clone, Copy, Default)]
pub struct Rotation {
    pub yaw: f32,
    pub pitch: f32,
}

impl Rotation {
    pub fn new(yaw: f32, pitch: f32) -> Self {
        let yaw = yaw % 360.0; // Normalize yaw
        let pitch = pitch.clamp(-90.0, 90.0); // Clamp pitch
        Self { yaw, pitch }
    }

    pub fn rotate_yaw(&mut self, delta_yaw: f32) {
        self.yaw = (self.yaw + delta_yaw) % 360.0; // Normalize yaw
    }

    pub fn rotate_pitch(&mut self, delta_pitch: f32) {
        self.pitch = (self.pitch + delta_pitch).clamp(-90.0, 90.0); // Clamp pitch
    }

    pub fn to_radians(&self) -> (f32, f32) {
        (self.yaw.to_radians(), self.pitch.to_radians())
    }

    pub fn from_radians(yaw_radians: f32, pitch_radians: f32) -> Self {
        Self::new(yaw_radians.to_degrees(), pitch_radians.to_degrees())
    }

    pub fn to_direction_vector(&self) -> (f32, f32, f32) {
        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();

        let x = pitch_rad.cos() * yaw_rad.cos();
        let y = pitch_rad.sin();
        let z = pitch_rad.cos() * yaw_rad.sin();

        (x, y, z)
    }
}

impl Debug for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rotation {{ yaw: {:.2}°, pitch: {:.2}° (normalized) }}",
            self.yaw, self.pitch
        )
    }
}

impl From<(f32, f32)> for Rotation {
    fn from((yaw, pitch): (f32, f32)) -> Self {
        Self::new(yaw, pitch)
    }
}

impl From<Rotation> for (f32, f32) {
    fn from(rotation: Rotation) -> (f32, f32) {
        (rotation.yaw, rotation.pitch)
    }
}

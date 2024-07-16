use std::any::Any;
use std::fmt::Debug;

// Example components
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn add_velocity(&mut self, velocity: &Velocity) {
        self.x += velocity.x;
        self.y += velocity.y;
    }
}
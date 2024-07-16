use std::fmt::Debug;
use crate::macros::impl_component;


// Example components
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl_component!(Position);

#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl_component!(Velocity);

impl Position {
    pub fn add_velocity(&mut self, velocity: &Velocity) {
        self.x += velocity.x;
        self.y += velocity.y;
    }
}

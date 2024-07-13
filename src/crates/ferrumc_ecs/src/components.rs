use std::any::Any;

#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Default)]
pub struct Player {
    pub name: String,
    pub health: i32,
}

/// A marker trait for components.
/// By implementing this trait, a type can be used as a component in the ECS.
pub trait Component: Any + 'static {}
impl<T: Any + 'static> Component for T {}
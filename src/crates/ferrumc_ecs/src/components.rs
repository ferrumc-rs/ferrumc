use std::any::Any;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Position {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Debug)]
pub struct Velocity {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

pub trait Component: Any + 'static + Debug {}
impl<T: Any + 'static + Debug> Component for T {}

use std::any::Any;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Position {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

#[derive(Debug)]
pub struct Velocity {
    pub(crate) x: i32,
    pub(crate) y: i32,
}


pub trait Component: Any + 'static + Debug {}
impl<T: Any + 'static + Debug> Component for T {}

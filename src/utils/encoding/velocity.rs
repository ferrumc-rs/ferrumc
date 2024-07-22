use ferrumc_macros::{Component, Constructor};

/// Represents a velocity in the world
/// FIXME: Just to test the ecs system for now
#[derive(Debug, Component, Constructor)]
pub struct Velocity {
    // Encoded as a 26 bit int
    pub x: i32,
    // Encoded as a 26 bit int
    pub z: i32,
    // Encoded as a 12 bit int
    pub y: i16,
}

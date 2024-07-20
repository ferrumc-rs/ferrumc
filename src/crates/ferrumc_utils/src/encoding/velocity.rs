use ferrumc_macros::Component;

/// Represents a velocity in the world
/// FIXME: Just to test the ecs system for now
#[derive(Debug, Component)]
pub struct Velocity {
    // Encoded as a 26 bit int
    pub x: i32,
    // Encoded as a 26 bit int
    pub z: i32,
    // Encoded as a 12 bit int
    pub y: i16,
}

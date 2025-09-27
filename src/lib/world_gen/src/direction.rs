use Direction::*;
use bevy_math::IVec3;

pub enum Direction {
    Down,
    Up,
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn values() -> [Self; 6] {
        [Down, Up, North, South, West, East]
    }

    pub fn horizontal() -> [Direction; 4] {
        [North, East, South, West]
    }

    pub fn vertical() -> [Direction; 2] {
        [Up, Down]
    }
    pub fn as_unit(&self) -> IVec3 {
        match self {
            Down => IVec3::new(0, -1, 0),
            Up => IVec3::new(0, 1, 0),
            North => IVec3::new(0, 0, -1),
            South => IVec3::new(0, 0, 1),
            West => IVec3::new(-1, 0, 0),
            East => IVec3::new(1, 0, 0),
        }
    }
}

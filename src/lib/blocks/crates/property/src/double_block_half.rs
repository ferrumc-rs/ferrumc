use crate::enum_property;
use bevy_math::IVec3;

enum_property!(
    DoubleBlockHalf,
    Upper => "upper",
    Lower => "lower",
);

impl DoubleBlockHalf {
    pub fn other_half(&self) -> Self {
        match self {
            Self::Upper => Self::Lower,
            Self::Lower => Self::Upper,
        }
    }

    pub fn direction_to_other(&self) -> IVec3 {
        match self {
            Self::Upper => IVec3::new(0, -1, 0),
            Self::Lower => IVec3::new(0, 1, 0),
        }
    }
}

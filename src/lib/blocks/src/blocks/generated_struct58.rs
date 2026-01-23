#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct58 {
    pub facing: Direction,
    pub powered: bool,
    pub waterlogged: bool,
}
impl TryFrom<u32> for GeneratedStruct58 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            25752u32 => Ok(GeneratedStruct58 {
                waterlogged: true,
                facing: Direction::North,
                powered: true,
            }),
            25753u32 => Ok(GeneratedStruct58 {
                facing: Direction::North,
                powered: true,
                waterlogged: false,
            }),
            25754u32 => Ok(GeneratedStruct58 {
                facing: Direction::North,
                powered: false,
                waterlogged: true,
            }),
            25755u32 => Ok(GeneratedStruct58 {
                waterlogged: false,
                facing: Direction::North,
                powered: false,
            }),
            25756u32 => Ok(GeneratedStruct58 {
                facing: Direction::East,
                powered: true,
                waterlogged: true,
            }),
            25757u32 => Ok(GeneratedStruct58 {
                powered: true,
                facing: Direction::East,
                waterlogged: false,
            }),
            25758u32 => Ok(GeneratedStruct58 {
                facing: Direction::East,
                powered: false,
                waterlogged: true,
            }),
            25759u32 => Ok(GeneratedStruct58 {
                facing: Direction::East,
                powered: false,
                waterlogged: false,
            }),
            25760u32 => Ok(GeneratedStruct58 {
                powered: true,
                facing: Direction::South,
                waterlogged: true,
            }),
            25761u32 => Ok(GeneratedStruct58 {
                powered: true,
                facing: Direction::South,
                waterlogged: false,
            }),
            25762u32 => Ok(GeneratedStruct58 {
                facing: Direction::South,
                waterlogged: true,
                powered: false,
            }),
            25763u32 => Ok(GeneratedStruct58 {
                facing: Direction::South,
                powered: false,
                waterlogged: false,
            }),
            25764u32 => Ok(GeneratedStruct58 {
                powered: true,
                waterlogged: true,
                facing: Direction::West,
            }),
            25765u32 => Ok(GeneratedStruct58 {
                powered: true,
                facing: Direction::West,
                waterlogged: false,
            }),
            25766u32 => Ok(GeneratedStruct58 {
                waterlogged: true,
                facing: Direction::West,
                powered: false,
            }),
            25767u32 => Ok(GeneratedStruct58 {
                powered: false,
                facing: Direction::West,
                waterlogged: false,
            }),
            25768u32 => Ok(GeneratedStruct58 {
                facing: Direction::Up,
                powered: true,
                waterlogged: true,
            }),
            25769u32 => Ok(GeneratedStruct58 {
                waterlogged: false,
                powered: true,
                facing: Direction::Up,
            }),
            25770u32 => Ok(GeneratedStruct58 {
                facing: Direction::Up,
                powered: false,
                waterlogged: true,
            }),
            25771u32 => Ok(GeneratedStruct58 {
                waterlogged: false,
                powered: false,
                facing: Direction::Up,
            }),
            25772u32 => Ok(GeneratedStruct58 {
                facing: Direction::Down,
                waterlogged: true,
                powered: true,
            }),
            25773u32 => Ok(GeneratedStruct58 {
                waterlogged: false,
                powered: true,
                facing: Direction::Down,
            }),
            25774u32 => Ok(GeneratedStruct58 {
                facing: Direction::Down,
                powered: false,
                waterlogged: true,
            }),
            25775u32 => Ok(GeneratedStruct58 {
                waterlogged: false,
                facing: Direction::Down,
                powered: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct58 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct58 {
                waterlogged: true,
                facing: Direction::North,
                powered: true,
            } => Ok(25752u32),
            GeneratedStruct58 {
                facing: Direction::North,
                powered: true,
                waterlogged: false,
            } => Ok(25753u32),
            GeneratedStruct58 {
                facing: Direction::North,
                powered: false,
                waterlogged: true,
            } => Ok(25754u32),
            GeneratedStruct58 {
                waterlogged: false,
                facing: Direction::North,
                powered: false,
            } => Ok(25755u32),
            GeneratedStruct58 {
                facing: Direction::East,
                powered: true,
                waterlogged: true,
            } => Ok(25756u32),
            GeneratedStruct58 {
                powered: true,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(25757u32),
            GeneratedStruct58 {
                facing: Direction::East,
                powered: false,
                waterlogged: true,
            } => Ok(25758u32),
            GeneratedStruct58 {
                facing: Direction::East,
                powered: false,
                waterlogged: false,
            } => Ok(25759u32),
            GeneratedStruct58 {
                powered: true,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(25760u32),
            GeneratedStruct58 {
                powered: true,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(25761u32),
            GeneratedStruct58 {
                facing: Direction::South,
                waterlogged: true,
                powered: false,
            } => Ok(25762u32),
            GeneratedStruct58 {
                facing: Direction::South,
                powered: false,
                waterlogged: false,
            } => Ok(25763u32),
            GeneratedStruct58 {
                powered: true,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(25764u32),
            GeneratedStruct58 {
                powered: true,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(25765u32),
            GeneratedStruct58 {
                waterlogged: true,
                facing: Direction::West,
                powered: false,
            } => Ok(25766u32),
            GeneratedStruct58 {
                powered: false,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(25767u32),
            GeneratedStruct58 {
                facing: Direction::Up,
                powered: true,
                waterlogged: true,
            } => Ok(25768u32),
            GeneratedStruct58 {
                waterlogged: false,
                powered: true,
                facing: Direction::Up,
            } => Ok(25769u32),
            GeneratedStruct58 {
                facing: Direction::Up,
                powered: false,
                waterlogged: true,
            } => Ok(25770u32),
            GeneratedStruct58 {
                waterlogged: false,
                powered: false,
                facing: Direction::Up,
            } => Ok(25771u32),
            GeneratedStruct58 {
                facing: Direction::Down,
                waterlogged: true,
                powered: true,
            } => Ok(25772u32),
            GeneratedStruct58 {
                waterlogged: false,
                powered: true,
                facing: Direction::Down,
            } => Ok(25773u32),
            GeneratedStruct58 {
                facing: Direction::Down,
                powered: false,
                waterlogged: true,
            } => Ok(25774u32),
            GeneratedStruct58 {
                waterlogged: false,
                facing: Direction::Down,
                powered: false,
            } => Ok(25775u32),
            _ => Err(()),
        }
    }
}

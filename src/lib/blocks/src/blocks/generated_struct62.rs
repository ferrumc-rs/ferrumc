#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct62 {
    pub facing: Direction,
    pub tilt: Tilt,
    pub waterlogged: bool,
}
impl TryFrom<u32> for GeneratedStruct62 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            25904u32 => Ok(GeneratedStruct62 {
                facing: Direction::North,
                tilt: Tilt::None,
                waterlogged: true,
            }),
            25905u32 => Ok(GeneratedStruct62 {
                tilt: Tilt::None,
                waterlogged: false,
                facing: Direction::North,
            }),
            25906u32 => Ok(GeneratedStruct62 {
                facing: Direction::North,
                waterlogged: true,
                tilt: Tilt::Unstable,
            }),
            25907u32 => Ok(GeneratedStruct62 {
                facing: Direction::North,
                tilt: Tilt::Unstable,
                waterlogged: false,
            }),
            25908u32 => Ok(GeneratedStruct62 {
                waterlogged: true,
                facing: Direction::North,
                tilt: Tilt::Partial,
            }),
            25909u32 => Ok(GeneratedStruct62 {
                tilt: Tilt::Partial,
                facing: Direction::North,
                waterlogged: false,
            }),
            25910u32 => Ok(GeneratedStruct62 {
                waterlogged: true,
                facing: Direction::North,
                tilt: Tilt::Full,
            }),
            25911u32 => Ok(GeneratedStruct62 {
                tilt: Tilt::Full,
                waterlogged: false,
                facing: Direction::North,
            }),
            25912u32 => Ok(GeneratedStruct62 {
                facing: Direction::South,
                tilt: Tilt::None,
                waterlogged: true,
            }),
            25913u32 => Ok(GeneratedStruct62 {
                waterlogged: false,
                facing: Direction::South,
                tilt: Tilt::None,
            }),
            25914u32 => Ok(GeneratedStruct62 {
                facing: Direction::South,
                tilt: Tilt::Unstable,
                waterlogged: true,
            }),
            25915u32 => Ok(GeneratedStruct62 {
                tilt: Tilt::Unstable,
                waterlogged: false,
                facing: Direction::South,
            }),
            25916u32 => Ok(GeneratedStruct62 {
                facing: Direction::South,
                waterlogged: true,
                tilt: Tilt::Partial,
            }),
            25917u32 => Ok(GeneratedStruct62 {
                facing: Direction::South,
                waterlogged: false,
                tilt: Tilt::Partial,
            }),
            25918u32 => Ok(GeneratedStruct62 {
                waterlogged: true,
                tilt: Tilt::Full,
                facing: Direction::South,
            }),
            25919u32 => Ok(GeneratedStruct62 {
                waterlogged: false,
                facing: Direction::South,
                tilt: Tilt::Full,
            }),
            25920u32 => Ok(GeneratedStruct62 {
                tilt: Tilt::None,
                waterlogged: true,
                facing: Direction::West,
            }),
            25921u32 => Ok(GeneratedStruct62 {
                tilt: Tilt::None,
                waterlogged: false,
                facing: Direction::West,
            }),
            25922u32 => Ok(GeneratedStruct62 {
                tilt: Tilt::Unstable,
                waterlogged: true,
                facing: Direction::West,
            }),
            25923u32 => Ok(GeneratedStruct62 {
                facing: Direction::West,
                tilt: Tilt::Unstable,
                waterlogged: false,
            }),
            25924u32 => Ok(GeneratedStruct62 {
                facing: Direction::West,
                waterlogged: true,
                tilt: Tilt::Partial,
            }),
            25925u32 => Ok(GeneratedStruct62 {
                waterlogged: false,
                tilt: Tilt::Partial,
                facing: Direction::West,
            }),
            25926u32 => Ok(GeneratedStruct62 {
                facing: Direction::West,
                tilt: Tilt::Full,
                waterlogged: true,
            }),
            25927u32 => Ok(GeneratedStruct62 {
                facing: Direction::West,
                tilt: Tilt::Full,
                waterlogged: false,
            }),
            25928u32 => Ok(GeneratedStruct62 {
                waterlogged: true,
                facing: Direction::East,
                tilt: Tilt::None,
            }),
            25929u32 => Ok(GeneratedStruct62 {
                tilt: Tilt::None,
                facing: Direction::East,
                waterlogged: false,
            }),
            25930u32 => Ok(GeneratedStruct62 {
                facing: Direction::East,
                tilt: Tilt::Unstable,
                waterlogged: true,
            }),
            25931u32 => Ok(GeneratedStruct62 {
                waterlogged: false,
                tilt: Tilt::Unstable,
                facing: Direction::East,
            }),
            25932u32 => Ok(GeneratedStruct62 {
                waterlogged: true,
                facing: Direction::East,
                tilt: Tilt::Partial,
            }),
            25933u32 => Ok(GeneratedStruct62 {
                waterlogged: false,
                tilt: Tilt::Partial,
                facing: Direction::East,
            }),
            25934u32 => Ok(GeneratedStruct62 {
                facing: Direction::East,
                tilt: Tilt::Full,
                waterlogged: true,
            }),
            25935u32 => Ok(GeneratedStruct62 {
                waterlogged: false,
                tilt: Tilt::Full,
                facing: Direction::East,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct62 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct62 {
                facing: Direction::North,
                tilt: Tilt::None,
                waterlogged: true,
            } => Ok(25904u32),
            GeneratedStruct62 {
                tilt: Tilt::None,
                waterlogged: false,
                facing: Direction::North,
            } => Ok(25905u32),
            GeneratedStruct62 {
                facing: Direction::North,
                waterlogged: true,
                tilt: Tilt::Unstable,
            } => Ok(25906u32),
            GeneratedStruct62 {
                facing: Direction::North,
                tilt: Tilt::Unstable,
                waterlogged: false,
            } => Ok(25907u32),
            GeneratedStruct62 {
                waterlogged: true,
                facing: Direction::North,
                tilt: Tilt::Partial,
            } => Ok(25908u32),
            GeneratedStruct62 {
                tilt: Tilt::Partial,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(25909u32),
            GeneratedStruct62 {
                waterlogged: true,
                facing: Direction::North,
                tilt: Tilt::Full,
            } => Ok(25910u32),
            GeneratedStruct62 {
                tilt: Tilt::Full,
                waterlogged: false,
                facing: Direction::North,
            } => Ok(25911u32),
            GeneratedStruct62 {
                facing: Direction::South,
                tilt: Tilt::None,
                waterlogged: true,
            } => Ok(25912u32),
            GeneratedStruct62 {
                waterlogged: false,
                facing: Direction::South,
                tilt: Tilt::None,
            } => Ok(25913u32),
            GeneratedStruct62 {
                facing: Direction::South,
                tilt: Tilt::Unstable,
                waterlogged: true,
            } => Ok(25914u32),
            GeneratedStruct62 {
                tilt: Tilt::Unstable,
                waterlogged: false,
                facing: Direction::South,
            } => Ok(25915u32),
            GeneratedStruct62 {
                facing: Direction::South,
                waterlogged: true,
                tilt: Tilt::Partial,
            } => Ok(25916u32),
            GeneratedStruct62 {
                facing: Direction::South,
                waterlogged: false,
                tilt: Tilt::Partial,
            } => Ok(25917u32),
            GeneratedStruct62 {
                waterlogged: true,
                tilt: Tilt::Full,
                facing: Direction::South,
            } => Ok(25918u32),
            GeneratedStruct62 {
                waterlogged: false,
                facing: Direction::South,
                tilt: Tilt::Full,
            } => Ok(25919u32),
            GeneratedStruct62 {
                tilt: Tilt::None,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(25920u32),
            GeneratedStruct62 {
                tilt: Tilt::None,
                waterlogged: false,
                facing: Direction::West,
            } => Ok(25921u32),
            GeneratedStruct62 {
                tilt: Tilt::Unstable,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(25922u32),
            GeneratedStruct62 {
                facing: Direction::West,
                tilt: Tilt::Unstable,
                waterlogged: false,
            } => Ok(25923u32),
            GeneratedStruct62 {
                facing: Direction::West,
                waterlogged: true,
                tilt: Tilt::Partial,
            } => Ok(25924u32),
            GeneratedStruct62 {
                waterlogged: false,
                tilt: Tilt::Partial,
                facing: Direction::West,
            } => Ok(25925u32),
            GeneratedStruct62 {
                facing: Direction::West,
                tilt: Tilt::Full,
                waterlogged: true,
            } => Ok(25926u32),
            GeneratedStruct62 {
                facing: Direction::West,
                tilt: Tilt::Full,
                waterlogged: false,
            } => Ok(25927u32),
            GeneratedStruct62 {
                waterlogged: true,
                facing: Direction::East,
                tilt: Tilt::None,
            } => Ok(25928u32),
            GeneratedStruct62 {
                tilt: Tilt::None,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(25929u32),
            GeneratedStruct62 {
                facing: Direction::East,
                tilt: Tilt::Unstable,
                waterlogged: true,
            } => Ok(25930u32),
            GeneratedStruct62 {
                waterlogged: false,
                tilt: Tilt::Unstable,
                facing: Direction::East,
            } => Ok(25931u32),
            GeneratedStruct62 {
                waterlogged: true,
                facing: Direction::East,
                tilt: Tilt::Partial,
            } => Ok(25932u32),
            GeneratedStruct62 {
                waterlogged: false,
                tilt: Tilt::Partial,
                facing: Direction::East,
            } => Ok(25933u32),
            GeneratedStruct62 {
                facing: Direction::East,
                tilt: Tilt::Full,
                waterlogged: true,
            } => Ok(25934u32),
            GeneratedStruct62 {
                waterlogged: false,
                tilt: Tilt::Full,
                facing: Direction::East,
            } => Ok(25935u32),
            _ => Err(()),
        }
    }
}

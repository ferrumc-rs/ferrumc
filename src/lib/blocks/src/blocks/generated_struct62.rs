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
                waterlogged: false,
                facing: Direction::North,
                tilt: Tilt::None,
            }),
            25906u32 => Ok(GeneratedStruct62 {
                waterlogged: true,
                tilt: Tilt::Unstable,
                facing: Direction::North,
            }),
            25907u32 => Ok(GeneratedStruct62 {
                waterlogged: false,
                tilt: Tilt::Unstable,
                facing: Direction::North,
            }),
            25908u32 => Ok(GeneratedStruct62 {
                facing: Direction::North,
                tilt: Tilt::Partial,
                waterlogged: true,
            }),
            25909u32 => Ok(GeneratedStruct62 {
                waterlogged: false,
                tilt: Tilt::Partial,
                facing: Direction::North,
            }),
            25910u32 => Ok(GeneratedStruct62 {
                facing: Direction::North,
                waterlogged: true,
                tilt: Tilt::Full,
            }),
            25911u32 => Ok(GeneratedStruct62 {
                tilt: Tilt::Full,
                waterlogged: false,
                facing: Direction::North,
            }),
            25912u32 => Ok(GeneratedStruct62 {
                facing: Direction::South,
                waterlogged: true,
                tilt: Tilt::None,
            }),
            25913u32 => Ok(GeneratedStruct62 {
                tilt: Tilt::None,
                facing: Direction::South,
                waterlogged: false,
            }),
            25914u32 => Ok(GeneratedStruct62 {
                waterlogged: true,
                tilt: Tilt::Unstable,
                facing: Direction::South,
            }),
            25915u32 => Ok(GeneratedStruct62 {
                waterlogged: false,
                tilt: Tilt::Unstable,
                facing: Direction::South,
            }),
            25916u32 => Ok(GeneratedStruct62 {
                waterlogged: true,
                facing: Direction::South,
                tilt: Tilt::Partial,
            }),
            25917u32 => Ok(GeneratedStruct62 {
                facing: Direction::South,
                tilt: Tilt::Partial,
                waterlogged: false,
            }),
            25918u32 => Ok(GeneratedStruct62 {
                facing: Direction::South,
                tilt: Tilt::Full,
                waterlogged: true,
            }),
            25919u32 => Ok(GeneratedStruct62 {
                facing: Direction::South,
                tilt: Tilt::Full,
                waterlogged: false,
            }),
            25920u32 => Ok(GeneratedStruct62 {
                waterlogged: true,
                tilt: Tilt::None,
                facing: Direction::West,
            }),
            25921u32 => Ok(GeneratedStruct62 {
                waterlogged: false,
                facing: Direction::West,
                tilt: Tilt::None,
            }),
            25922u32 => Ok(GeneratedStruct62 {
                waterlogged: true,
                facing: Direction::West,
                tilt: Tilt::Unstable,
            }),
            25923u32 => Ok(GeneratedStruct62 {
                tilt: Tilt::Unstable,
                waterlogged: false,
                facing: Direction::West,
            }),
            25924u32 => Ok(GeneratedStruct62 {
                waterlogged: true,
                facing: Direction::West,
                tilt: Tilt::Partial,
            }),
            25925u32 => Ok(GeneratedStruct62 {
                tilt: Tilt::Partial,
                facing: Direction::West,
                waterlogged: false,
            }),
            25926u32 => Ok(GeneratedStruct62 {
                tilt: Tilt::Full,
                waterlogged: true,
                facing: Direction::West,
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
                facing: Direction::East,
                tilt: Tilt::None,
                waterlogged: false,
            }),
            25930u32 => Ok(GeneratedStruct62 {
                tilt: Tilt::Unstable,
                waterlogged: true,
                facing: Direction::East,
            }),
            25931u32 => Ok(GeneratedStruct62 {
                waterlogged: false,
                facing: Direction::East,
                tilt: Tilt::Unstable,
            }),
            25932u32 => Ok(GeneratedStruct62 {
                waterlogged: true,
                tilt: Tilt::Partial,
                facing: Direction::East,
            }),
            25933u32 => Ok(GeneratedStruct62 {
                facing: Direction::East,
                tilt: Tilt::Partial,
                waterlogged: false,
            }),
            25934u32 => Ok(GeneratedStruct62 {
                waterlogged: true,
                facing: Direction::East,
                tilt: Tilt::Full,
            }),
            25935u32 => Ok(GeneratedStruct62 {
                facing: Direction::East,
                waterlogged: false,
                tilt: Tilt::Full,
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
                waterlogged: false,
                facing: Direction::North,
                tilt: Tilt::None,
            } => Ok(25905u32),
            GeneratedStruct62 {
                waterlogged: true,
                tilt: Tilt::Unstable,
                facing: Direction::North,
            } => Ok(25906u32),
            GeneratedStruct62 {
                waterlogged: false,
                tilt: Tilt::Unstable,
                facing: Direction::North,
            } => Ok(25907u32),
            GeneratedStruct62 {
                facing: Direction::North,
                tilt: Tilt::Partial,
                waterlogged: true,
            } => Ok(25908u32),
            GeneratedStruct62 {
                waterlogged: false,
                tilt: Tilt::Partial,
                facing: Direction::North,
            } => Ok(25909u32),
            GeneratedStruct62 {
                facing: Direction::North,
                waterlogged: true,
                tilt: Tilt::Full,
            } => Ok(25910u32),
            GeneratedStruct62 {
                tilt: Tilt::Full,
                waterlogged: false,
                facing: Direction::North,
            } => Ok(25911u32),
            GeneratedStruct62 {
                facing: Direction::South,
                waterlogged: true,
                tilt: Tilt::None,
            } => Ok(25912u32),
            GeneratedStruct62 {
                tilt: Tilt::None,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(25913u32),
            GeneratedStruct62 {
                waterlogged: true,
                tilt: Tilt::Unstable,
                facing: Direction::South,
            } => Ok(25914u32),
            GeneratedStruct62 {
                waterlogged: false,
                tilt: Tilt::Unstable,
                facing: Direction::South,
            } => Ok(25915u32),
            GeneratedStruct62 {
                waterlogged: true,
                facing: Direction::South,
                tilt: Tilt::Partial,
            } => Ok(25916u32),
            GeneratedStruct62 {
                facing: Direction::South,
                tilt: Tilt::Partial,
                waterlogged: false,
            } => Ok(25917u32),
            GeneratedStruct62 {
                facing: Direction::South,
                tilt: Tilt::Full,
                waterlogged: true,
            } => Ok(25918u32),
            GeneratedStruct62 {
                facing: Direction::South,
                tilt: Tilt::Full,
                waterlogged: false,
            } => Ok(25919u32),
            GeneratedStruct62 {
                waterlogged: true,
                tilt: Tilt::None,
                facing: Direction::West,
            } => Ok(25920u32),
            GeneratedStruct62 {
                waterlogged: false,
                facing: Direction::West,
                tilt: Tilt::None,
            } => Ok(25921u32),
            GeneratedStruct62 {
                waterlogged: true,
                facing: Direction::West,
                tilt: Tilt::Unstable,
            } => Ok(25922u32),
            GeneratedStruct62 {
                tilt: Tilt::Unstable,
                waterlogged: false,
                facing: Direction::West,
            } => Ok(25923u32),
            GeneratedStruct62 {
                waterlogged: true,
                facing: Direction::West,
                tilt: Tilt::Partial,
            } => Ok(25924u32),
            GeneratedStruct62 {
                tilt: Tilt::Partial,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(25925u32),
            GeneratedStruct62 {
                tilt: Tilt::Full,
                waterlogged: true,
                facing: Direction::West,
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
                facing: Direction::East,
                tilt: Tilt::None,
                waterlogged: false,
            } => Ok(25929u32),
            GeneratedStruct62 {
                tilt: Tilt::Unstable,
                waterlogged: true,
                facing: Direction::East,
            } => Ok(25930u32),
            GeneratedStruct62 {
                waterlogged: false,
                facing: Direction::East,
                tilt: Tilt::Unstable,
            } => Ok(25931u32),
            GeneratedStruct62 {
                waterlogged: true,
                tilt: Tilt::Partial,
                facing: Direction::East,
            } => Ok(25932u32),
            GeneratedStruct62 {
                facing: Direction::East,
                tilt: Tilt::Partial,
                waterlogged: false,
            } => Ok(25933u32),
            GeneratedStruct62 {
                waterlogged: true,
                facing: Direction::East,
                tilt: Tilt::Full,
            } => Ok(25934u32),
            GeneratedStruct62 {
                facing: Direction::East,
                waterlogged: false,
                tilt: Tilt::Full,
            } => Ok(25935u32),
            _ => Err(()),
        }
    }
}

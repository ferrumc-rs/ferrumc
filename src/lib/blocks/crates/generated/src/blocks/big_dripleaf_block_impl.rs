use crate::BigDripleafBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for BigDripleafBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            25904u32 => Ok(BigDripleafBlock {
                facing: Direction::North,
                tilt: Tilt::None,
                waterlogged: true,
            }),
            25905u32 => Ok(BigDripleafBlock {
                facing: Direction::North,
                tilt: Tilt::None,
                waterlogged: false,
            }),
            25906u32 => Ok(BigDripleafBlock {
                facing: Direction::North,
                tilt: Tilt::Unstable,
                waterlogged: true,
            }),
            25907u32 => Ok(BigDripleafBlock {
                facing: Direction::North,
                tilt: Tilt::Unstable,
                waterlogged: false,
            }),
            25908u32 => Ok(BigDripleafBlock {
                facing: Direction::North,
                tilt: Tilt::Partial,
                waterlogged: true,
            }),
            25909u32 => Ok(BigDripleafBlock {
                facing: Direction::North,
                tilt: Tilt::Partial,
                waterlogged: false,
            }),
            25910u32 => Ok(BigDripleafBlock {
                facing: Direction::North,
                tilt: Tilt::Full,
                waterlogged: true,
            }),
            25911u32 => Ok(BigDripleafBlock {
                facing: Direction::North,
                tilt: Tilt::Full,
                waterlogged: false,
            }),
            25912u32 => Ok(BigDripleafBlock {
                facing: Direction::South,
                tilt: Tilt::None,
                waterlogged: true,
            }),
            25913u32 => Ok(BigDripleafBlock {
                facing: Direction::South,
                tilt: Tilt::None,
                waterlogged: false,
            }),
            25914u32 => Ok(BigDripleafBlock {
                facing: Direction::South,
                tilt: Tilt::Unstable,
                waterlogged: true,
            }),
            25915u32 => Ok(BigDripleafBlock {
                facing: Direction::South,
                tilt: Tilt::Unstable,
                waterlogged: false,
            }),
            25916u32 => Ok(BigDripleafBlock {
                facing: Direction::South,
                tilt: Tilt::Partial,
                waterlogged: true,
            }),
            25917u32 => Ok(BigDripleafBlock {
                facing: Direction::South,
                tilt: Tilt::Partial,
                waterlogged: false,
            }),
            25918u32 => Ok(BigDripleafBlock {
                facing: Direction::South,
                tilt: Tilt::Full,
                waterlogged: true,
            }),
            25919u32 => Ok(BigDripleafBlock {
                facing: Direction::South,
                tilt: Tilt::Full,
                waterlogged: false,
            }),
            25920u32 => Ok(BigDripleafBlock {
                facing: Direction::West,
                tilt: Tilt::None,
                waterlogged: true,
            }),
            25921u32 => Ok(BigDripleafBlock {
                facing: Direction::West,
                tilt: Tilt::None,
                waterlogged: false,
            }),
            25922u32 => Ok(BigDripleafBlock {
                facing: Direction::West,
                tilt: Tilt::Unstable,
                waterlogged: true,
            }),
            25923u32 => Ok(BigDripleafBlock {
                facing: Direction::West,
                tilt: Tilt::Unstable,
                waterlogged: false,
            }),
            25924u32 => Ok(BigDripleafBlock {
                facing: Direction::West,
                tilt: Tilt::Partial,
                waterlogged: true,
            }),
            25925u32 => Ok(BigDripleafBlock {
                facing: Direction::West,
                tilt: Tilt::Partial,
                waterlogged: false,
            }),
            25926u32 => Ok(BigDripleafBlock {
                facing: Direction::West,
                tilt: Tilt::Full,
                waterlogged: true,
            }),
            25927u32 => Ok(BigDripleafBlock {
                facing: Direction::West,
                tilt: Tilt::Full,
                waterlogged: false,
            }),
            25928u32 => Ok(BigDripleafBlock {
                facing: Direction::East,
                tilt: Tilt::None,
                waterlogged: true,
            }),
            25929u32 => Ok(BigDripleafBlock {
                facing: Direction::East,
                tilt: Tilt::None,
                waterlogged: false,
            }),
            25930u32 => Ok(BigDripleafBlock {
                facing: Direction::East,
                tilt: Tilt::Unstable,
                waterlogged: true,
            }),
            25931u32 => Ok(BigDripleafBlock {
                facing: Direction::East,
                tilt: Tilt::Unstable,
                waterlogged: false,
            }),
            25932u32 => Ok(BigDripleafBlock {
                facing: Direction::East,
                tilt: Tilt::Partial,
                waterlogged: true,
            }),
            25933u32 => Ok(BigDripleafBlock {
                facing: Direction::East,
                tilt: Tilt::Partial,
                waterlogged: false,
            }),
            25934u32 => Ok(BigDripleafBlock {
                facing: Direction::East,
                tilt: Tilt::Full,
                waterlogged: true,
            }),
            25935u32 => Ok(BigDripleafBlock {
                facing: Direction::East,
                tilt: Tilt::Full,
                waterlogged: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for BigDripleafBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            BigDripleafBlock {
                facing: Direction::North,
                tilt: Tilt::None,
                waterlogged: true,
            } => Ok(25904u32),
            BigDripleafBlock {
                facing: Direction::North,
                tilt: Tilt::None,
                waterlogged: false,
            } => Ok(25905u32),
            BigDripleafBlock {
                facing: Direction::North,
                tilt: Tilt::Unstable,
                waterlogged: true,
            } => Ok(25906u32),
            BigDripleafBlock {
                facing: Direction::North,
                tilt: Tilt::Unstable,
                waterlogged: false,
            } => Ok(25907u32),
            BigDripleafBlock {
                facing: Direction::North,
                tilt: Tilt::Partial,
                waterlogged: true,
            } => Ok(25908u32),
            BigDripleafBlock {
                facing: Direction::North,
                tilt: Tilt::Partial,
                waterlogged: false,
            } => Ok(25909u32),
            BigDripleafBlock {
                facing: Direction::North,
                tilt: Tilt::Full,
                waterlogged: true,
            } => Ok(25910u32),
            BigDripleafBlock {
                facing: Direction::North,
                tilt: Tilt::Full,
                waterlogged: false,
            } => Ok(25911u32),
            BigDripleafBlock {
                facing: Direction::South,
                tilt: Tilt::None,
                waterlogged: true,
            } => Ok(25912u32),
            BigDripleafBlock {
                facing: Direction::South,
                tilt: Tilt::None,
                waterlogged: false,
            } => Ok(25913u32),
            BigDripleafBlock {
                facing: Direction::South,
                tilt: Tilt::Unstable,
                waterlogged: true,
            } => Ok(25914u32),
            BigDripleafBlock {
                facing: Direction::South,
                tilt: Tilt::Unstable,
                waterlogged: false,
            } => Ok(25915u32),
            BigDripleafBlock {
                facing: Direction::South,
                tilt: Tilt::Partial,
                waterlogged: true,
            } => Ok(25916u32),
            BigDripleafBlock {
                facing: Direction::South,
                tilt: Tilt::Partial,
                waterlogged: false,
            } => Ok(25917u32),
            BigDripleafBlock {
                facing: Direction::South,
                tilt: Tilt::Full,
                waterlogged: true,
            } => Ok(25918u32),
            BigDripleafBlock {
                facing: Direction::South,
                tilt: Tilt::Full,
                waterlogged: false,
            } => Ok(25919u32),
            BigDripleafBlock {
                facing: Direction::West,
                tilt: Tilt::None,
                waterlogged: true,
            } => Ok(25920u32),
            BigDripleafBlock {
                facing: Direction::West,
                tilt: Tilt::None,
                waterlogged: false,
            } => Ok(25921u32),
            BigDripleafBlock {
                facing: Direction::West,
                tilt: Tilt::Unstable,
                waterlogged: true,
            } => Ok(25922u32),
            BigDripleafBlock {
                facing: Direction::West,
                tilt: Tilt::Unstable,
                waterlogged: false,
            } => Ok(25923u32),
            BigDripleafBlock {
                facing: Direction::West,
                tilt: Tilt::Partial,
                waterlogged: true,
            } => Ok(25924u32),
            BigDripleafBlock {
                facing: Direction::West,
                tilt: Tilt::Partial,
                waterlogged: false,
            } => Ok(25925u32),
            BigDripleafBlock {
                facing: Direction::West,
                tilt: Tilt::Full,
                waterlogged: true,
            } => Ok(25926u32),
            BigDripleafBlock {
                facing: Direction::West,
                tilt: Tilt::Full,
                waterlogged: false,
            } => Ok(25927u32),
            BigDripleafBlock {
                facing: Direction::East,
                tilt: Tilt::None,
                waterlogged: true,
            } => Ok(25928u32),
            BigDripleafBlock {
                facing: Direction::East,
                tilt: Tilt::None,
                waterlogged: false,
            } => Ok(25929u32),
            BigDripleafBlock {
                facing: Direction::East,
                tilt: Tilt::Unstable,
                waterlogged: true,
            } => Ok(25930u32),
            BigDripleafBlock {
                facing: Direction::East,
                tilt: Tilt::Unstable,
                waterlogged: false,
            } => Ok(25931u32),
            BigDripleafBlock {
                facing: Direction::East,
                tilt: Tilt::Partial,
                waterlogged: true,
            } => Ok(25932u32),
            BigDripleafBlock {
                facing: Direction::East,
                tilt: Tilt::Partial,
                waterlogged: false,
            } => Ok(25933u32),
            BigDripleafBlock {
                facing: Direction::East,
                tilt: Tilt::Full,
                waterlogged: true,
            } => Ok(25934u32),
            BigDripleafBlock {
                facing: Direction::East,
                tilt: Tilt::Full,
                waterlogged: false,
            } => Ok(25935u32),
            _ => Err(()),
        }
    }
}

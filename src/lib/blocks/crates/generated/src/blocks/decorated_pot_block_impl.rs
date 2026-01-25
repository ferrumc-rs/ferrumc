use crate::DecoratedPotBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for DecoratedPotBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            27634u32 => Ok(DecoratedPotBlock {
                cracked: true,
                facing: Direction::North,
                waterlogged: true,
            }),
            27635u32 => Ok(DecoratedPotBlock {
                cracked: true,
                facing: Direction::North,
                waterlogged: false,
            }),
            27636u32 => Ok(DecoratedPotBlock {
                cracked: true,
                facing: Direction::South,
                waterlogged: true,
            }),
            27637u32 => Ok(DecoratedPotBlock {
                cracked: true,
                facing: Direction::South,
                waterlogged: false,
            }),
            27638u32 => Ok(DecoratedPotBlock {
                cracked: true,
                facing: Direction::West,
                waterlogged: true,
            }),
            27639u32 => Ok(DecoratedPotBlock {
                cracked: true,
                facing: Direction::West,
                waterlogged: false,
            }),
            27640u32 => Ok(DecoratedPotBlock {
                cracked: true,
                facing: Direction::East,
                waterlogged: true,
            }),
            27641u32 => Ok(DecoratedPotBlock {
                cracked: true,
                facing: Direction::East,
                waterlogged: false,
            }),
            27642u32 => Ok(DecoratedPotBlock {
                cracked: false,
                facing: Direction::North,
                waterlogged: true,
            }),
            27643u32 => Ok(DecoratedPotBlock {
                cracked: false,
                facing: Direction::North,
                waterlogged: false,
            }),
            27644u32 => Ok(DecoratedPotBlock {
                cracked: false,
                facing: Direction::South,
                waterlogged: true,
            }),
            27645u32 => Ok(DecoratedPotBlock {
                cracked: false,
                facing: Direction::South,
                waterlogged: false,
            }),
            27646u32 => Ok(DecoratedPotBlock {
                cracked: false,
                facing: Direction::West,
                waterlogged: true,
            }),
            27647u32 => Ok(DecoratedPotBlock {
                cracked: false,
                facing: Direction::West,
                waterlogged: false,
            }),
            27648u32 => Ok(DecoratedPotBlock {
                cracked: false,
                facing: Direction::East,
                waterlogged: true,
            }),
            27649u32 => Ok(DecoratedPotBlock {
                cracked: false,
                facing: Direction::East,
                waterlogged: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for DecoratedPotBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            DecoratedPotBlock {
                cracked: true,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(27634u32),
            DecoratedPotBlock {
                cracked: true,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(27635u32),
            DecoratedPotBlock {
                cracked: true,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(27636u32),
            DecoratedPotBlock {
                cracked: true,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(27637u32),
            DecoratedPotBlock {
                cracked: true,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(27638u32),
            DecoratedPotBlock {
                cracked: true,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(27639u32),
            DecoratedPotBlock {
                cracked: true,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(27640u32),
            DecoratedPotBlock {
                cracked: true,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(27641u32),
            DecoratedPotBlock {
                cracked: false,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(27642u32),
            DecoratedPotBlock {
                cracked: false,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(27643u32),
            DecoratedPotBlock {
                cracked: false,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(27644u32),
            DecoratedPotBlock {
                cracked: false,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(27645u32),
            DecoratedPotBlock {
                cracked: false,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(27646u32),
            DecoratedPotBlock {
                cracked: false,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(27647u32),
            DecoratedPotBlock {
                cracked: false,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(27648u32),
            DecoratedPotBlock {
                cracked: false,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(27649u32),
            _ => Err(()),
        }
    }
}

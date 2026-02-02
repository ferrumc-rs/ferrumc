use crate::LightningRodBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for LightningRodBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            25752u32 => Ok(LightningRodBlock {
                facing: Direction::North,
                powered: true,
                waterlogged: true,
            }),
            25753u32 => Ok(LightningRodBlock {
                facing: Direction::North,
                powered: true,
                waterlogged: false,
            }),
            25754u32 => Ok(LightningRodBlock {
                facing: Direction::North,
                powered: false,
                waterlogged: true,
            }),
            25755u32 => Ok(LightningRodBlock {
                facing: Direction::North,
                powered: false,
                waterlogged: false,
            }),
            25756u32 => Ok(LightningRodBlock {
                facing: Direction::East,
                powered: true,
                waterlogged: true,
            }),
            25757u32 => Ok(LightningRodBlock {
                facing: Direction::East,
                powered: true,
                waterlogged: false,
            }),
            25758u32 => Ok(LightningRodBlock {
                facing: Direction::East,
                powered: false,
                waterlogged: true,
            }),
            25759u32 => Ok(LightningRodBlock {
                facing: Direction::East,
                powered: false,
                waterlogged: false,
            }),
            25760u32 => Ok(LightningRodBlock {
                facing: Direction::South,
                powered: true,
                waterlogged: true,
            }),
            25761u32 => Ok(LightningRodBlock {
                facing: Direction::South,
                powered: true,
                waterlogged: false,
            }),
            25762u32 => Ok(LightningRodBlock {
                facing: Direction::South,
                powered: false,
                waterlogged: true,
            }),
            25763u32 => Ok(LightningRodBlock {
                facing: Direction::South,
                powered: false,
                waterlogged: false,
            }),
            25764u32 => Ok(LightningRodBlock {
                facing: Direction::West,
                powered: true,
                waterlogged: true,
            }),
            25765u32 => Ok(LightningRodBlock {
                facing: Direction::West,
                powered: true,
                waterlogged: false,
            }),
            25766u32 => Ok(LightningRodBlock {
                facing: Direction::West,
                powered: false,
                waterlogged: true,
            }),
            25767u32 => Ok(LightningRodBlock {
                facing: Direction::West,
                powered: false,
                waterlogged: false,
            }),
            25768u32 => Ok(LightningRodBlock {
                facing: Direction::Up,
                powered: true,
                waterlogged: true,
            }),
            25769u32 => Ok(LightningRodBlock {
                facing: Direction::Up,
                powered: true,
                waterlogged: false,
            }),
            25770u32 => Ok(LightningRodBlock {
                facing: Direction::Up,
                powered: false,
                waterlogged: true,
            }),
            25771u32 => Ok(LightningRodBlock {
                facing: Direction::Up,
                powered: false,
                waterlogged: false,
            }),
            25772u32 => Ok(LightningRodBlock {
                facing: Direction::Down,
                powered: true,
                waterlogged: true,
            }),
            25773u32 => Ok(LightningRodBlock {
                facing: Direction::Down,
                powered: true,
                waterlogged: false,
            }),
            25774u32 => Ok(LightningRodBlock {
                facing: Direction::Down,
                powered: false,
                waterlogged: true,
            }),
            25775u32 => Ok(LightningRodBlock {
                facing: Direction::Down,
                powered: false,
                waterlogged: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for LightningRodBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            LightningRodBlock {
                facing: Direction::North,
                powered: true,
                waterlogged: true,
            } => Ok(25752u32),
            LightningRodBlock {
                facing: Direction::North,
                powered: true,
                waterlogged: false,
            } => Ok(25753u32),
            LightningRodBlock {
                facing: Direction::North,
                powered: false,
                waterlogged: true,
            } => Ok(25754u32),
            LightningRodBlock {
                facing: Direction::North,
                powered: false,
                waterlogged: false,
            } => Ok(25755u32),
            LightningRodBlock {
                facing: Direction::East,
                powered: true,
                waterlogged: true,
            } => Ok(25756u32),
            LightningRodBlock {
                facing: Direction::East,
                powered: true,
                waterlogged: false,
            } => Ok(25757u32),
            LightningRodBlock {
                facing: Direction::East,
                powered: false,
                waterlogged: true,
            } => Ok(25758u32),
            LightningRodBlock {
                facing: Direction::East,
                powered: false,
                waterlogged: false,
            } => Ok(25759u32),
            LightningRodBlock {
                facing: Direction::South,
                powered: true,
                waterlogged: true,
            } => Ok(25760u32),
            LightningRodBlock {
                facing: Direction::South,
                powered: true,
                waterlogged: false,
            } => Ok(25761u32),
            LightningRodBlock {
                facing: Direction::South,
                powered: false,
                waterlogged: true,
            } => Ok(25762u32),
            LightningRodBlock {
                facing: Direction::South,
                powered: false,
                waterlogged: false,
            } => Ok(25763u32),
            LightningRodBlock {
                facing: Direction::West,
                powered: true,
                waterlogged: true,
            } => Ok(25764u32),
            LightningRodBlock {
                facing: Direction::West,
                powered: true,
                waterlogged: false,
            } => Ok(25765u32),
            LightningRodBlock {
                facing: Direction::West,
                powered: false,
                waterlogged: true,
            } => Ok(25766u32),
            LightningRodBlock {
                facing: Direction::West,
                powered: false,
                waterlogged: false,
            } => Ok(25767u32),
            LightningRodBlock {
                facing: Direction::Up,
                powered: true,
                waterlogged: true,
            } => Ok(25768u32),
            LightningRodBlock {
                facing: Direction::Up,
                powered: true,
                waterlogged: false,
            } => Ok(25769u32),
            LightningRodBlock {
                facing: Direction::Up,
                powered: false,
                waterlogged: true,
            } => Ok(25770u32),
            LightningRodBlock {
                facing: Direction::Up,
                powered: false,
                waterlogged: false,
            } => Ok(25771u32),
            LightningRodBlock {
                facing: Direction::Down,
                powered: true,
                waterlogged: true,
            } => Ok(25772u32),
            LightningRodBlock {
                facing: Direction::Down,
                powered: true,
                waterlogged: false,
            } => Ok(25773u32),
            LightningRodBlock {
                facing: Direction::Down,
                powered: false,
                waterlogged: true,
            } => Ok(25774u32),
            LightningRodBlock {
                facing: Direction::Down,
                powered: false,
                waterlogged: false,
            } => Ok(25775u32),
            _ => Err(()),
        }
    }
}

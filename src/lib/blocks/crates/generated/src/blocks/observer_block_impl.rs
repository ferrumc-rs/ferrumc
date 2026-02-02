use crate::ObserverBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for ObserverBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            13573u32 => Ok(ObserverBlock {
                facing: Direction::North,
                powered: true,
            }),
            13574u32 => Ok(ObserverBlock {
                facing: Direction::North,
                powered: false,
            }),
            13575u32 => Ok(ObserverBlock {
                facing: Direction::East,
                powered: true,
            }),
            13576u32 => Ok(ObserverBlock {
                facing: Direction::East,
                powered: false,
            }),
            13577u32 => Ok(ObserverBlock {
                facing: Direction::South,
                powered: true,
            }),
            13578u32 => Ok(ObserverBlock {
                facing: Direction::South,
                powered: false,
            }),
            13579u32 => Ok(ObserverBlock {
                facing: Direction::West,
                powered: true,
            }),
            13580u32 => Ok(ObserverBlock {
                facing: Direction::West,
                powered: false,
            }),
            13581u32 => Ok(ObserverBlock {
                facing: Direction::Up,
                powered: true,
            }),
            13582u32 => Ok(ObserverBlock {
                facing: Direction::Up,
                powered: false,
            }),
            13583u32 => Ok(ObserverBlock {
                facing: Direction::Down,
                powered: true,
            }),
            13584u32 => Ok(ObserverBlock {
                facing: Direction::Down,
                powered: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for ObserverBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            ObserverBlock {
                facing: Direction::North,
                powered: true,
            } => Ok(13573u32),
            ObserverBlock {
                facing: Direction::North,
                powered: false,
            } => Ok(13574u32),
            ObserverBlock {
                facing: Direction::East,
                powered: true,
            } => Ok(13575u32),
            ObserverBlock {
                facing: Direction::East,
                powered: false,
            } => Ok(13576u32),
            ObserverBlock {
                facing: Direction::South,
                powered: true,
            } => Ok(13577u32),
            ObserverBlock {
                facing: Direction::South,
                powered: false,
            } => Ok(13578u32),
            ObserverBlock {
                facing: Direction::West,
                powered: true,
            } => Ok(13579u32),
            ObserverBlock {
                facing: Direction::West,
                powered: false,
            } => Ok(13580u32),
            ObserverBlock {
                facing: Direction::Up,
                powered: true,
            } => Ok(13581u32),
            ObserverBlock {
                facing: Direction::Up,
                powered: false,
            } => Ok(13582u32),
            ObserverBlock {
                facing: Direction::Down,
                powered: true,
            } => Ok(13583u32),
            ObserverBlock {
                facing: Direction::Down,
                powered: false,
            } => Ok(13584u32),
            _ => Err(()),
        }
    }
}

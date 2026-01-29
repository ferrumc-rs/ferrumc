use crate::BarrelBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for BarrelBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            19431u32 => Ok(BarrelBlock {
                facing: Direction::North,
                open: true,
            }),
            19432u32 => Ok(BarrelBlock {
                facing: Direction::North,
                open: false,
            }),
            19433u32 => Ok(BarrelBlock {
                facing: Direction::East,
                open: true,
            }),
            19434u32 => Ok(BarrelBlock {
                facing: Direction::East,
                open: false,
            }),
            19435u32 => Ok(BarrelBlock {
                facing: Direction::South,
                open: true,
            }),
            19436u32 => Ok(BarrelBlock {
                facing: Direction::South,
                open: false,
            }),
            19437u32 => Ok(BarrelBlock {
                facing: Direction::West,
                open: true,
            }),
            19438u32 => Ok(BarrelBlock {
                facing: Direction::West,
                open: false,
            }),
            19439u32 => Ok(BarrelBlock {
                facing: Direction::Up,
                open: true,
            }),
            19440u32 => Ok(BarrelBlock {
                facing: Direction::Up,
                open: false,
            }),
            19441u32 => Ok(BarrelBlock {
                facing: Direction::Down,
                open: true,
            }),
            19442u32 => Ok(BarrelBlock {
                facing: Direction::Down,
                open: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for BarrelBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            BarrelBlock {
                facing: Direction::North,
                open: true,
            } => Ok(19431u32),
            BarrelBlock {
                facing: Direction::North,
                open: false,
            } => Ok(19432u32),
            BarrelBlock {
                facing: Direction::East,
                open: true,
            } => Ok(19433u32),
            BarrelBlock {
                facing: Direction::East,
                open: false,
            } => Ok(19434u32),
            BarrelBlock {
                facing: Direction::South,
                open: true,
            } => Ok(19435u32),
            BarrelBlock {
                facing: Direction::South,
                open: false,
            } => Ok(19436u32),
            BarrelBlock {
                facing: Direction::West,
                open: true,
            } => Ok(19437u32),
            BarrelBlock {
                facing: Direction::West,
                open: false,
            } => Ok(19438u32),
            BarrelBlock {
                facing: Direction::Up,
                open: true,
            } => Ok(19439u32),
            BarrelBlock {
                facing: Direction::Up,
                open: false,
            } => Ok(19440u32),
            BarrelBlock {
                facing: Direction::Down,
                open: true,
            } => Ok(19441u32),
            BarrelBlock {
                facing: Direction::Down,
                open: false,
            } => Ok(19442u32),
            _ => Err(()),
        }
    }
}

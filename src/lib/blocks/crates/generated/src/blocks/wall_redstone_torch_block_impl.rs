use crate::WallRedstoneTorchBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for WallRedstoneTorchBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            5918u32 => Ok(WallRedstoneTorchBlock {
                facing: Direction::North,
                lit: true,
            }),
            5919u32 => Ok(WallRedstoneTorchBlock {
                facing: Direction::North,
                lit: false,
            }),
            5920u32 => Ok(WallRedstoneTorchBlock {
                facing: Direction::South,
                lit: true,
            }),
            5921u32 => Ok(WallRedstoneTorchBlock {
                facing: Direction::South,
                lit: false,
            }),
            5922u32 => Ok(WallRedstoneTorchBlock {
                facing: Direction::West,
                lit: true,
            }),
            5923u32 => Ok(WallRedstoneTorchBlock {
                facing: Direction::West,
                lit: false,
            }),
            5924u32 => Ok(WallRedstoneTorchBlock {
                facing: Direction::East,
                lit: true,
            }),
            5925u32 => Ok(WallRedstoneTorchBlock {
                facing: Direction::East,
                lit: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for WallRedstoneTorchBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            WallRedstoneTorchBlock {
                facing: Direction::North,
                lit: true,
            } => Ok(5918u32),
            WallRedstoneTorchBlock {
                facing: Direction::North,
                lit: false,
            } => Ok(5919u32),
            WallRedstoneTorchBlock {
                facing: Direction::South,
                lit: true,
            } => Ok(5920u32),
            WallRedstoneTorchBlock {
                facing: Direction::South,
                lit: false,
            } => Ok(5921u32),
            WallRedstoneTorchBlock {
                facing: Direction::West,
                lit: true,
            } => Ok(5922u32),
            WallRedstoneTorchBlock {
                facing: Direction::West,
                lit: false,
            } => Ok(5923u32),
            WallRedstoneTorchBlock {
                facing: Direction::East,
                lit: true,
            } => Ok(5924u32),
            WallRedstoneTorchBlock {
                facing: Direction::East,
                lit: false,
            } => Ok(5925u32),
            _ => Err(()),
        }
    }
}

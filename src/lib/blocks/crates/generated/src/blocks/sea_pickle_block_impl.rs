use crate::SeaPickleBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for SeaPickleBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            13956u32 => Ok(SeaPickleBlock {
                pickles: 1i32,
                waterlogged: true,
            }),
            13957u32 => Ok(SeaPickleBlock {
                pickles: 1i32,
                waterlogged: false,
            }),
            13958u32 => Ok(SeaPickleBlock {
                pickles: 2i32,
                waterlogged: true,
            }),
            13959u32 => Ok(SeaPickleBlock {
                pickles: 2i32,
                waterlogged: false,
            }),
            13960u32 => Ok(SeaPickleBlock {
                pickles: 3i32,
                waterlogged: true,
            }),
            13961u32 => Ok(SeaPickleBlock {
                pickles: 3i32,
                waterlogged: false,
            }),
            13962u32 => Ok(SeaPickleBlock {
                pickles: 4i32,
                waterlogged: true,
            }),
            13963u32 => Ok(SeaPickleBlock {
                pickles: 4i32,
                waterlogged: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for SeaPickleBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            SeaPickleBlock {
                pickles: 1i32,
                waterlogged: true,
            } => Ok(13956u32),
            SeaPickleBlock {
                pickles: 1i32,
                waterlogged: false,
            } => Ok(13957u32),
            SeaPickleBlock {
                pickles: 2i32,
                waterlogged: true,
            } => Ok(13958u32),
            SeaPickleBlock {
                pickles: 2i32,
                waterlogged: false,
            } => Ok(13959u32),
            SeaPickleBlock {
                pickles: 3i32,
                waterlogged: true,
            } => Ok(13960u32),
            SeaPickleBlock {
                pickles: 3i32,
                waterlogged: false,
            } => Ok(13961u32),
            SeaPickleBlock {
                pickles: 4i32,
                waterlogged: true,
            } => Ok(13962u32),
            SeaPickleBlock {
                pickles: 4i32,
                waterlogged: false,
            } => Ok(13963u32),
            _ => Err(()),
        }
    }
}

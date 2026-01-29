use crate::ChainBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for ChainBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            7016u32 => Ok(ChainBlock {
                axis: Axis::X,
                waterlogged: true,
            }),
            7017u32 => Ok(ChainBlock {
                axis: Axis::X,
                waterlogged: false,
            }),
            7018u32 => Ok(ChainBlock {
                axis: Axis::Y,
                waterlogged: true,
            }),
            7019u32 => Ok(ChainBlock {
                axis: Axis::Y,
                waterlogged: false,
            }),
            7020u32 => Ok(ChainBlock {
                axis: Axis::Z,
                waterlogged: true,
            }),
            7021u32 => Ok(ChainBlock {
                axis: Axis::Z,
                waterlogged: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for ChainBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            ChainBlock {
                axis: Axis::X,
                waterlogged: true,
            } => Ok(7016u32),
            ChainBlock {
                axis: Axis::X,
                waterlogged: false,
            } => Ok(7017u32),
            ChainBlock {
                axis: Axis::Y,
                waterlogged: true,
            } => Ok(7018u32),
            ChainBlock {
                axis: Axis::Y,
                waterlogged: false,
            } => Ok(7019u32),
            ChainBlock {
                axis: Axis::Z,
                waterlogged: true,
            } => Ok(7020u32),
            ChainBlock {
                axis: Axis::Z,
                waterlogged: false,
            } => Ok(7021u32),
            _ => Err(()),
        }
    }
}

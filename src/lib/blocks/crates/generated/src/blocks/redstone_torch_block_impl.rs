use crate::RedstoneTorchBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for RedstoneTorchBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            5916u32 => Ok(RedstoneTorchBlock { lit: true }),
            5917u32 => Ok(RedstoneTorchBlock { lit: false }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for RedstoneTorchBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            RedstoneTorchBlock { lit: true } => Ok(5916u32),
            RedstoneTorchBlock { lit: false } => Ok(5917u32),
            _ => Err(()),
        }
    }
}

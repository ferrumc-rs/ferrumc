use crate::RedstoneLampBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for RedstoneLampBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            8201u32 => Ok(RedstoneLampBlock { lit: true }),
            8202u32 => Ok(RedstoneLampBlock { lit: false }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for RedstoneLampBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            RedstoneLampBlock { lit: true } => Ok(8201u32),
            RedstoneLampBlock { lit: false } => Ok(8202u32),
            _ => Err(()),
        }
    }
}

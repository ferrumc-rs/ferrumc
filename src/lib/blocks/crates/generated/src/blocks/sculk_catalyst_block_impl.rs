use crate::SculkCatalystBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for SculkCatalystBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            23956u32 => Ok(SculkCatalystBlock { bloom: true }),
            23957u32 => Ok(SculkCatalystBlock { bloom: false }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for SculkCatalystBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            SculkCatalystBlock { bloom: true } => Ok(23956u32),
            SculkCatalystBlock { bloom: false } => Ok(23957u32),
            _ => Err(()),
        }
    }
}

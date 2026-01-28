use crate::TntBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for TntBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            2140u32 => Ok(TntBlock { unstable: true }),
            2141u32 => Ok(TntBlock { unstable: false }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for TntBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            TntBlock { unstable: true } => Ok(2140u32),
            TntBlock { unstable: false } => Ok(2141u32),
            _ => Err(()),
        }
    }
}

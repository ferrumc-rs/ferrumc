use crate::PaleHangingMossBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for PaleHangingMossBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            27907u32 => Ok(PaleHangingMossBlock { tip: true }),
            27908u32 => Ok(PaleHangingMossBlock { tip: false }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for PaleHangingMossBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            PaleHangingMossBlock { tip: true } => Ok(27907u32),
            PaleHangingMossBlock { tip: false } => Ok(27908u32),
            _ => Err(()),
        }
    }
}

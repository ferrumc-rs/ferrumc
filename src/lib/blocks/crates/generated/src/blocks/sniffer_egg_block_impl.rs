use crate::SnifferEggBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for SnifferEggBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            13823u32 => Ok(SnifferEggBlock { hatch: 0i32 }),
            13824u32 => Ok(SnifferEggBlock { hatch: 1i32 }),
            13825u32 => Ok(SnifferEggBlock { hatch: 2i32 }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for SnifferEggBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            SnifferEggBlock { hatch: 0i32 } => Ok(13823u32),
            SnifferEggBlock { hatch: 1i32 } => Ok(13824u32),
            SnifferEggBlock { hatch: 2i32 } => Ok(13825u32),
            _ => Err(()),
        }
    }
}

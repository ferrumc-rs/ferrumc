use crate::BubbleColumnBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for BubbleColumnBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            13983u32 => Ok(BubbleColumnBlock { drag: true }),
            13984u32 => Ok(BubbleColumnBlock { drag: false }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for BubbleColumnBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            BubbleColumnBlock { drag: true } => Ok(13983u32),
            BubbleColumnBlock { drag: false } => Ok(13984u32),
            _ => Err(()),
        }
    }
}

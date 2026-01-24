use crate::FrostedIceBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for FrostedIceBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            13562u32 => Ok(FrostedIceBlock { age: 0i32 }),
            13563u32 => Ok(FrostedIceBlock { age: 1i32 }),
            13564u32 => Ok(FrostedIceBlock { age: 2i32 }),
            13565u32 => Ok(FrostedIceBlock { age: 3i32 }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for FrostedIceBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            FrostedIceBlock { age: 0i32 } => Ok(13562u32),
            FrostedIceBlock { age: 1i32 } => Ok(13563u32),
            FrostedIceBlock { age: 2i32 } => Ok(13564u32),
            FrostedIceBlock { age: 3i32 } => Ok(13565u32),
            _ => Err(()),
        }
    }
}

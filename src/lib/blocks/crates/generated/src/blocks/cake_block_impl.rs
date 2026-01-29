use crate::CakeBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for CakeBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            6053u32 => Ok(CakeBlock { bites: 0i32 }),
            6054u32 => Ok(CakeBlock { bites: 1i32 }),
            6055u32 => Ok(CakeBlock { bites: 2i32 }),
            6056u32 => Ok(CakeBlock { bites: 3i32 }),
            6057u32 => Ok(CakeBlock { bites: 4i32 }),
            6058u32 => Ok(CakeBlock { bites: 5i32 }),
            6059u32 => Ok(CakeBlock { bites: 6i32 }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for CakeBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            CakeBlock { bites: 0i32 } => Ok(6053u32),
            CakeBlock { bites: 1i32 } => Ok(6054u32),
            CakeBlock { bites: 2i32 } => Ok(6055u32),
            CakeBlock { bites: 3i32 } => Ok(6056u32),
            CakeBlock { bites: 4i32 } => Ok(6057u32),
            CakeBlock { bites: 5i32 } => Ok(6058u32),
            CakeBlock { bites: 6i32 } => Ok(6059u32),
            _ => Err(()),
        }
    }
}

use crate::FarmlandBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for FarmlandBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            4350u32 => Ok(FarmlandBlock { moisture: 0i32 }),
            4351u32 => Ok(FarmlandBlock { moisture: 1i32 }),
            4352u32 => Ok(FarmlandBlock { moisture: 2i32 }),
            4353u32 => Ok(FarmlandBlock { moisture: 3i32 }),
            4354u32 => Ok(FarmlandBlock { moisture: 4i32 }),
            4355u32 => Ok(FarmlandBlock { moisture: 5i32 }),
            4356u32 => Ok(FarmlandBlock { moisture: 6i32 }),
            4357u32 => Ok(FarmlandBlock { moisture: 7i32 }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for FarmlandBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            FarmlandBlock { moisture: 0i32 } => Ok(4350u32),
            FarmlandBlock { moisture: 1i32 } => Ok(4351u32),
            FarmlandBlock { moisture: 2i32 } => Ok(4352u32),
            FarmlandBlock { moisture: 3i32 } => Ok(4353u32),
            FarmlandBlock { moisture: 4i32 } => Ok(4354u32),
            FarmlandBlock { moisture: 5i32 } => Ok(4355u32),
            FarmlandBlock { moisture: 6i32 } => Ok(4356u32),
            FarmlandBlock { moisture: 7i32 } => Ok(4357u32),
            _ => Err(()),
        }
    }
}

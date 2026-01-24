use crate::ComposterBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for ComposterBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            20400u32 => Ok(ComposterBlock { level: 0i32 }),
            20401u32 => Ok(ComposterBlock { level: 1i32 }),
            20402u32 => Ok(ComposterBlock { level: 2i32 }),
            20403u32 => Ok(ComposterBlock { level: 3i32 }),
            20404u32 => Ok(ComposterBlock { level: 4i32 }),
            20405u32 => Ok(ComposterBlock { level: 5i32 }),
            20406u32 => Ok(ComposterBlock { level: 6i32 }),
            20407u32 => Ok(ComposterBlock { level: 7i32 }),
            20408u32 => Ok(ComposterBlock { level: 8i32 }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for ComposterBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            ComposterBlock { level: 0i32 } => Ok(20400u32),
            ComposterBlock { level: 1i32 } => Ok(20401u32),
            ComposterBlock { level: 2i32 } => Ok(20402u32),
            ComposterBlock { level: 3i32 } => Ok(20403u32),
            ComposterBlock { level: 4i32 } => Ok(20404u32),
            ComposterBlock { level: 5i32 } => Ok(20405u32),
            ComposterBlock { level: 6i32 } => Ok(20406u32),
            ComposterBlock { level: 7i32 } => Ok(20407u32),
            ComposterBlock { level: 8i32 } => Ok(20408u32),
            _ => Err(()),
        }
    }
}

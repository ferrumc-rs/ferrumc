use crate::SnowBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for SnowBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            5950u32 => Ok(SnowBlock { layers: 1i32 }),
            5951u32 => Ok(SnowBlock { layers: 2i32 }),
            5952u32 => Ok(SnowBlock { layers: 3i32 }),
            5953u32 => Ok(SnowBlock { layers: 4i32 }),
            5954u32 => Ok(SnowBlock { layers: 5i32 }),
            5955u32 => Ok(SnowBlock { layers: 6i32 }),
            5956u32 => Ok(SnowBlock { layers: 7i32 }),
            5957u32 => Ok(SnowBlock { layers: 8i32 }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for SnowBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            SnowBlock { layers: 1i32 } => Ok(5950u32),
            SnowBlock { layers: 2i32 } => Ok(5951u32),
            SnowBlock { layers: 3i32 } => Ok(5952u32),
            SnowBlock { layers: 4i32 } => Ok(5953u32),
            SnowBlock { layers: 5i32 } => Ok(5954u32),
            SnowBlock { layers: 6i32 } => Ok(5955u32),
            SnowBlock { layers: 7i32 } => Ok(5956u32),
            SnowBlock { layers: 8i32 } => Ok(5957u32),
            _ => Err(()),
        }
    }
}

use crate::TargetBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for TargetBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            20409u32 => Ok(TargetBlock { power: 0i32 }),
            20410u32 => Ok(TargetBlock { power: 1i32 }),
            20411u32 => Ok(TargetBlock { power: 2i32 }),
            20412u32 => Ok(TargetBlock { power: 3i32 }),
            20413u32 => Ok(TargetBlock { power: 4i32 }),
            20414u32 => Ok(TargetBlock { power: 5i32 }),
            20415u32 => Ok(TargetBlock { power: 6i32 }),
            20416u32 => Ok(TargetBlock { power: 7i32 }),
            20417u32 => Ok(TargetBlock { power: 8i32 }),
            20418u32 => Ok(TargetBlock { power: 9i32 }),
            20419u32 => Ok(TargetBlock { power: 10i32 }),
            20420u32 => Ok(TargetBlock { power: 11i32 }),
            20421u32 => Ok(TargetBlock { power: 12i32 }),
            20422u32 => Ok(TargetBlock { power: 13i32 }),
            20423u32 => Ok(TargetBlock { power: 14i32 }),
            20424u32 => Ok(TargetBlock { power: 15i32 }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for TargetBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            TargetBlock { power: 0i32 } => Ok(20409u32),
            TargetBlock { power: 1i32 } => Ok(20410u32),
            TargetBlock { power: 2i32 } => Ok(20411u32),
            TargetBlock { power: 3i32 } => Ok(20412u32),
            TargetBlock { power: 4i32 } => Ok(20413u32),
            TargetBlock { power: 5i32 } => Ok(20414u32),
            TargetBlock { power: 6i32 } => Ok(20415u32),
            TargetBlock { power: 7i32 } => Ok(20416u32),
            TargetBlock { power: 8i32 } => Ok(20417u32),
            TargetBlock { power: 9i32 } => Ok(20418u32),
            TargetBlock { power: 10i32 } => Ok(20419u32),
            TargetBlock { power: 11i32 } => Ok(20420u32),
            TargetBlock { power: 12i32 } => Ok(20421u32),
            TargetBlock { power: 13i32 } => Ok(20422u32),
            TargetBlock { power: 14i32 } => Ok(20423u32),
            TargetBlock { power: 15i32 } => Ok(20424u32),
            _ => Err(()),
        }
    }
}

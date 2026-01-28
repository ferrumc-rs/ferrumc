use crate::RespawnAnchor;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for RespawnAnchor {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            20478u32 => Ok(RespawnAnchor { charges: 0i32 }),
            20479u32 => Ok(RespawnAnchor { charges: 1i32 }),
            20480u32 => Ok(RespawnAnchor { charges: 2i32 }),
            20481u32 => Ok(RespawnAnchor { charges: 3i32 }),
            20482u32 => Ok(RespawnAnchor { charges: 4i32 }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for RespawnAnchor {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            RespawnAnchor { charges: 0i32 } => Ok(20478u32),
            RespawnAnchor { charges: 1i32 } => Ok(20479u32),
            RespawnAnchor { charges: 2i32 } => Ok(20480u32),
            RespawnAnchor { charges: 3i32 } => Ok(20481u32),
            RespawnAnchor { charges: 4i32 } => Ok(20482u32),
            _ => Err(()),
        }
    }
}

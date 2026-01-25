use crate::JigsawBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for JigsawBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            20383u32 => Ok(JigsawBlock {
                orientation: FrontAndTop::DownEast,
            }),
            20384u32 => Ok(JigsawBlock {
                orientation: FrontAndTop::DownNorth,
            }),
            20385u32 => Ok(JigsawBlock {
                orientation: FrontAndTop::DownSouth,
            }),
            20386u32 => Ok(JigsawBlock {
                orientation: FrontAndTop::DownWest,
            }),
            20387u32 => Ok(JigsawBlock {
                orientation: FrontAndTop::UpEast,
            }),
            20388u32 => Ok(JigsawBlock {
                orientation: FrontAndTop::UpNorth,
            }),
            20389u32 => Ok(JigsawBlock {
                orientation: FrontAndTop::UpSouth,
            }),
            20390u32 => Ok(JigsawBlock {
                orientation: FrontAndTop::UpWest,
            }),
            20391u32 => Ok(JigsawBlock {
                orientation: FrontAndTop::WestUp,
            }),
            20392u32 => Ok(JigsawBlock {
                orientation: FrontAndTop::EastUp,
            }),
            20393u32 => Ok(JigsawBlock {
                orientation: FrontAndTop::NorthUp,
            }),
            20394u32 => Ok(JigsawBlock {
                orientation: FrontAndTop::SouthUp,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for JigsawBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            JigsawBlock {
                orientation: FrontAndTop::DownEast,
            } => Ok(20383u32),
            JigsawBlock {
                orientation: FrontAndTop::DownNorth,
            } => Ok(20384u32),
            JigsawBlock {
                orientation: FrontAndTop::DownSouth,
            } => Ok(20385u32),
            JigsawBlock {
                orientation: FrontAndTop::DownWest,
            } => Ok(20386u32),
            JigsawBlock {
                orientation: FrontAndTop::UpEast,
            } => Ok(20387u32),
            JigsawBlock {
                orientation: FrontAndTop::UpNorth,
            } => Ok(20388u32),
            JigsawBlock {
                orientation: FrontAndTop::UpSouth,
            } => Ok(20389u32),
            JigsawBlock {
                orientation: FrontAndTop::UpWest,
            } => Ok(20390u32),
            JigsawBlock {
                orientation: FrontAndTop::WestUp,
            } => Ok(20391u32),
            JigsawBlock {
                orientation: FrontAndTop::EastUp,
            } => Ok(20392u32),
            JigsawBlock {
                orientation: FrontAndTop::NorthUp,
            } => Ok(20393u32),
            JigsawBlock {
                orientation: FrontAndTop::SouthUp,
            } => Ok(20394u32),
            _ => Err(()),
        }
    }
}

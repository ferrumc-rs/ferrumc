#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct83 {
    pub orientation: FrontAndTop,
}
impl TryInto<u32> for GeneratedStruct83 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct83 {
                orientation: FrontAndTop::DownEast,
            } => Ok(20383u32),
            GeneratedStruct83 {
                orientation: FrontAndTop::DownNorth,
            } => Ok(20384u32),
            GeneratedStruct83 {
                orientation: FrontAndTop::DownSouth,
            } => Ok(20385u32),
            GeneratedStruct83 {
                orientation: FrontAndTop::DownWest,
            } => Ok(20386u32),
            GeneratedStruct83 {
                orientation: FrontAndTop::UpEast,
            } => Ok(20387u32),
            GeneratedStruct83 {
                orientation: FrontAndTop::UpNorth,
            } => Ok(20388u32),
            GeneratedStruct83 {
                orientation: FrontAndTop::UpSouth,
            } => Ok(20389u32),
            GeneratedStruct83 {
                orientation: FrontAndTop::UpWest,
            } => Ok(20390u32),
            GeneratedStruct83 {
                orientation: FrontAndTop::WestUp,
            } => Ok(20391u32),
            GeneratedStruct83 {
                orientation: FrontAndTop::EastUp,
            } => Ok(20392u32),
            GeneratedStruct83 {
                orientation: FrontAndTop::NorthUp,
            } => Ok(20393u32),
            GeneratedStruct83 {
                orientation: FrontAndTop::SouthUp,
            } => Ok(20394u32),
            _ => Err(()),
        }
    }
}

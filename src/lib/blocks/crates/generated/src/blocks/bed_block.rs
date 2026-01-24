#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum BedBlockType {
    BlackBed,
    BlueBed,
    BrownBed,
    CyanBed,
    GrayBed,
    GreenBed,
    LightBlueBed,
    LightGrayBed,
    LimeBed,
    MagentaBed,
    OrangeBed,
    PinkBed,
    PurpleBed,
    RedBed,
    WhiteBed,
    YellowBed,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BedBlock {
    pub block_type: BedBlockType,
    pub facing: Direction,
    pub occupied: bool,
    pub part: BedPart,
}

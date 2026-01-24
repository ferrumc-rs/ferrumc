#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum DoublePlantBlockType {
    LargeFern,
    Lilac,
    Peony,
    PitcherPlant,
    RoseBush,
    Sunflower,
    TallGrass,
    TallSeagrass,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DoublePlantBlock {
    pub block_type: DoublePlantBlockType,
    pub half: DoubleBlockHalf,
}

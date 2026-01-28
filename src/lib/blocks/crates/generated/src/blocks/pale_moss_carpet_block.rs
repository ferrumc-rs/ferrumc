#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PaleMossCarpetBlock {
    pub bottom: bool,
    pub east: WallSide,
    pub north: WallSide,
    pub south: WallSide,
    pub west: WallSide,
}

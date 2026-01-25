#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TripwireBlock {
    pub attached: bool,
    pub disarmed: bool,
    pub east: bool,
    pub north: bool,
    pub powered: bool,
    pub south: bool,
    pub west: bool,
}

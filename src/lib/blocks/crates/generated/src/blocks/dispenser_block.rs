#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum DispenserBlockType {
    Dispenser,
    Dropper,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DispenserBlock {
    pub block_type: DispenserBlockType,
    pub facing: Direction,
    pub triggered: bool,
}

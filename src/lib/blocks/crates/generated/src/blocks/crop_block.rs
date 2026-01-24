#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum CropBlockType {
    Beetroots,
    Cactus,
    Carrots,
    ChorusFlower,
    Kelp,
    MelonStem,
    NetherWart,
    Potatoes,
    PumpkinStem,
    SugarCane,
    SweetBerryBush,
    TorchflowerCrop,
    TwistingVines,
    WeepingVines,
    Wheat,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CropBlock {
    pub block_type: CropBlockType,
    pub age: i32,
}

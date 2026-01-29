#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum SkullBlockType {
    CreeperHead,
    DragonHead,
    PiglinHead,
    PlayerHead,
    SkeletonSkull,
    WitherSkeletonSkull,
    ZombieHead,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SkullBlock {
    pub block_type: SkullBlockType,
    pub powered: bool,
    pub rotation: i32,
}

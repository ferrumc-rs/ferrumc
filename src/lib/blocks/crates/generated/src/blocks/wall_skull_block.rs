#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum WallSkullBlockType {
    CreeperWallHead,
    DragonWallHead,
    Observer,
    PiglinWallHead,
    PlayerWallHead,
    SkeletonWallSkull,
    WitherSkeletonWallSkull,
    ZombieWallHead,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WallSkullBlock {
    pub block_type: WallSkullBlockType,
    pub facing: Direction,
    pub powered: bool,
}

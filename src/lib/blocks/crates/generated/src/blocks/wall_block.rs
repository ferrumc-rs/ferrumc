#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum WallBlockType {
    AndesiteWall,
    BlackstoneWall,
    BrickWall,
    CobbledDeepslateWall,
    CobblestoneWall,
    DeepslateBrickWall,
    DeepslateTileWall,
    DioriteWall,
    EndStoneBrickWall,
    GraniteWall,
    MossyCobblestoneWall,
    MossyStoneBrickWall,
    MudBrickWall,
    NetherBrickWall,
    PolishedBlackstoneBrickWall,
    PolishedBlackstoneWall,
    PolishedDeepslateWall,
    PolishedTuffWall,
    PrismarineWall,
    RedNetherBrickWall,
    RedSandstoneWall,
    ResinBrickWall,
    SandstoneWall,
    StoneBrickWall,
    TuffBrickWall,
    TuffWall,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WallBlock {
    pub block_type: WallBlockType,
    pub east: WallSide,
    pub north: WallSide,
    pub south: WallSide,
    pub up: bool,
    pub waterlogged: bool,
    pub west: WallSide,
}

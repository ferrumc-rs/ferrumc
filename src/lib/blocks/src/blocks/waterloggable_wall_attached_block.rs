#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum WaterloggableWallAttachedBlockType {
    AcaciaWallHangingSign,
    AcaciaWallSign,
    AmethystCluster,
    BambooWallHangingSign,
    BambooWallSign,
    BigDripleafStem,
    BirchWallHangingSign,
    BirchWallSign,
    BrainCoralWallFan,
    BubbleCoralWallFan,
    CherryWallHangingSign,
    CherryWallSign,
    CrimsonWallHangingSign,
    CrimsonWallSign,
    DarkOakWallHangingSign,
    DarkOakWallSign,
    DeadBrainCoralWallFan,
    DeadBubbleCoralWallFan,
    DeadFireCoralWallFan,
    DeadHornCoralWallFan,
    DeadTubeCoralWallFan,
    EnderChest,
    FireCoralWallFan,
    HornCoralWallFan,
    JungleWallHangingSign,
    JungleWallSign,
    Ladder,
    LargeAmethystBud,
    MangroveWallHangingSign,
    MangroveWallSign,
    MediumAmethystBud,
    OakWallHangingSign,
    OakWallSign,
    PaleOakWallHangingSign,
    PaleOakWallSign,
    SmallAmethystBud,
    SpruceWallHangingSign,
    SpruceWallSign,
    TubeCoralWallFan,
    WarpedWallHangingSign,
    WarpedWallSign,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WaterloggableWallAttachedBlock {
    pub block_type: WaterloggableWallAttachedBlockType,
    pub facing: Direction,
    pub waterlogged: bool,
}
impl WaterloggableWallAttachedBlock {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<WaterloggableWallAttachedBlock>();
}
impl TryFrom<u32> for WaterloggableWallAttachedBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            5730u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            5731u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            5732u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5733u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            5734u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            5735u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            5736u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            5737u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            4882u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            4883u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            4884u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            4885u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            4886u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            4887u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            4888u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            4889u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            22061u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::North,
                waterlogged: true,
            }),
            22062u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::North,
                waterlogged: false,
            }),
            22063u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::East,
                waterlogged: true,
            }),
            22064u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::East,
                waterlogged: false,
            }),
            22065u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::South,
                waterlogged: true,
            }),
            22066u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::South,
                waterlogged: false,
            }),
            22067u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::West,
                waterlogged: true,
            }),
            22068u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::West,
                waterlogged: false,
            }),
            22069u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::Up,
                waterlogged: true,
            }),
            22070u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::Up,
                waterlogged: false,
            }),
            22071u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::Down,
                waterlogged: true,
            }),
            22072u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::Down,
                waterlogged: false,
            }),
            5794u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            5795u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            5796u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5797u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            5798u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            5799u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            5800u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            5801u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            4930u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            4931u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            4932u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            4933u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            4934u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            4935u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            4936u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            4937u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            25936u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BigDripleafStem,
                facing: Direction::North,
                waterlogged: true,
            }),
            25937u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BigDripleafStem,
                facing: Direction::North,
                waterlogged: false,
            }),
            25938u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BigDripleafStem,
                facing: Direction::South,
                waterlogged: true,
            }),
            25939u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BigDripleafStem,
                facing: Direction::South,
                waterlogged: false,
            }),
            25940u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BigDripleafStem,
                facing: Direction::West,
                waterlogged: true,
            }),
            25941u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BigDripleafStem,
                facing: Direction::West,
                waterlogged: false,
            }),
            25942u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BigDripleafStem,
                facing: Direction::East,
                waterlogged: true,
            }),
            25943u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BigDripleafStem,
                facing: Direction::East,
                waterlogged: false,
            }),
            5722u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            5723u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            5724u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5725u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            5726u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            5727u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            5728u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            5729u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            4874u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            4875u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            4876u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            4877u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            4878u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            4879u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            4880u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            4881u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            13924u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BrainCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            }),
            13925u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BrainCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            }),
            13926u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BrainCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            }),
            13927u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BrainCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            }),
            13928u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BrainCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            }),
            13929u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BrainCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            }),
            13930u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BrainCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            }),
            13931u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BrainCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            }),
            13932u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BubbleCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            }),
            13933u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BubbleCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            }),
            13934u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BubbleCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            }),
            13935u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BubbleCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            }),
            13936u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BubbleCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            }),
            13937u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BubbleCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            }),
            13938u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BubbleCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            }),
            13939u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BubbleCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            }),
            5738u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            5739u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            5740u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5741u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            5742u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            5743u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            5744u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            5745u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            4890u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            4891u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            4892u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            4893u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            4894u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            4895u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            4896u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            4897u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            5778u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            5779u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            5780u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5781u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            5782u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            5783u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            5784u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            5785u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            20363u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            20364u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            20365u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            20366u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            20367u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            20368u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            20369u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            20370u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            5754u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            5755u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            5756u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5757u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            5758u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            5759u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            5760u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            5761u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            4906u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            4907u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            4908u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            4909u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            4910u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            4911u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            4912u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            4913u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            13884u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBrainCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            }),
            13885u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBrainCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            }),
            13886u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBrainCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            }),
            13887u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBrainCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            }),
            13888u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBrainCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            }),
            13889u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBrainCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            }),
            13890u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBrainCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            }),
            13891u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBrainCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            }),
            13892u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBubbleCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            }),
            13893u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBubbleCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            }),
            13894u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBubbleCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            }),
            13895u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBubbleCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            }),
            13896u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBubbleCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            }),
            13897u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBubbleCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            }),
            13898u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBubbleCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            }),
            13899u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBubbleCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            }),
            13900u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadFireCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            }),
            13901u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadFireCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            }),
            13902u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadFireCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            }),
            13903u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadFireCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            }),
            13904u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadFireCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            }),
            13905u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadFireCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            }),
            13906u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadFireCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            }),
            13907u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadFireCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            }),
            13908u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadHornCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            }),
            13909u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadHornCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            }),
            13910u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadHornCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            }),
            13911u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadHornCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            }),
            13912u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadHornCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            }),
            13913u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadHornCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            }),
            13914u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadHornCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            }),
            13915u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadHornCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            }),
            13876u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadTubeCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            }),
            13877u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadTubeCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            }),
            13878u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadTubeCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            }),
            13879u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadTubeCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            }),
            13880u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadTubeCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            }),
            13881u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadTubeCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            }),
            13882u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadTubeCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            }),
            13883u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadTubeCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            }),
            8297u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::EnderChest,
                facing: Direction::North,
                waterlogged: true,
            }),
            8298u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::EnderChest,
                facing: Direction::North,
                waterlogged: false,
            }),
            8299u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::EnderChest,
                facing: Direction::South,
                waterlogged: true,
            }),
            8300u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::EnderChest,
                facing: Direction::South,
                waterlogged: false,
            }),
            8301u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::EnderChest,
                facing: Direction::West,
                waterlogged: true,
            }),
            8302u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::EnderChest,
                facing: Direction::West,
                waterlogged: false,
            }),
            8303u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::EnderChest,
                facing: Direction::East,
                waterlogged: true,
            }),
            8304u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::EnderChest,
                facing: Direction::East,
                waterlogged: false,
            }),
            13940u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::FireCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            }),
            13941u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::FireCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            }),
            13942u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::FireCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            }),
            13943u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::FireCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            }),
            13944u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::FireCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            }),
            13945u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::FireCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            }),
            13946u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::FireCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            }),
            13947u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::FireCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            }),
            13948u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::HornCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            }),
            13949u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::HornCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            }),
            13950u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::HornCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            }),
            13951u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::HornCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            }),
            13952u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::HornCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            }),
            13953u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::HornCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            }),
            13954u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::HornCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            }),
            13955u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::HornCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            }),
            5746u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            5747u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            5748u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5749u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            5750u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            5751u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            5752u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            5753u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            4898u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            4899u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            4900u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            4901u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            4902u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            4903u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            4904u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            4905u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            4750u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::Ladder,
                facing: Direction::North,
                waterlogged: true,
            }),
            4751u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::Ladder,
                facing: Direction::North,
                waterlogged: false,
            }),
            4752u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::Ladder,
                facing: Direction::South,
                waterlogged: true,
            }),
            4753u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::Ladder,
                facing: Direction::South,
                waterlogged: false,
            }),
            4754u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::Ladder,
                facing: Direction::West,
                waterlogged: true,
            }),
            4755u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::Ladder,
                facing: Direction::West,
                waterlogged: false,
            }),
            4756u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::Ladder,
                facing: Direction::East,
                waterlogged: true,
            }),
            4757u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::Ladder,
                facing: Direction::East,
                waterlogged: false,
            }),
            22073u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::North,
                waterlogged: true,
            }),
            22074u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::North,
                waterlogged: false,
            }),
            22075u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::East,
                waterlogged: true,
            }),
            22076u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::East,
                waterlogged: false,
            }),
            22077u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::South,
                waterlogged: true,
            }),
            22078u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::South,
                waterlogged: false,
            }),
            22079u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::West,
                waterlogged: true,
            }),
            22080u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::West,
                waterlogged: false,
            }),
            22081u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::Up,
                waterlogged: true,
            }),
            22082u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::Up,
                waterlogged: false,
            }),
            22083u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::Down,
                waterlogged: true,
            }),
            22084u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::Down,
                waterlogged: false,
            }),
            5770u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            5771u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            5772u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5773u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            5774u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            5775u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            5776u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            5777u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            4922u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            4923u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            4924u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            4925u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            4926u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            4927u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            4928u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            4929u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            22085u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::North,
                waterlogged: true,
            }),
            22086u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::North,
                waterlogged: false,
            }),
            22087u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::East,
                waterlogged: true,
            }),
            22088u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::East,
                waterlogged: false,
            }),
            22089u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::South,
                waterlogged: true,
            }),
            22090u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::South,
                waterlogged: false,
            }),
            22091u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::West,
                waterlogged: true,
            }),
            22092u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::West,
                waterlogged: false,
            }),
            22093u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::Up,
                waterlogged: true,
            }),
            22094u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::Up,
                waterlogged: false,
            }),
            22095u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::Down,
                waterlogged: true,
            }),
            22096u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::Down,
                waterlogged: false,
            }),
            5706u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            5707u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            5708u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5709u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            5710u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            5711u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            5712u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            5713u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            4858u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            4859u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            4860u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            4861u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            4862u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            4863u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            4864u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            4865u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            5762u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            5763u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            5764u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5765u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            5766u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            5767u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            5768u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            5769u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            4914u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            4915u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            4916u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            4917u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            4918u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            4919u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            4920u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            4921u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            22097u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::North,
                waterlogged: true,
            }),
            22098u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::North,
                waterlogged: false,
            }),
            22099u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::East,
                waterlogged: true,
            }),
            22100u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::East,
                waterlogged: false,
            }),
            22101u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::South,
                waterlogged: true,
            }),
            22102u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::South,
                waterlogged: false,
            }),
            22103u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::West,
                waterlogged: true,
            }),
            22104u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::West,
                waterlogged: false,
            }),
            22105u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::Up,
                waterlogged: true,
            }),
            22106u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::Up,
                waterlogged: false,
            }),
            22107u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::Down,
                waterlogged: true,
            }),
            22108u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::Down,
                waterlogged: false,
            }),
            5714u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            5715u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            5716u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5717u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            5718u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            5719u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            5720u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            5721u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            4866u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            4867u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            4868u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            4869u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            4870u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            4871u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            4872u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            4873u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            13916u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::TubeCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            }),
            13917u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::TubeCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            }),
            13918u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::TubeCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            }),
            13919u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::TubeCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            }),
            13920u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::TubeCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            }),
            13921u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::TubeCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            }),
            13922u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::TubeCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            }),
            13923u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::TubeCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            }),
            5786u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            5787u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            5788u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5789u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            5790u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            5791u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            5792u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            5793u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            20371u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            20372u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            20373u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            20374u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            20375u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            20376u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            20377u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            20378u32 => Ok(WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for WaterloggableWallAttachedBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(5730u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(5731u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5732u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(5733u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(5734u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(5735u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(5736u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(5737u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(4882u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(4883u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(4884u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(4885u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(4886u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(4887u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(4888u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AcaciaWallSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(4889u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(22061u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(22062u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(22063u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(22064u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(22065u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(22066u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(22067u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(22068u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::Up,
                waterlogged: true,
            } => Ok(22069u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::Up,
                waterlogged: false,
            } => Ok(22070u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::Down,
                waterlogged: true,
            } => Ok(22071u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::AmethystCluster,
                facing: Direction::Down,
                waterlogged: false,
            } => Ok(22072u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(5794u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(5795u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5796u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(5797u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(5798u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(5799u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(5800u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(5801u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(4930u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(4931u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(4932u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(4933u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(4934u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(4935u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(4936u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BambooWallSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(4937u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BigDripleafStem,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(25936u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BigDripleafStem,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(25937u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BigDripleafStem,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(25938u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BigDripleafStem,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(25939u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BigDripleafStem,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(25940u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BigDripleafStem,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(25941u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BigDripleafStem,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(25942u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BigDripleafStem,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(25943u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(5722u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(5723u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5724u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(5725u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(5726u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(5727u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(5728u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(5729u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(4874u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(4875u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(4876u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(4877u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(4878u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(4879u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(4880u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BirchWallSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(4881u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BrainCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(13924u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BrainCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(13925u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BrainCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(13926u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BrainCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(13927u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BrainCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(13928u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BrainCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(13929u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BrainCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(13930u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BrainCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(13931u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BubbleCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(13932u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BubbleCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(13933u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BubbleCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(13934u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BubbleCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(13935u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BubbleCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(13936u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BubbleCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(13937u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BubbleCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(13938u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::BubbleCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(13939u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(5738u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(5739u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5740u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(5741u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(5742u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(5743u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(5744u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(5745u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(4890u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(4891u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(4892u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(4893u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(4894u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(4895u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(4896u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CherryWallSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(4897u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(5778u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(5779u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5780u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(5781u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(5782u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(5783u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(5784u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(5785u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(20363u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(20364u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(20365u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(20366u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(20367u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(20368u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(20369u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::CrimsonWallSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(20370u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(5754u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(5755u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5756u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(5757u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(5758u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(5759u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(5760u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(5761u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(4906u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(4907u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(4908u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(4909u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(4910u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(4911u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(4912u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DarkOakWallSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(4913u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBrainCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(13884u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBrainCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(13885u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBrainCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(13886u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBrainCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(13887u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBrainCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(13888u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBrainCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(13889u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBrainCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(13890u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBrainCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(13891u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBubbleCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(13892u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBubbleCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(13893u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBubbleCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(13894u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBubbleCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(13895u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBubbleCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(13896u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBubbleCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(13897u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBubbleCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(13898u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadBubbleCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(13899u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadFireCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(13900u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadFireCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(13901u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadFireCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(13902u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadFireCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(13903u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadFireCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(13904u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadFireCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(13905u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadFireCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(13906u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadFireCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(13907u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadHornCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(13908u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadHornCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(13909u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadHornCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(13910u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadHornCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(13911u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadHornCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(13912u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadHornCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(13913u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadHornCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(13914u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadHornCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(13915u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadTubeCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(13876u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadTubeCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(13877u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadTubeCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(13878u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadTubeCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(13879u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadTubeCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(13880u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadTubeCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(13881u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadTubeCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(13882u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::DeadTubeCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(13883u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::EnderChest,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(8297u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::EnderChest,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(8298u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::EnderChest,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(8299u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::EnderChest,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(8300u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::EnderChest,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(8301u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::EnderChest,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(8302u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::EnderChest,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(8303u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::EnderChest,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(8304u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::FireCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(13940u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::FireCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(13941u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::FireCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(13942u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::FireCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(13943u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::FireCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(13944u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::FireCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(13945u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::FireCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(13946u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::FireCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(13947u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::HornCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(13948u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::HornCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(13949u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::HornCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(13950u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::HornCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(13951u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::HornCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(13952u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::HornCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(13953u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::HornCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(13954u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::HornCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(13955u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(5746u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(5747u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5748u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(5749u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(5750u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(5751u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(5752u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(5753u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(4898u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(4899u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(4900u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(4901u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(4902u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(4903u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(4904u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::JungleWallSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(4905u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::Ladder,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(4750u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::Ladder,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(4751u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::Ladder,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(4752u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::Ladder,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(4753u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::Ladder,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(4754u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::Ladder,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(4755u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::Ladder,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(4756u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::Ladder,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(4757u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(22073u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(22074u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(22075u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(22076u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(22077u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(22078u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(22079u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(22080u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::Up,
                waterlogged: true,
            } => Ok(22081u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::Up,
                waterlogged: false,
            } => Ok(22082u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::Down,
                waterlogged: true,
            } => Ok(22083u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::LargeAmethystBud,
                facing: Direction::Down,
                waterlogged: false,
            } => Ok(22084u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(5770u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(5771u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5772u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(5773u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(5774u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(5775u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(5776u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(5777u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(4922u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(4923u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(4924u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(4925u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(4926u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(4927u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(4928u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MangroveWallSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(4929u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(22085u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(22086u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(22087u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(22088u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(22089u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(22090u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(22091u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(22092u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::Up,
                waterlogged: true,
            } => Ok(22093u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::Up,
                waterlogged: false,
            } => Ok(22094u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::Down,
                waterlogged: true,
            } => Ok(22095u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::MediumAmethystBud,
                facing: Direction::Down,
                waterlogged: false,
            } => Ok(22096u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(5706u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(5707u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5708u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(5709u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(5710u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(5711u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(5712u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(5713u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(4858u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(4859u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(4860u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(4861u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(4862u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(4863u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(4864u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::OakWallSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(4865u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(5762u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(5763u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5764u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(5765u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(5766u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(5767u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(5768u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(5769u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(4914u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(4915u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(4916u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(4917u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(4918u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(4919u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(4920u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::PaleOakWallSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(4921u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(22097u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(22098u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(22099u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(22100u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(22101u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(22102u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(22103u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(22104u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::Up,
                waterlogged: true,
            } => Ok(22105u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::Up,
                waterlogged: false,
            } => Ok(22106u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::Down,
                waterlogged: true,
            } => Ok(22107u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SmallAmethystBud,
                facing: Direction::Down,
                waterlogged: false,
            } => Ok(22108u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(5714u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(5715u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5716u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(5717u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(5718u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(5719u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(5720u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(5721u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(4866u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(4867u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(4868u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(4869u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(4870u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(4871u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(4872u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::SpruceWallSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(4873u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::TubeCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(13916u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::TubeCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(13917u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::TubeCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(13918u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::TubeCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(13919u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::TubeCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(13920u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::TubeCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(13921u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::TubeCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(13922u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::TubeCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(13923u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(5786u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(5787u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5788u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(5789u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(5790u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(5791u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(5792u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(5793u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(20371u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(20372u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(20373u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(20374u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(20375u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(20376u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(20377u32),
            WaterloggableWallAttachedBlock {
                block_type: WaterloggableWallAttachedBlockType::WarpedWallSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(20378u32),
            _ => Err(()),
        }
    }
}

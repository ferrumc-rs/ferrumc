#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub enum GeneratedStruct66Type {
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
pub struct GeneratedStruct66 {
    pub block_type: GeneratedStruct66Type,
    pub facing: Direction,
    pub waterlogged: bool,
}
impl TryFrom<u32> for GeneratedStruct66 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            5730u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallHangingSign,
                waterlogged: true,
                facing: Direction::North,
            }),
            5731u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallHangingSign,
                waterlogged: false,
                facing: Direction::North,
            }),
            5732u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5733u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            5734u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            5735u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            5736u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallHangingSign,
                waterlogged: true,
                facing: Direction::East,
            }),
            5737u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallHangingSign,
                waterlogged: false,
                facing: Direction::East,
            }),
            4882u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallSign,
                waterlogged: true,
                facing: Direction::North,
            }),
            4883u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            4884u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallSign,
                waterlogged: true,
                facing: Direction::South,
            }),
            4885u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            4886u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            4887u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallSign,
                waterlogged: false,
                facing: Direction::West,
            }),
            4888u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            4889u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallSign,
                waterlogged: false,
                facing: Direction::East,
            }),
            22061u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                waterlogged: true,
                facing: Direction::North,
            }),
            22062u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                facing: Direction::North,
                waterlogged: false,
            }),
            22063u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                facing: Direction::East,
                waterlogged: true,
            }),
            22064u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                facing: Direction::East,
                waterlogged: false,
            }),
            22065u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                waterlogged: true,
                facing: Direction::South,
            }),
            22066u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                facing: Direction::South,
                waterlogged: false,
            }),
            22067u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                facing: Direction::West,
                waterlogged: true,
            }),
            22068u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                waterlogged: false,
                facing: Direction::West,
            }),
            22069u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                waterlogged: true,
                facing: Direction::Up,
            }),
            22070u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                facing: Direction::Up,
                waterlogged: false,
            }),
            22071u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                waterlogged: true,
                facing: Direction::Down,
            }),
            22072u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                waterlogged: false,
                facing: Direction::Down,
            }),
            5794u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            5795u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            5796u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5797u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            5798u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            5799u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            5800u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallHangingSign,
                waterlogged: true,
                facing: Direction::East,
            }),
            5801u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            4930u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            4931u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallSign,
                waterlogged: false,
                facing: Direction::North,
            }),
            4932u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            4933u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            4934u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            4935u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            4936u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            4937u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallSign,
                waterlogged: false,
                facing: Direction::East,
            }),
            25936u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BigDripleafStem,
                facing: Direction::North,
                waterlogged: true,
            }),
            25937u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BigDripleafStem,
                facing: Direction::North,
                waterlogged: false,
            }),
            25938u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BigDripleafStem,
                facing: Direction::South,
                waterlogged: true,
            }),
            25939u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BigDripleafStem,
                facing: Direction::South,
                waterlogged: false,
            }),
            25940u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BigDripleafStem,
                facing: Direction::West,
                waterlogged: true,
            }),
            25941u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BigDripleafStem,
                waterlogged: false,
                facing: Direction::West,
            }),
            25942u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BigDripleafStem,
                facing: Direction::East,
                waterlogged: true,
            }),
            25943u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BigDripleafStem,
                waterlogged: false,
                facing: Direction::East,
            }),
            5722u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallHangingSign,
                waterlogged: true,
                facing: Direction::North,
            }),
            5723u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallHangingSign,
                waterlogged: false,
                facing: Direction::North,
            }),
            5724u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallHangingSign,
                waterlogged: true,
                facing: Direction::South,
            }),
            5725u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            5726u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallHangingSign,
                waterlogged: true,
                facing: Direction::West,
            }),
            5727u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallHangingSign,
                waterlogged: false,
                facing: Direction::West,
            }),
            5728u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallHangingSign,
                waterlogged: true,
                facing: Direction::East,
            }),
            5729u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallHangingSign,
                waterlogged: false,
                facing: Direction::East,
            }),
            4874u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            4875u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            4876u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            4877u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            4878u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallSign,
                waterlogged: true,
                facing: Direction::West,
            }),
            4879u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallSign,
                waterlogged: false,
                facing: Direction::West,
            }),
            4880u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            4881u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            13924u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BrainCoralWallFan,
                waterlogged: true,
                facing: Direction::North,
            }),
            13925u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BrainCoralWallFan,
                waterlogged: false,
                facing: Direction::North,
            }),
            13926u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BrainCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            }),
            13927u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BrainCoralWallFan,
                waterlogged: false,
                facing: Direction::South,
            }),
            13928u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BrainCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            }),
            13929u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BrainCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            }),
            13930u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BrainCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            }),
            13931u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BrainCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            }),
            13932u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BubbleCoralWallFan,
                waterlogged: true,
                facing: Direction::North,
            }),
            13933u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BubbleCoralWallFan,
                waterlogged: false,
                facing: Direction::North,
            }),
            13934u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BubbleCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            }),
            13935u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BubbleCoralWallFan,
                waterlogged: false,
                facing: Direction::South,
            }),
            13936u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BubbleCoralWallFan,
                waterlogged: true,
                facing: Direction::West,
            }),
            13937u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BubbleCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            }),
            13938u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BubbleCoralWallFan,
                waterlogged: true,
                facing: Direction::East,
            }),
            13939u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BubbleCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            }),
            5738u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            5739u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            5740u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5741u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallHangingSign,
                waterlogged: false,
                facing: Direction::South,
            }),
            5742u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallHangingSign,
                waterlogged: true,
                facing: Direction::West,
            }),
            5743u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            5744u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            5745u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            4890u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallSign,
                waterlogged: true,
                facing: Direction::North,
            }),
            4891u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            4892u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            4893u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            4894u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            4895u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            4896u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            4897u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallSign,
                waterlogged: false,
                facing: Direction::East,
            }),
            5778u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallHangingSign,
                waterlogged: true,
                facing: Direction::North,
            }),
            5779u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            5780u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5781u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            5782u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallHangingSign,
                waterlogged: true,
                facing: Direction::West,
            }),
            5783u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallHangingSign,
                waterlogged: false,
                facing: Direction::West,
            }),
            5784u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallHangingSign,
                waterlogged: true,
                facing: Direction::East,
            }),
            5785u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            20363u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            20364u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallSign,
                waterlogged: false,
                facing: Direction::North,
            }),
            20365u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallSign,
                waterlogged: true,
                facing: Direction::South,
            }),
            20366u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            20367u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallSign,
                waterlogged: true,
                facing: Direction::West,
            }),
            20368u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallSign,
                waterlogged: false,
                facing: Direction::West,
            }),
            20369u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            20370u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            5754u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallHangingSign,
                waterlogged: true,
                facing: Direction::North,
            }),
            5755u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            5756u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5757u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallHangingSign,
                waterlogged: false,
                facing: Direction::South,
            }),
            5758u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallHangingSign,
                waterlogged: true,
                facing: Direction::West,
            }),
            5759u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            5760u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            5761u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            4906u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            4907u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            4908u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            4909u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            4910u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallSign,
                waterlogged: true,
                facing: Direction::West,
            }),
            4911u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            4912u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            4913u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            13884u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBrainCoralWallFan,
                waterlogged: true,
                facing: Direction::North,
            }),
            13885u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBrainCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            }),
            13886u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBrainCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            }),
            13887u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBrainCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            }),
            13888u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBrainCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            }),
            13889u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBrainCoralWallFan,
                waterlogged: false,
                facing: Direction::West,
            }),
            13890u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBrainCoralWallFan,
                waterlogged: true,
                facing: Direction::East,
            }),
            13891u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBrainCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            }),
            13892u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBubbleCoralWallFan,
                waterlogged: true,
                facing: Direction::North,
            }),
            13893u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBubbleCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            }),
            13894u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBubbleCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            }),
            13895u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBubbleCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            }),
            13896u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBubbleCoralWallFan,
                waterlogged: true,
                facing: Direction::West,
            }),
            13897u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBubbleCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            }),
            13898u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBubbleCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            }),
            13899u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBubbleCoralWallFan,
                waterlogged: false,
                facing: Direction::East,
            }),
            13900u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadFireCoralWallFan,
                waterlogged: true,
                facing: Direction::North,
            }),
            13901u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadFireCoralWallFan,
                waterlogged: false,
                facing: Direction::North,
            }),
            13902u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadFireCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            }),
            13903u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadFireCoralWallFan,
                waterlogged: false,
                facing: Direction::South,
            }),
            13904u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadFireCoralWallFan,
                waterlogged: true,
                facing: Direction::West,
            }),
            13905u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadFireCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            }),
            13906u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadFireCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            }),
            13907u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadFireCoralWallFan,
                waterlogged: false,
                facing: Direction::East,
            }),
            13908u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadHornCoralWallFan,
                waterlogged: true,
                facing: Direction::North,
            }),
            13909u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadHornCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            }),
            13910u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadHornCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            }),
            13911u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadHornCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            }),
            13912u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadHornCoralWallFan,
                waterlogged: true,
                facing: Direction::West,
            }),
            13913u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadHornCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            }),
            13914u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadHornCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            }),
            13915u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadHornCoralWallFan,
                waterlogged: false,
                facing: Direction::East,
            }),
            13876u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadTubeCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            }),
            13877u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadTubeCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            }),
            13878u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadTubeCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            }),
            13879u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadTubeCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            }),
            13880u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadTubeCoralWallFan,
                waterlogged: true,
                facing: Direction::West,
            }),
            13881u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadTubeCoralWallFan,
                waterlogged: false,
                facing: Direction::West,
            }),
            13882u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadTubeCoralWallFan,
                waterlogged: true,
                facing: Direction::East,
            }),
            13883u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadTubeCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            }),
            8297u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::EnderChest,
                waterlogged: true,
                facing: Direction::North,
            }),
            8298u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::EnderChest,
                facing: Direction::North,
                waterlogged: false,
            }),
            8299u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::EnderChest,
                waterlogged: true,
                facing: Direction::South,
            }),
            8300u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::EnderChest,
                waterlogged: false,
                facing: Direction::South,
            }),
            8301u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::EnderChest,
                facing: Direction::West,
                waterlogged: true,
            }),
            8302u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::EnderChest,
                waterlogged: false,
                facing: Direction::West,
            }),
            8303u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::EnderChest,
                waterlogged: true,
                facing: Direction::East,
            }),
            8304u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::EnderChest,
                facing: Direction::East,
                waterlogged: false,
            }),
            13940u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::FireCoralWallFan,
                waterlogged: true,
                facing: Direction::North,
            }),
            13941u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::FireCoralWallFan,
                waterlogged: false,
                facing: Direction::North,
            }),
            13942u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::FireCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            }),
            13943u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::FireCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            }),
            13944u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::FireCoralWallFan,
                waterlogged: true,
                facing: Direction::West,
            }),
            13945u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::FireCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            }),
            13946u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::FireCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            }),
            13947u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::FireCoralWallFan,
                waterlogged: false,
                facing: Direction::East,
            }),
            13948u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::HornCoralWallFan,
                waterlogged: true,
                facing: Direction::North,
            }),
            13949u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::HornCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            }),
            13950u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::HornCoralWallFan,
                waterlogged: true,
                facing: Direction::South,
            }),
            13951u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::HornCoralWallFan,
                waterlogged: false,
                facing: Direction::South,
            }),
            13952u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::HornCoralWallFan,
                waterlogged: true,
                facing: Direction::West,
            }),
            13953u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::HornCoralWallFan,
                waterlogged: false,
                facing: Direction::West,
            }),
            13954u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::HornCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            }),
            13955u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::HornCoralWallFan,
                waterlogged: false,
                facing: Direction::East,
            }),
            5746u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            5747u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            5748u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5749u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallHangingSign,
                waterlogged: false,
                facing: Direction::South,
            }),
            5750u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            5751u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            5752u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            5753u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallHangingSign,
                waterlogged: false,
                facing: Direction::East,
            }),
            4898u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            4899u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallSign,
                waterlogged: false,
                facing: Direction::North,
            }),
            4900u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            4901u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallSign,
                waterlogged: false,
                facing: Direction::South,
            }),
            4902u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallSign,
                waterlogged: true,
                facing: Direction::West,
            }),
            4903u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            4904u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallSign,
                waterlogged: true,
                facing: Direction::East,
            }),
            4905u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            4750u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::Ladder,
                facing: Direction::North,
                waterlogged: true,
            }),
            4751u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::Ladder,
                facing: Direction::North,
                waterlogged: false,
            }),
            4752u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::Ladder,
                waterlogged: true,
                facing: Direction::South,
            }),
            4753u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::Ladder,
                waterlogged: false,
                facing: Direction::South,
            }),
            4754u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::Ladder,
                waterlogged: true,
                facing: Direction::West,
            }),
            4755u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::Ladder,
                facing: Direction::West,
                waterlogged: false,
            }),
            4756u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::Ladder,
                facing: Direction::East,
                waterlogged: true,
            }),
            4757u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::Ladder,
                facing: Direction::East,
                waterlogged: false,
            }),
            22073u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                waterlogged: true,
                facing: Direction::North,
            }),
            22074u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                facing: Direction::North,
                waterlogged: false,
            }),
            22075u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                facing: Direction::East,
                waterlogged: true,
            }),
            22076u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                facing: Direction::East,
                waterlogged: false,
            }),
            22077u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                waterlogged: true,
                facing: Direction::South,
            }),
            22078u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                facing: Direction::South,
                waterlogged: false,
            }),
            22079u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                facing: Direction::West,
                waterlogged: true,
            }),
            22080u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                facing: Direction::West,
                waterlogged: false,
            }),
            22081u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                waterlogged: true,
                facing: Direction::Up,
            }),
            22082u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                facing: Direction::Up,
                waterlogged: false,
            }),
            22083u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                facing: Direction::Down,
                waterlogged: true,
            }),
            22084u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                waterlogged: false,
                facing: Direction::Down,
            }),
            5770u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            5771u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            5772u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5773u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            5774u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            5775u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallHangingSign,
                waterlogged: false,
                facing: Direction::West,
            }),
            5776u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            5777u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallHangingSign,
                waterlogged: false,
                facing: Direction::East,
            }),
            4922u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            4923u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            4924u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallSign,
                waterlogged: true,
                facing: Direction::South,
            }),
            4925u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            4926u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            4927u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            4928u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallSign,
                waterlogged: true,
                facing: Direction::East,
            }),
            4929u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallSign,
                waterlogged: false,
                facing: Direction::East,
            }),
            22085u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                facing: Direction::North,
                waterlogged: true,
            }),
            22086u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                facing: Direction::North,
                waterlogged: false,
            }),
            22087u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                facing: Direction::East,
                waterlogged: true,
            }),
            22088u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                waterlogged: false,
                facing: Direction::East,
            }),
            22089u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                facing: Direction::South,
                waterlogged: true,
            }),
            22090u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                facing: Direction::South,
                waterlogged: false,
            }),
            22091u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                facing: Direction::West,
                waterlogged: true,
            }),
            22092u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                facing: Direction::West,
                waterlogged: false,
            }),
            22093u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                facing: Direction::Up,
                waterlogged: true,
            }),
            22094u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                waterlogged: false,
                facing: Direction::Up,
            }),
            22095u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                facing: Direction::Down,
                waterlogged: true,
            }),
            22096u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                facing: Direction::Down,
                waterlogged: false,
            }),
            5706u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallHangingSign,
                waterlogged: true,
                facing: Direction::North,
            }),
            5707u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            5708u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5709u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallHangingSign,
                waterlogged: false,
                facing: Direction::South,
            }),
            5710u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallHangingSign,
                waterlogged: true,
                facing: Direction::West,
            }),
            5711u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallHangingSign,
                waterlogged: false,
                facing: Direction::West,
            }),
            5712u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            5713u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallHangingSign,
                waterlogged: false,
                facing: Direction::East,
            }),
            4858u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallSign,
                waterlogged: true,
                facing: Direction::North,
            }),
            4859u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallSign,
                waterlogged: false,
                facing: Direction::North,
            }),
            4860u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            4861u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            4862u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            4863u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            4864u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            4865u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallSign,
                waterlogged: false,
                facing: Direction::East,
            }),
            5762u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            5763u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            5764u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5765u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            5766u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            5767u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            5768u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallHangingSign,
                waterlogged: true,
                facing: Direction::East,
            }),
            5769u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            4914u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            4915u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallSign,
                waterlogged: false,
                facing: Direction::North,
            }),
            4916u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            4917u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            4918u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            4919u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallSign,
                waterlogged: false,
                facing: Direction::West,
            }),
            4920u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallSign,
                waterlogged: true,
                facing: Direction::East,
            }),
            4921u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallSign,
                waterlogged: false,
                facing: Direction::East,
            }),
            22097u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                waterlogged: true,
                facing: Direction::North,
            }),
            22098u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                waterlogged: false,
                facing: Direction::North,
            }),
            22099u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                waterlogged: true,
                facing: Direction::East,
            }),
            22100u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                waterlogged: false,
                facing: Direction::East,
            }),
            22101u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                facing: Direction::South,
                waterlogged: true,
            }),
            22102u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                facing: Direction::South,
                waterlogged: false,
            }),
            22103u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                waterlogged: true,
                facing: Direction::West,
            }),
            22104u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                waterlogged: false,
                facing: Direction::West,
            }),
            22105u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                facing: Direction::Up,
                waterlogged: true,
            }),
            22106u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                facing: Direction::Up,
                waterlogged: false,
            }),
            22107u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                facing: Direction::Down,
                waterlogged: true,
            }),
            22108u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                waterlogged: false,
                facing: Direction::Down,
            }),
            5714u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallHangingSign,
                waterlogged: true,
                facing: Direction::North,
            }),
            5715u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            5716u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            }),
            5717u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            }),
            5718u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            5719u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallHangingSign,
                waterlogged: false,
                facing: Direction::West,
            }),
            5720u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallHangingSign,
                waterlogged: true,
                facing: Direction::East,
            }),
            5721u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallHangingSign,
                waterlogged: false,
                facing: Direction::East,
            }),
            4866u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            4867u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallSign,
                facing: Direction::North,
                waterlogged: false,
            }),
            4868u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallSign,
                waterlogged: true,
                facing: Direction::South,
            }),
            4869u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallSign,
                waterlogged: false,
                facing: Direction::South,
            }),
            4870u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            4871u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallSign,
                waterlogged: false,
                facing: Direction::West,
            }),
            4872u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            4873u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallSign,
                waterlogged: false,
                facing: Direction::East,
            }),
            13916u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::TubeCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            }),
            13917u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::TubeCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            }),
            13918u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::TubeCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            }),
            13919u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::TubeCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            }),
            13920u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::TubeCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            }),
            13921u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::TubeCoralWallFan,
                waterlogged: false,
                facing: Direction::West,
            }),
            13922u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::TubeCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            }),
            13923u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::TubeCoralWallFan,
                waterlogged: false,
                facing: Direction::East,
            }),
            5786u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            5787u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallHangingSign,
                waterlogged: false,
                facing: Direction::North,
            }),
            5788u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallHangingSign,
                waterlogged: true,
                facing: Direction::South,
            }),
            5789u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallHangingSign,
                waterlogged: false,
                facing: Direction::South,
            }),
            5790u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            }),
            5791u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            }),
            5792u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            }),
            5793u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            20371u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallSign,
                facing: Direction::North,
                waterlogged: true,
            }),
            20372u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallSign,
                waterlogged: false,
                facing: Direction::North,
            }),
            20373u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallSign,
                waterlogged: true,
                facing: Direction::South,
            }),
            20374u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallSign,
                waterlogged: false,
                facing: Direction::South,
            }),
            20375u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallSign,
                waterlogged: true,
                facing: Direction::West,
            }),
            20376u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallSign,
                waterlogged: false,
                facing: Direction::West,
            }),
            20377u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallSign,
                waterlogged: true,
                facing: Direction::East,
            }),
            20378u32 => Ok(GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallSign,
                facing: Direction::East,
                waterlogged: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct66 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallHangingSign,
                waterlogged: true,
                facing: Direction::North,
            } => Ok(5730u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallHangingSign,
                waterlogged: false,
                facing: Direction::North,
            } => Ok(5731u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5732u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(5733u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(5734u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(5735u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallHangingSign,
                waterlogged: true,
                facing: Direction::East,
            } => Ok(5736u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallHangingSign,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(5737u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallSign,
                waterlogged: true,
                facing: Direction::North,
            } => Ok(4882u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(4883u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallSign,
                waterlogged: true,
                facing: Direction::South,
            } => Ok(4884u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(4885u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(4886u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallSign,
                waterlogged: false,
                facing: Direction::West,
            } => Ok(4887u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(4888u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AcaciaWallSign,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(4889u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                waterlogged: true,
                facing: Direction::North,
            } => Ok(22061u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(22062u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(22063u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(22064u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                waterlogged: true,
                facing: Direction::South,
            } => Ok(22065u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(22066u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(22067u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                waterlogged: false,
                facing: Direction::West,
            } => Ok(22068u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                waterlogged: true,
                facing: Direction::Up,
            } => Ok(22069u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                facing: Direction::Up,
                waterlogged: false,
            } => Ok(22070u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                waterlogged: true,
                facing: Direction::Down,
            } => Ok(22071u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::AmethystCluster,
                waterlogged: false,
                facing: Direction::Down,
            } => Ok(22072u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(5794u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(5795u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5796u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(5797u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(5798u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(5799u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallHangingSign,
                waterlogged: true,
                facing: Direction::East,
            } => Ok(5800u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(5801u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(4930u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallSign,
                waterlogged: false,
                facing: Direction::North,
            } => Ok(4931u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(4932u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(4933u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(4934u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(4935u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(4936u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BambooWallSign,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(4937u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BigDripleafStem,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(25936u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BigDripleafStem,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(25937u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BigDripleafStem,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(25938u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BigDripleafStem,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(25939u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BigDripleafStem,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(25940u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BigDripleafStem,
                waterlogged: false,
                facing: Direction::West,
            } => Ok(25941u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BigDripleafStem,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(25942u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BigDripleafStem,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(25943u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallHangingSign,
                waterlogged: true,
                facing: Direction::North,
            } => Ok(5722u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallHangingSign,
                waterlogged: false,
                facing: Direction::North,
            } => Ok(5723u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallHangingSign,
                waterlogged: true,
                facing: Direction::South,
            } => Ok(5724u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(5725u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallHangingSign,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(5726u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallHangingSign,
                waterlogged: false,
                facing: Direction::West,
            } => Ok(5727u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallHangingSign,
                waterlogged: true,
                facing: Direction::East,
            } => Ok(5728u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallHangingSign,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(5729u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(4874u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(4875u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(4876u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(4877u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallSign,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(4878u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallSign,
                waterlogged: false,
                facing: Direction::West,
            } => Ok(4879u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(4880u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BirchWallSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(4881u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BrainCoralWallFan,
                waterlogged: true,
                facing: Direction::North,
            } => Ok(13924u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BrainCoralWallFan,
                waterlogged: false,
                facing: Direction::North,
            } => Ok(13925u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BrainCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(13926u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BrainCoralWallFan,
                waterlogged: false,
                facing: Direction::South,
            } => Ok(13927u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BrainCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(13928u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BrainCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(13929u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BrainCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(13930u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BrainCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(13931u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BubbleCoralWallFan,
                waterlogged: true,
                facing: Direction::North,
            } => Ok(13932u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BubbleCoralWallFan,
                waterlogged: false,
                facing: Direction::North,
            } => Ok(13933u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BubbleCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(13934u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BubbleCoralWallFan,
                waterlogged: false,
                facing: Direction::South,
            } => Ok(13935u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BubbleCoralWallFan,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(13936u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BubbleCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(13937u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BubbleCoralWallFan,
                waterlogged: true,
                facing: Direction::East,
            } => Ok(13938u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::BubbleCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(13939u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(5738u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(5739u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5740u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallHangingSign,
                waterlogged: false,
                facing: Direction::South,
            } => Ok(5741u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallHangingSign,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(5742u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(5743u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(5744u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(5745u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallSign,
                waterlogged: true,
                facing: Direction::North,
            } => Ok(4890u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(4891u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(4892u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(4893u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(4894u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(4895u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(4896u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CherryWallSign,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(4897u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallHangingSign,
                waterlogged: true,
                facing: Direction::North,
            } => Ok(5778u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(5779u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5780u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(5781u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallHangingSign,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(5782u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallHangingSign,
                waterlogged: false,
                facing: Direction::West,
            } => Ok(5783u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallHangingSign,
                waterlogged: true,
                facing: Direction::East,
            } => Ok(5784u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(5785u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(20363u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallSign,
                waterlogged: false,
                facing: Direction::North,
            } => Ok(20364u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallSign,
                waterlogged: true,
                facing: Direction::South,
            } => Ok(20365u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(20366u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallSign,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(20367u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallSign,
                waterlogged: false,
                facing: Direction::West,
            } => Ok(20368u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(20369u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::CrimsonWallSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(20370u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallHangingSign,
                waterlogged: true,
                facing: Direction::North,
            } => Ok(5754u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(5755u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5756u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallHangingSign,
                waterlogged: false,
                facing: Direction::South,
            } => Ok(5757u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallHangingSign,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(5758u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(5759u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(5760u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(5761u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(4906u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(4907u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(4908u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(4909u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallSign,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(4910u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(4911u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(4912u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DarkOakWallSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(4913u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBrainCoralWallFan,
                waterlogged: true,
                facing: Direction::North,
            } => Ok(13884u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBrainCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(13885u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBrainCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(13886u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBrainCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(13887u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBrainCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(13888u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBrainCoralWallFan,
                waterlogged: false,
                facing: Direction::West,
            } => Ok(13889u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBrainCoralWallFan,
                waterlogged: true,
                facing: Direction::East,
            } => Ok(13890u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBrainCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(13891u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBubbleCoralWallFan,
                waterlogged: true,
                facing: Direction::North,
            } => Ok(13892u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBubbleCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(13893u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBubbleCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(13894u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBubbleCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(13895u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBubbleCoralWallFan,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(13896u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBubbleCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(13897u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBubbleCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(13898u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadBubbleCoralWallFan,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(13899u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadFireCoralWallFan,
                waterlogged: true,
                facing: Direction::North,
            } => Ok(13900u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadFireCoralWallFan,
                waterlogged: false,
                facing: Direction::North,
            } => Ok(13901u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadFireCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(13902u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadFireCoralWallFan,
                waterlogged: false,
                facing: Direction::South,
            } => Ok(13903u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadFireCoralWallFan,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(13904u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadFireCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(13905u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadFireCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(13906u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadFireCoralWallFan,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(13907u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadHornCoralWallFan,
                waterlogged: true,
                facing: Direction::North,
            } => Ok(13908u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadHornCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(13909u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadHornCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(13910u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadHornCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(13911u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadHornCoralWallFan,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(13912u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadHornCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(13913u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadHornCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(13914u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadHornCoralWallFan,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(13915u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadTubeCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(13876u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadTubeCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(13877u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadTubeCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(13878u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadTubeCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(13879u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadTubeCoralWallFan,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(13880u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadTubeCoralWallFan,
                waterlogged: false,
                facing: Direction::West,
            } => Ok(13881u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadTubeCoralWallFan,
                waterlogged: true,
                facing: Direction::East,
            } => Ok(13882u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::DeadTubeCoralWallFan,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(13883u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::EnderChest,
                waterlogged: true,
                facing: Direction::North,
            } => Ok(8297u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::EnderChest,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(8298u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::EnderChest,
                waterlogged: true,
                facing: Direction::South,
            } => Ok(8299u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::EnderChest,
                waterlogged: false,
                facing: Direction::South,
            } => Ok(8300u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::EnderChest,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(8301u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::EnderChest,
                waterlogged: false,
                facing: Direction::West,
            } => Ok(8302u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::EnderChest,
                waterlogged: true,
                facing: Direction::East,
            } => Ok(8303u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::EnderChest,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(8304u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::FireCoralWallFan,
                waterlogged: true,
                facing: Direction::North,
            } => Ok(13940u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::FireCoralWallFan,
                waterlogged: false,
                facing: Direction::North,
            } => Ok(13941u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::FireCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(13942u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::FireCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(13943u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::FireCoralWallFan,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(13944u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::FireCoralWallFan,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(13945u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::FireCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(13946u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::FireCoralWallFan,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(13947u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::HornCoralWallFan,
                waterlogged: true,
                facing: Direction::North,
            } => Ok(13948u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::HornCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(13949u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::HornCoralWallFan,
                waterlogged: true,
                facing: Direction::South,
            } => Ok(13950u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::HornCoralWallFan,
                waterlogged: false,
                facing: Direction::South,
            } => Ok(13951u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::HornCoralWallFan,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(13952u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::HornCoralWallFan,
                waterlogged: false,
                facing: Direction::West,
            } => Ok(13953u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::HornCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(13954u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::HornCoralWallFan,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(13955u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(5746u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(5747u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5748u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallHangingSign,
                waterlogged: false,
                facing: Direction::South,
            } => Ok(5749u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(5750u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(5751u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(5752u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallHangingSign,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(5753u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(4898u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallSign,
                waterlogged: false,
                facing: Direction::North,
            } => Ok(4899u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(4900u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallSign,
                waterlogged: false,
                facing: Direction::South,
            } => Ok(4901u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallSign,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(4902u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(4903u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallSign,
                waterlogged: true,
                facing: Direction::East,
            } => Ok(4904u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::JungleWallSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(4905u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::Ladder,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(4750u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::Ladder,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(4751u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::Ladder,
                waterlogged: true,
                facing: Direction::South,
            } => Ok(4752u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::Ladder,
                waterlogged: false,
                facing: Direction::South,
            } => Ok(4753u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::Ladder,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(4754u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::Ladder,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(4755u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::Ladder,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(4756u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::Ladder,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(4757u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                waterlogged: true,
                facing: Direction::North,
            } => Ok(22073u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(22074u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(22075u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(22076u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                waterlogged: true,
                facing: Direction::South,
            } => Ok(22077u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(22078u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(22079u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(22080u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                waterlogged: true,
                facing: Direction::Up,
            } => Ok(22081u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                facing: Direction::Up,
                waterlogged: false,
            } => Ok(22082u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                facing: Direction::Down,
                waterlogged: true,
            } => Ok(22083u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::LargeAmethystBud,
                waterlogged: false,
                facing: Direction::Down,
            } => Ok(22084u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(5770u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(5771u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5772u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(5773u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(5774u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallHangingSign,
                waterlogged: false,
                facing: Direction::West,
            } => Ok(5775u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(5776u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallHangingSign,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(5777u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(4922u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(4923u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallSign,
                waterlogged: true,
                facing: Direction::South,
            } => Ok(4924u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(4925u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(4926u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(4927u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallSign,
                waterlogged: true,
                facing: Direction::East,
            } => Ok(4928u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MangroveWallSign,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(4929u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(22085u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(22086u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(22087u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(22088u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(22089u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(22090u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(22091u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(22092u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                facing: Direction::Up,
                waterlogged: true,
            } => Ok(22093u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                waterlogged: false,
                facing: Direction::Up,
            } => Ok(22094u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                facing: Direction::Down,
                waterlogged: true,
            } => Ok(22095u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::MediumAmethystBud,
                facing: Direction::Down,
                waterlogged: false,
            } => Ok(22096u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallHangingSign,
                waterlogged: true,
                facing: Direction::North,
            } => Ok(5706u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(5707u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5708u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallHangingSign,
                waterlogged: false,
                facing: Direction::South,
            } => Ok(5709u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallHangingSign,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(5710u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallHangingSign,
                waterlogged: false,
                facing: Direction::West,
            } => Ok(5711u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(5712u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallHangingSign,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(5713u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallSign,
                waterlogged: true,
                facing: Direction::North,
            } => Ok(4858u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallSign,
                waterlogged: false,
                facing: Direction::North,
            } => Ok(4859u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(4860u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(4861u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(4862u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(4863u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(4864u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::OakWallSign,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(4865u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(5762u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(5763u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5764u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(5765u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(5766u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(5767u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallHangingSign,
                waterlogged: true,
                facing: Direction::East,
            } => Ok(5768u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(5769u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(4914u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallSign,
                waterlogged: false,
                facing: Direction::North,
            } => Ok(4915u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(4916u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(4917u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(4918u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallSign,
                waterlogged: false,
                facing: Direction::West,
            } => Ok(4919u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallSign,
                waterlogged: true,
                facing: Direction::East,
            } => Ok(4920u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::PaleOakWallSign,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(4921u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                waterlogged: true,
                facing: Direction::North,
            } => Ok(22097u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                waterlogged: false,
                facing: Direction::North,
            } => Ok(22098u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                waterlogged: true,
                facing: Direction::East,
            } => Ok(22099u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(22100u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(22101u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(22102u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(22103u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                waterlogged: false,
                facing: Direction::West,
            } => Ok(22104u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                facing: Direction::Up,
                waterlogged: true,
            } => Ok(22105u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                facing: Direction::Up,
                waterlogged: false,
            } => Ok(22106u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                facing: Direction::Down,
                waterlogged: true,
            } => Ok(22107u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SmallAmethystBud,
                waterlogged: false,
                facing: Direction::Down,
            } => Ok(22108u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallHangingSign,
                waterlogged: true,
                facing: Direction::North,
            } => Ok(5714u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallHangingSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(5715u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallHangingSign,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(5716u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallHangingSign,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(5717u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(5718u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallHangingSign,
                waterlogged: false,
                facing: Direction::West,
            } => Ok(5719u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallHangingSign,
                waterlogged: true,
                facing: Direction::East,
            } => Ok(5720u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallHangingSign,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(5721u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(4866u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallSign,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(4867u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallSign,
                waterlogged: true,
                facing: Direction::South,
            } => Ok(4868u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallSign,
                waterlogged: false,
                facing: Direction::South,
            } => Ok(4869u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(4870u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallSign,
                waterlogged: false,
                facing: Direction::West,
            } => Ok(4871u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(4872u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::SpruceWallSign,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(4873u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::TubeCoralWallFan,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(13916u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::TubeCoralWallFan,
                facing: Direction::North,
                waterlogged: false,
            } => Ok(13917u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::TubeCoralWallFan,
                facing: Direction::South,
                waterlogged: true,
            } => Ok(13918u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::TubeCoralWallFan,
                facing: Direction::South,
                waterlogged: false,
            } => Ok(13919u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::TubeCoralWallFan,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(13920u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::TubeCoralWallFan,
                waterlogged: false,
                facing: Direction::West,
            } => Ok(13921u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::TubeCoralWallFan,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(13922u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::TubeCoralWallFan,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(13923u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallHangingSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(5786u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallHangingSign,
                waterlogged: false,
                facing: Direction::North,
            } => Ok(5787u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallHangingSign,
                waterlogged: true,
                facing: Direction::South,
            } => Ok(5788u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallHangingSign,
                waterlogged: false,
                facing: Direction::South,
            } => Ok(5789u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallHangingSign,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(5790u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallHangingSign,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(5791u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallHangingSign,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(5792u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallHangingSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(5793u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallSign,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(20371u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallSign,
                waterlogged: false,
                facing: Direction::North,
            } => Ok(20372u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallSign,
                waterlogged: true,
                facing: Direction::South,
            } => Ok(20373u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallSign,
                waterlogged: false,
                facing: Direction::South,
            } => Ok(20374u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallSign,
                waterlogged: true,
                facing: Direction::West,
            } => Ok(20375u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallSign,
                waterlogged: false,
                facing: Direction::West,
            } => Ok(20376u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallSign,
                waterlogged: true,
                facing: Direction::East,
            } => Ok(20377u32),
            GeneratedStruct66 {
                block_type: GeneratedStruct66Type::WarpedWallSign,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(20378u32),
            _ => Err(()),
        }
    }
}

use crate::FacingBlock;
use crate::FacingBlockType;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for FacingBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            9916u32 => Ok(FacingBlock {
                block_type: FacingBlockType::Anvil,
                facing: Direction::North,
            }),
            9917u32 => Ok(FacingBlock {
                block_type: FacingBlockType::Anvil,
                facing: Direction::South,
            }),
            9918u32 => Ok(FacingBlock {
                block_type: FacingBlockType::Anvil,
                facing: Direction::West,
            }),
            9919u32 => Ok(FacingBlock {
                block_type: FacingBlockType::Anvil,
                facing: Direction::East,
            }),
            7060u32 => Ok(FacingBlock {
                block_type: FacingBlockType::AttachedMelonStem,
                facing: Direction::North,
            }),
            7061u32 => Ok(FacingBlock {
                block_type: FacingBlockType::AttachedMelonStem,
                facing: Direction::South,
            }),
            7062u32 => Ok(FacingBlock {
                block_type: FacingBlockType::AttachedMelonStem,
                facing: Direction::West,
            }),
            7063u32 => Ok(FacingBlock {
                block_type: FacingBlockType::AttachedMelonStem,
                facing: Direction::East,
            }),
            7056u32 => Ok(FacingBlock {
                block_type: FacingBlockType::AttachedPumpkinStem,
                facing: Direction::North,
            }),
            7057u32 => Ok(FacingBlock {
                block_type: FacingBlockType::AttachedPumpkinStem,
                facing: Direction::South,
            }),
            7058u32 => Ok(FacingBlock {
                block_type: FacingBlockType::AttachedPumpkinStem,
                facing: Direction::West,
            }),
            7059u32 => Ok(FacingBlock {
                block_type: FacingBlockType::AttachedPumpkinStem,
                facing: Direction::East,
            }),
            13747u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlackGlazedTerracotta,
                facing: Direction::North,
            }),
            13748u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlackGlazedTerracotta,
                facing: Direction::South,
            }),
            13749u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlackGlazedTerracotta,
                facing: Direction::West,
            }),
            13750u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlackGlazedTerracotta,
                facing: Direction::East,
            }),
            13681u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlackShulkerBox,
                facing: Direction::North,
            }),
            13682u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlackShulkerBox,
                facing: Direction::East,
            }),
            13683u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlackShulkerBox,
                facing: Direction::South,
            }),
            13684u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlackShulkerBox,
                facing: Direction::West,
            }),
            13685u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlackShulkerBox,
                facing: Direction::Up,
            }),
            13686u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlackShulkerBox,
                facing: Direction::Down,
            }),
            11964u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlackWallBanner,
                facing: Direction::North,
            }),
            11965u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlackWallBanner,
                facing: Direction::South,
            }),
            11966u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlackWallBanner,
                facing: Direction::West,
            }),
            11967u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlackWallBanner,
                facing: Direction::East,
            }),
            13731u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlueGlazedTerracotta,
                facing: Direction::North,
            }),
            13732u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlueGlazedTerracotta,
                facing: Direction::South,
            }),
            13733u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlueGlazedTerracotta,
                facing: Direction::West,
            }),
            13734u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlueGlazedTerracotta,
                facing: Direction::East,
            }),
            13657u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlueShulkerBox,
                facing: Direction::North,
            }),
            13658u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlueShulkerBox,
                facing: Direction::East,
            }),
            13659u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlueShulkerBox,
                facing: Direction::South,
            }),
            13660u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlueShulkerBox,
                facing: Direction::West,
            }),
            13661u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlueShulkerBox,
                facing: Direction::Up,
            }),
            13662u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlueShulkerBox,
                facing: Direction::Down,
            }),
            11948u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlueWallBanner,
                facing: Direction::North,
            }),
            11949u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlueWallBanner,
                facing: Direction::South,
            }),
            11950u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlueWallBanner,
                facing: Direction::West,
            }),
            11951u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BlueWallBanner,
                facing: Direction::East,
            }),
            13735u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BrownGlazedTerracotta,
                facing: Direction::North,
            }),
            13736u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BrownGlazedTerracotta,
                facing: Direction::South,
            }),
            13737u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BrownGlazedTerracotta,
                facing: Direction::West,
            }),
            13738u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BrownGlazedTerracotta,
                facing: Direction::East,
            }),
            13663u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BrownShulkerBox,
                facing: Direction::North,
            }),
            13664u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BrownShulkerBox,
                facing: Direction::East,
            }),
            13665u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BrownShulkerBox,
                facing: Direction::South,
            }),
            13666u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BrownShulkerBox,
                facing: Direction::West,
            }),
            13667u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BrownShulkerBox,
                facing: Direction::Up,
            }),
            13668u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BrownShulkerBox,
                facing: Direction::Down,
            }),
            11952u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BrownWallBanner,
                facing: Direction::North,
            }),
            11953u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BrownWallBanner,
                facing: Direction::South,
            }),
            11954u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BrownWallBanner,
                facing: Direction::West,
            }),
            11955u32 => Ok(FacingBlock {
                block_type: FacingBlockType::BrownWallBanner,
                facing: Direction::East,
            }),
            6045u32 => Ok(FacingBlock {
                block_type: FacingBlockType::CarvedPumpkin,
                facing: Direction::North,
            }),
            6046u32 => Ok(FacingBlock {
                block_type: FacingBlockType::CarvedPumpkin,
                facing: Direction::South,
            }),
            6047u32 => Ok(FacingBlock {
                block_type: FacingBlockType::CarvedPumpkin,
                facing: Direction::West,
            }),
            6048u32 => Ok(FacingBlock {
                block_type: FacingBlockType::CarvedPumpkin,
                facing: Direction::East,
            }),
            9920u32 => Ok(FacingBlock {
                block_type: FacingBlockType::ChippedAnvil,
                facing: Direction::North,
            }),
            9921u32 => Ok(FacingBlock {
                block_type: FacingBlockType::ChippedAnvil,
                facing: Direction::South,
            }),
            9922u32 => Ok(FacingBlock {
                block_type: FacingBlockType::ChippedAnvil,
                facing: Direction::West,
            }),
            9923u32 => Ok(FacingBlock {
                block_type: FacingBlockType::ChippedAnvil,
                facing: Direction::East,
            }),
            13723u32 => Ok(FacingBlock {
                block_type: FacingBlockType::CyanGlazedTerracotta,
                facing: Direction::North,
            }),
            13724u32 => Ok(FacingBlock {
                block_type: FacingBlockType::CyanGlazedTerracotta,
                facing: Direction::South,
            }),
            13725u32 => Ok(FacingBlock {
                block_type: FacingBlockType::CyanGlazedTerracotta,
                facing: Direction::West,
            }),
            13726u32 => Ok(FacingBlock {
                block_type: FacingBlockType::CyanGlazedTerracotta,
                facing: Direction::East,
            }),
            13645u32 => Ok(FacingBlock {
                block_type: FacingBlockType::CyanShulkerBox,
                facing: Direction::North,
            }),
            13646u32 => Ok(FacingBlock {
                block_type: FacingBlockType::CyanShulkerBox,
                facing: Direction::East,
            }),
            13647u32 => Ok(FacingBlock {
                block_type: FacingBlockType::CyanShulkerBox,
                facing: Direction::South,
            }),
            13648u32 => Ok(FacingBlock {
                block_type: FacingBlockType::CyanShulkerBox,
                facing: Direction::West,
            }),
            13649u32 => Ok(FacingBlock {
                block_type: FacingBlockType::CyanShulkerBox,
                facing: Direction::Up,
            }),
            13650u32 => Ok(FacingBlock {
                block_type: FacingBlockType::CyanShulkerBox,
                facing: Direction::Down,
            }),
            11940u32 => Ok(FacingBlock {
                block_type: FacingBlockType::CyanWallBanner,
                facing: Direction::North,
            }),
            11941u32 => Ok(FacingBlock {
                block_type: FacingBlockType::CyanWallBanner,
                facing: Direction::South,
            }),
            11942u32 => Ok(FacingBlock {
                block_type: FacingBlockType::CyanWallBanner,
                facing: Direction::West,
            }),
            11943u32 => Ok(FacingBlock {
                block_type: FacingBlockType::CyanWallBanner,
                facing: Direction::East,
            }),
            9924u32 => Ok(FacingBlock {
                block_type: FacingBlockType::DamagedAnvil,
                facing: Direction::North,
            }),
            9925u32 => Ok(FacingBlock {
                block_type: FacingBlockType::DamagedAnvil,
                facing: Direction::South,
            }),
            9926u32 => Ok(FacingBlock {
                block_type: FacingBlockType::DamagedAnvil,
                facing: Direction::West,
            }),
            9927u32 => Ok(FacingBlock {
                block_type: FacingBlockType::DamagedAnvil,
                facing: Direction::East,
            }),
            13357u32 => Ok(FacingBlock {
                block_type: FacingBlockType::EndRod,
                facing: Direction::North,
            }),
            13358u32 => Ok(FacingBlock {
                block_type: FacingBlockType::EndRod,
                facing: Direction::East,
            }),
            13359u32 => Ok(FacingBlock {
                block_type: FacingBlockType::EndRod,
                facing: Direction::South,
            }),
            13360u32 => Ok(FacingBlock {
                block_type: FacingBlockType::EndRod,
                facing: Direction::West,
            }),
            13361u32 => Ok(FacingBlock {
                block_type: FacingBlockType::EndRod,
                facing: Direction::Up,
            }),
            13362u32 => Ok(FacingBlock {
                block_type: FacingBlockType::EndRod,
                facing: Direction::Down,
            }),
            13715u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GrayGlazedTerracotta,
                facing: Direction::North,
            }),
            13716u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GrayGlazedTerracotta,
                facing: Direction::South,
            }),
            13717u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GrayGlazedTerracotta,
                facing: Direction::West,
            }),
            13718u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GrayGlazedTerracotta,
                facing: Direction::East,
            }),
            13633u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GrayShulkerBox,
                facing: Direction::North,
            }),
            13634u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GrayShulkerBox,
                facing: Direction::East,
            }),
            13635u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GrayShulkerBox,
                facing: Direction::South,
            }),
            13636u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GrayShulkerBox,
                facing: Direction::West,
            }),
            13637u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GrayShulkerBox,
                facing: Direction::Up,
            }),
            13638u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GrayShulkerBox,
                facing: Direction::Down,
            }),
            11932u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GrayWallBanner,
                facing: Direction::North,
            }),
            11933u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GrayWallBanner,
                facing: Direction::South,
            }),
            11934u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GrayWallBanner,
                facing: Direction::West,
            }),
            11935u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GrayWallBanner,
                facing: Direction::East,
            }),
            13739u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GreenGlazedTerracotta,
                facing: Direction::North,
            }),
            13740u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GreenGlazedTerracotta,
                facing: Direction::South,
            }),
            13741u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GreenGlazedTerracotta,
                facing: Direction::West,
            }),
            13742u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GreenGlazedTerracotta,
                facing: Direction::East,
            }),
            13669u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GreenShulkerBox,
                facing: Direction::North,
            }),
            13670u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GreenShulkerBox,
                facing: Direction::East,
            }),
            13671u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GreenShulkerBox,
                facing: Direction::South,
            }),
            13672u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GreenShulkerBox,
                facing: Direction::West,
            }),
            13673u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GreenShulkerBox,
                facing: Direction::Up,
            }),
            13674u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GreenShulkerBox,
                facing: Direction::Down,
            }),
            11956u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GreenWallBanner,
                facing: Direction::North,
            }),
            11957u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GreenWallBanner,
                facing: Direction::South,
            }),
            11958u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GreenWallBanner,
                facing: Direction::West,
            }),
            11959u32 => Ok(FacingBlock {
                block_type: FacingBlockType::GreenWallBanner,
                facing: Direction::East,
            }),
            6049u32 => Ok(FacingBlock {
                block_type: FacingBlockType::JackOLantern,
                facing: Direction::North,
            }),
            6050u32 => Ok(FacingBlock {
                block_type: FacingBlockType::JackOLantern,
                facing: Direction::South,
            }),
            6051u32 => Ok(FacingBlock {
                block_type: FacingBlockType::JackOLantern,
                facing: Direction::West,
            }),
            6052u32 => Ok(FacingBlock {
                block_type: FacingBlockType::JackOLantern,
                facing: Direction::East,
            }),
            13699u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightBlueGlazedTerracotta,
                facing: Direction::North,
            }),
            13700u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightBlueGlazedTerracotta,
                facing: Direction::South,
            }),
            13701u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightBlueGlazedTerracotta,
                facing: Direction::West,
            }),
            13702u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightBlueGlazedTerracotta,
                facing: Direction::East,
            }),
            13609u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightBlueShulkerBox,
                facing: Direction::North,
            }),
            13610u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightBlueShulkerBox,
                facing: Direction::East,
            }),
            13611u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightBlueShulkerBox,
                facing: Direction::South,
            }),
            13612u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightBlueShulkerBox,
                facing: Direction::West,
            }),
            13613u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightBlueShulkerBox,
                facing: Direction::Up,
            }),
            13614u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightBlueShulkerBox,
                facing: Direction::Down,
            }),
            11916u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightBlueWallBanner,
                facing: Direction::North,
            }),
            11917u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightBlueWallBanner,
                facing: Direction::South,
            }),
            11918u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightBlueWallBanner,
                facing: Direction::West,
            }),
            11919u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightBlueWallBanner,
                facing: Direction::East,
            }),
            13719u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightGrayGlazedTerracotta,
                facing: Direction::North,
            }),
            13720u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightGrayGlazedTerracotta,
                facing: Direction::South,
            }),
            13721u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightGrayGlazedTerracotta,
                facing: Direction::West,
            }),
            13722u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightGrayGlazedTerracotta,
                facing: Direction::East,
            }),
            13639u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightGrayShulkerBox,
                facing: Direction::North,
            }),
            13640u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightGrayShulkerBox,
                facing: Direction::East,
            }),
            13641u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightGrayShulkerBox,
                facing: Direction::South,
            }),
            13642u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightGrayShulkerBox,
                facing: Direction::West,
            }),
            13643u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightGrayShulkerBox,
                facing: Direction::Up,
            }),
            13644u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightGrayShulkerBox,
                facing: Direction::Down,
            }),
            11936u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightGrayWallBanner,
                facing: Direction::North,
            }),
            11937u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightGrayWallBanner,
                facing: Direction::South,
            }),
            11938u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightGrayWallBanner,
                facing: Direction::West,
            }),
            11939u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LightGrayWallBanner,
                facing: Direction::East,
            }),
            13707u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LimeGlazedTerracotta,
                facing: Direction::North,
            }),
            13708u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LimeGlazedTerracotta,
                facing: Direction::South,
            }),
            13709u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LimeGlazedTerracotta,
                facing: Direction::West,
            }),
            13710u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LimeGlazedTerracotta,
                facing: Direction::East,
            }),
            13621u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LimeShulkerBox,
                facing: Direction::North,
            }),
            13622u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LimeShulkerBox,
                facing: Direction::East,
            }),
            13623u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LimeShulkerBox,
                facing: Direction::South,
            }),
            13624u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LimeShulkerBox,
                facing: Direction::West,
            }),
            13625u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LimeShulkerBox,
                facing: Direction::Up,
            }),
            13626u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LimeShulkerBox,
                facing: Direction::Down,
            }),
            11924u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LimeWallBanner,
                facing: Direction::North,
            }),
            11925u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LimeWallBanner,
                facing: Direction::South,
            }),
            11926u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LimeWallBanner,
                facing: Direction::West,
            }),
            11927u32 => Ok(FacingBlock {
                block_type: FacingBlockType::LimeWallBanner,
                facing: Direction::East,
            }),
            19427u32 => Ok(FacingBlock {
                block_type: FacingBlockType::Loom,
                facing: Direction::North,
            }),
            19428u32 => Ok(FacingBlock {
                block_type: FacingBlockType::Loom,
                facing: Direction::South,
            }),
            19429u32 => Ok(FacingBlock {
                block_type: FacingBlockType::Loom,
                facing: Direction::West,
            }),
            19430u32 => Ok(FacingBlock {
                block_type: FacingBlockType::Loom,
                facing: Direction::East,
            }),
            13695u32 => Ok(FacingBlock {
                block_type: FacingBlockType::MagentaGlazedTerracotta,
                facing: Direction::North,
            }),
            13696u32 => Ok(FacingBlock {
                block_type: FacingBlockType::MagentaGlazedTerracotta,
                facing: Direction::South,
            }),
            13697u32 => Ok(FacingBlock {
                block_type: FacingBlockType::MagentaGlazedTerracotta,
                facing: Direction::West,
            }),
            13698u32 => Ok(FacingBlock {
                block_type: FacingBlockType::MagentaGlazedTerracotta,
                facing: Direction::East,
            }),
            13603u32 => Ok(FacingBlock {
                block_type: FacingBlockType::MagentaShulkerBox,
                facing: Direction::North,
            }),
            13604u32 => Ok(FacingBlock {
                block_type: FacingBlockType::MagentaShulkerBox,
                facing: Direction::East,
            }),
            13605u32 => Ok(FacingBlock {
                block_type: FacingBlockType::MagentaShulkerBox,
                facing: Direction::South,
            }),
            13606u32 => Ok(FacingBlock {
                block_type: FacingBlockType::MagentaShulkerBox,
                facing: Direction::West,
            }),
            13607u32 => Ok(FacingBlock {
                block_type: FacingBlockType::MagentaShulkerBox,
                facing: Direction::Up,
            }),
            13608u32 => Ok(FacingBlock {
                block_type: FacingBlockType::MagentaShulkerBox,
                facing: Direction::Down,
            }),
            11912u32 => Ok(FacingBlock {
                block_type: FacingBlockType::MagentaWallBanner,
                facing: Direction::North,
            }),
            11913u32 => Ok(FacingBlock {
                block_type: FacingBlockType::MagentaWallBanner,
                facing: Direction::South,
            }),
            11914u32 => Ok(FacingBlock {
                block_type: FacingBlockType::MagentaWallBanner,
                facing: Direction::West,
            }),
            11915u32 => Ok(FacingBlock {
                block_type: FacingBlockType::MagentaWallBanner,
                facing: Direction::East,
            }),
            13691u32 => Ok(FacingBlock {
                block_type: FacingBlockType::OrangeGlazedTerracotta,
                facing: Direction::North,
            }),
            13692u32 => Ok(FacingBlock {
                block_type: FacingBlockType::OrangeGlazedTerracotta,
                facing: Direction::South,
            }),
            13693u32 => Ok(FacingBlock {
                block_type: FacingBlockType::OrangeGlazedTerracotta,
                facing: Direction::West,
            }),
            13694u32 => Ok(FacingBlock {
                block_type: FacingBlockType::OrangeGlazedTerracotta,
                facing: Direction::East,
            }),
            13597u32 => Ok(FacingBlock {
                block_type: FacingBlockType::OrangeShulkerBox,
                facing: Direction::North,
            }),
            13598u32 => Ok(FacingBlock {
                block_type: FacingBlockType::OrangeShulkerBox,
                facing: Direction::East,
            }),
            13599u32 => Ok(FacingBlock {
                block_type: FacingBlockType::OrangeShulkerBox,
                facing: Direction::South,
            }),
            13600u32 => Ok(FacingBlock {
                block_type: FacingBlockType::OrangeShulkerBox,
                facing: Direction::West,
            }),
            13601u32 => Ok(FacingBlock {
                block_type: FacingBlockType::OrangeShulkerBox,
                facing: Direction::Up,
            }),
            13602u32 => Ok(FacingBlock {
                block_type: FacingBlockType::OrangeShulkerBox,
                facing: Direction::Down,
            }),
            11908u32 => Ok(FacingBlock {
                block_type: FacingBlockType::OrangeWallBanner,
                facing: Direction::North,
            }),
            11909u32 => Ok(FacingBlock {
                block_type: FacingBlockType::OrangeWallBanner,
                facing: Direction::South,
            }),
            11910u32 => Ok(FacingBlock {
                block_type: FacingBlockType::OrangeWallBanner,
                facing: Direction::West,
            }),
            11911u32 => Ok(FacingBlock {
                block_type: FacingBlockType::OrangeWallBanner,
                facing: Direction::East,
            }),
            13711u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PinkGlazedTerracotta,
                facing: Direction::North,
            }),
            13712u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PinkGlazedTerracotta,
                facing: Direction::South,
            }),
            13713u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PinkGlazedTerracotta,
                facing: Direction::West,
            }),
            13714u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PinkGlazedTerracotta,
                facing: Direction::East,
            }),
            13627u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PinkShulkerBox,
                facing: Direction::North,
            }),
            13628u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PinkShulkerBox,
                facing: Direction::East,
            }),
            13629u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PinkShulkerBox,
                facing: Direction::South,
            }),
            13630u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PinkShulkerBox,
                facing: Direction::West,
            }),
            13631u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PinkShulkerBox,
                facing: Direction::Up,
            }),
            13632u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PinkShulkerBox,
                facing: Direction::Down,
            }),
            11928u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PinkWallBanner,
                facing: Direction::North,
            }),
            11929u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PinkWallBanner,
                facing: Direction::South,
            }),
            11930u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PinkWallBanner,
                facing: Direction::West,
            }),
            11931u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PinkWallBanner,
                facing: Direction::East,
            }),
            13727u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PurpleGlazedTerracotta,
                facing: Direction::North,
            }),
            13728u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PurpleGlazedTerracotta,
                facing: Direction::South,
            }),
            13729u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PurpleGlazedTerracotta,
                facing: Direction::West,
            }),
            13730u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PurpleGlazedTerracotta,
                facing: Direction::East,
            }),
            13651u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PurpleShulkerBox,
                facing: Direction::North,
            }),
            13652u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PurpleShulkerBox,
                facing: Direction::East,
            }),
            13653u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PurpleShulkerBox,
                facing: Direction::South,
            }),
            13654u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PurpleShulkerBox,
                facing: Direction::West,
            }),
            13655u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PurpleShulkerBox,
                facing: Direction::Up,
            }),
            13656u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PurpleShulkerBox,
                facing: Direction::Down,
            }),
            11944u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PurpleWallBanner,
                facing: Direction::North,
            }),
            11945u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PurpleWallBanner,
                facing: Direction::South,
            }),
            11946u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PurpleWallBanner,
                facing: Direction::West,
            }),
            11947u32 => Ok(FacingBlock {
                block_type: FacingBlockType::PurpleWallBanner,
                facing: Direction::East,
            }),
            13743u32 => Ok(FacingBlock {
                block_type: FacingBlockType::RedGlazedTerracotta,
                facing: Direction::North,
            }),
            13744u32 => Ok(FacingBlock {
                block_type: FacingBlockType::RedGlazedTerracotta,
                facing: Direction::South,
            }),
            13745u32 => Ok(FacingBlock {
                block_type: FacingBlockType::RedGlazedTerracotta,
                facing: Direction::West,
            }),
            13746u32 => Ok(FacingBlock {
                block_type: FacingBlockType::RedGlazedTerracotta,
                facing: Direction::East,
            }),
            13675u32 => Ok(FacingBlock {
                block_type: FacingBlockType::RedShulkerBox,
                facing: Direction::North,
            }),
            13676u32 => Ok(FacingBlock {
                block_type: FacingBlockType::RedShulkerBox,
                facing: Direction::East,
            }),
            13677u32 => Ok(FacingBlock {
                block_type: FacingBlockType::RedShulkerBox,
                facing: Direction::South,
            }),
            13678u32 => Ok(FacingBlock {
                block_type: FacingBlockType::RedShulkerBox,
                facing: Direction::West,
            }),
            13679u32 => Ok(FacingBlock {
                block_type: FacingBlockType::RedShulkerBox,
                facing: Direction::Up,
            }),
            13680u32 => Ok(FacingBlock {
                block_type: FacingBlockType::RedShulkerBox,
                facing: Direction::Down,
            }),
            11960u32 => Ok(FacingBlock {
                block_type: FacingBlockType::RedWallBanner,
                facing: Direction::North,
            }),
            11961u32 => Ok(FacingBlock {
                block_type: FacingBlockType::RedWallBanner,
                facing: Direction::South,
            }),
            11962u32 => Ok(FacingBlock {
                block_type: FacingBlockType::RedWallBanner,
                facing: Direction::West,
            }),
            11963u32 => Ok(FacingBlock {
                block_type: FacingBlockType::RedWallBanner,
                facing: Direction::East,
            }),
            13585u32 => Ok(FacingBlock {
                block_type: FacingBlockType::ShulkerBox,
                facing: Direction::North,
            }),
            13586u32 => Ok(FacingBlock {
                block_type: FacingBlockType::ShulkerBox,
                facing: Direction::East,
            }),
            13587u32 => Ok(FacingBlock {
                block_type: FacingBlockType::ShulkerBox,
                facing: Direction::South,
            }),
            13588u32 => Ok(FacingBlock {
                block_type: FacingBlockType::ShulkerBox,
                facing: Direction::West,
            }),
            13589u32 => Ok(FacingBlock {
                block_type: FacingBlockType::ShulkerBox,
                facing: Direction::Up,
            }),
            13590u32 => Ok(FacingBlock {
                block_type: FacingBlockType::ShulkerBox,
                facing: Direction::Down,
            }),
            6038u32 => Ok(FacingBlock {
                block_type: FacingBlockType::SoulWallTorch,
                facing: Direction::North,
            }),
            6039u32 => Ok(FacingBlock {
                block_type: FacingBlockType::SoulWallTorch,
                facing: Direction::South,
            }),
            6040u32 => Ok(FacingBlock {
                block_type: FacingBlockType::SoulWallTorch,
                facing: Direction::West,
            }),
            6041u32 => Ok(FacingBlock {
                block_type: FacingBlockType::SoulWallTorch,
                facing: Direction::East,
            }),
            19490u32 => Ok(FacingBlock {
                block_type: FacingBlockType::Stonecutter,
                facing: Direction::North,
            }),
            19491u32 => Ok(FacingBlock {
                block_type: FacingBlockType::Stonecutter,
                facing: Direction::South,
            }),
            19492u32 => Ok(FacingBlock {
                block_type: FacingBlockType::Stonecutter,
                facing: Direction::West,
            }),
            19493u32 => Ok(FacingBlock {
                block_type: FacingBlockType::Stonecutter,
                facing: Direction::East,
            }),
            2402u32 => Ok(FacingBlock {
                block_type: FacingBlockType::WallTorch,
                facing: Direction::North,
            }),
            2403u32 => Ok(FacingBlock {
                block_type: FacingBlockType::WallTorch,
                facing: Direction::South,
            }),
            2404u32 => Ok(FacingBlock {
                block_type: FacingBlockType::WallTorch,
                facing: Direction::West,
            }),
            2405u32 => Ok(FacingBlock {
                block_type: FacingBlockType::WallTorch,
                facing: Direction::East,
            }),
            13687u32 => Ok(FacingBlock {
                block_type: FacingBlockType::WhiteGlazedTerracotta,
                facing: Direction::North,
            }),
            13688u32 => Ok(FacingBlock {
                block_type: FacingBlockType::WhiteGlazedTerracotta,
                facing: Direction::South,
            }),
            13689u32 => Ok(FacingBlock {
                block_type: FacingBlockType::WhiteGlazedTerracotta,
                facing: Direction::West,
            }),
            13690u32 => Ok(FacingBlock {
                block_type: FacingBlockType::WhiteGlazedTerracotta,
                facing: Direction::East,
            }),
            13591u32 => Ok(FacingBlock {
                block_type: FacingBlockType::WhiteShulkerBox,
                facing: Direction::North,
            }),
            13592u32 => Ok(FacingBlock {
                block_type: FacingBlockType::WhiteShulkerBox,
                facing: Direction::East,
            }),
            13593u32 => Ok(FacingBlock {
                block_type: FacingBlockType::WhiteShulkerBox,
                facing: Direction::South,
            }),
            13594u32 => Ok(FacingBlock {
                block_type: FacingBlockType::WhiteShulkerBox,
                facing: Direction::West,
            }),
            13595u32 => Ok(FacingBlock {
                block_type: FacingBlockType::WhiteShulkerBox,
                facing: Direction::Up,
            }),
            13596u32 => Ok(FacingBlock {
                block_type: FacingBlockType::WhiteShulkerBox,
                facing: Direction::Down,
            }),
            11904u32 => Ok(FacingBlock {
                block_type: FacingBlockType::WhiteWallBanner,
                facing: Direction::North,
            }),
            11905u32 => Ok(FacingBlock {
                block_type: FacingBlockType::WhiteWallBanner,
                facing: Direction::South,
            }),
            11906u32 => Ok(FacingBlock {
                block_type: FacingBlockType::WhiteWallBanner,
                facing: Direction::West,
            }),
            11907u32 => Ok(FacingBlock {
                block_type: FacingBlockType::WhiteWallBanner,
                facing: Direction::East,
            }),
            13703u32 => Ok(FacingBlock {
                block_type: FacingBlockType::YellowGlazedTerracotta,
                facing: Direction::North,
            }),
            13704u32 => Ok(FacingBlock {
                block_type: FacingBlockType::YellowGlazedTerracotta,
                facing: Direction::South,
            }),
            13705u32 => Ok(FacingBlock {
                block_type: FacingBlockType::YellowGlazedTerracotta,
                facing: Direction::West,
            }),
            13706u32 => Ok(FacingBlock {
                block_type: FacingBlockType::YellowGlazedTerracotta,
                facing: Direction::East,
            }),
            13615u32 => Ok(FacingBlock {
                block_type: FacingBlockType::YellowShulkerBox,
                facing: Direction::North,
            }),
            13616u32 => Ok(FacingBlock {
                block_type: FacingBlockType::YellowShulkerBox,
                facing: Direction::East,
            }),
            13617u32 => Ok(FacingBlock {
                block_type: FacingBlockType::YellowShulkerBox,
                facing: Direction::South,
            }),
            13618u32 => Ok(FacingBlock {
                block_type: FacingBlockType::YellowShulkerBox,
                facing: Direction::West,
            }),
            13619u32 => Ok(FacingBlock {
                block_type: FacingBlockType::YellowShulkerBox,
                facing: Direction::Up,
            }),
            13620u32 => Ok(FacingBlock {
                block_type: FacingBlockType::YellowShulkerBox,
                facing: Direction::Down,
            }),
            11920u32 => Ok(FacingBlock {
                block_type: FacingBlockType::YellowWallBanner,
                facing: Direction::North,
            }),
            11921u32 => Ok(FacingBlock {
                block_type: FacingBlockType::YellowWallBanner,
                facing: Direction::South,
            }),
            11922u32 => Ok(FacingBlock {
                block_type: FacingBlockType::YellowWallBanner,
                facing: Direction::West,
            }),
            11923u32 => Ok(FacingBlock {
                block_type: FacingBlockType::YellowWallBanner,
                facing: Direction::East,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for FacingBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            FacingBlock {
                block_type: FacingBlockType::Anvil,
                facing: Direction::North,
            } => Ok(9916u32),
            FacingBlock {
                block_type: FacingBlockType::Anvil,
                facing: Direction::South,
            } => Ok(9917u32),
            FacingBlock {
                block_type: FacingBlockType::Anvil,
                facing: Direction::West,
            } => Ok(9918u32),
            FacingBlock {
                block_type: FacingBlockType::Anvil,
                facing: Direction::East,
            } => Ok(9919u32),
            FacingBlock {
                block_type: FacingBlockType::AttachedMelonStem,
                facing: Direction::North,
            } => Ok(7060u32),
            FacingBlock {
                block_type: FacingBlockType::AttachedMelonStem,
                facing: Direction::South,
            } => Ok(7061u32),
            FacingBlock {
                block_type: FacingBlockType::AttachedMelonStem,
                facing: Direction::West,
            } => Ok(7062u32),
            FacingBlock {
                block_type: FacingBlockType::AttachedMelonStem,
                facing: Direction::East,
            } => Ok(7063u32),
            FacingBlock {
                block_type: FacingBlockType::AttachedPumpkinStem,
                facing: Direction::North,
            } => Ok(7056u32),
            FacingBlock {
                block_type: FacingBlockType::AttachedPumpkinStem,
                facing: Direction::South,
            } => Ok(7057u32),
            FacingBlock {
                block_type: FacingBlockType::AttachedPumpkinStem,
                facing: Direction::West,
            } => Ok(7058u32),
            FacingBlock {
                block_type: FacingBlockType::AttachedPumpkinStem,
                facing: Direction::East,
            } => Ok(7059u32),
            FacingBlock {
                block_type: FacingBlockType::BlackGlazedTerracotta,
                facing: Direction::North,
            } => Ok(13747u32),
            FacingBlock {
                block_type: FacingBlockType::BlackGlazedTerracotta,
                facing: Direction::South,
            } => Ok(13748u32),
            FacingBlock {
                block_type: FacingBlockType::BlackGlazedTerracotta,
                facing: Direction::West,
            } => Ok(13749u32),
            FacingBlock {
                block_type: FacingBlockType::BlackGlazedTerracotta,
                facing: Direction::East,
            } => Ok(13750u32),
            FacingBlock {
                block_type: FacingBlockType::BlackShulkerBox,
                facing: Direction::North,
            } => Ok(13681u32),
            FacingBlock {
                block_type: FacingBlockType::BlackShulkerBox,
                facing: Direction::East,
            } => Ok(13682u32),
            FacingBlock {
                block_type: FacingBlockType::BlackShulkerBox,
                facing: Direction::South,
            } => Ok(13683u32),
            FacingBlock {
                block_type: FacingBlockType::BlackShulkerBox,
                facing: Direction::West,
            } => Ok(13684u32),
            FacingBlock {
                block_type: FacingBlockType::BlackShulkerBox,
                facing: Direction::Up,
            } => Ok(13685u32),
            FacingBlock {
                block_type: FacingBlockType::BlackShulkerBox,
                facing: Direction::Down,
            } => Ok(13686u32),
            FacingBlock {
                block_type: FacingBlockType::BlackWallBanner,
                facing: Direction::North,
            } => Ok(11964u32),
            FacingBlock {
                block_type: FacingBlockType::BlackWallBanner,
                facing: Direction::South,
            } => Ok(11965u32),
            FacingBlock {
                block_type: FacingBlockType::BlackWallBanner,
                facing: Direction::West,
            } => Ok(11966u32),
            FacingBlock {
                block_type: FacingBlockType::BlackWallBanner,
                facing: Direction::East,
            } => Ok(11967u32),
            FacingBlock {
                block_type: FacingBlockType::BlueGlazedTerracotta,
                facing: Direction::North,
            } => Ok(13731u32),
            FacingBlock {
                block_type: FacingBlockType::BlueGlazedTerracotta,
                facing: Direction::South,
            } => Ok(13732u32),
            FacingBlock {
                block_type: FacingBlockType::BlueGlazedTerracotta,
                facing: Direction::West,
            } => Ok(13733u32),
            FacingBlock {
                block_type: FacingBlockType::BlueGlazedTerracotta,
                facing: Direction::East,
            } => Ok(13734u32),
            FacingBlock {
                block_type: FacingBlockType::BlueShulkerBox,
                facing: Direction::North,
            } => Ok(13657u32),
            FacingBlock {
                block_type: FacingBlockType::BlueShulkerBox,
                facing: Direction::East,
            } => Ok(13658u32),
            FacingBlock {
                block_type: FacingBlockType::BlueShulkerBox,
                facing: Direction::South,
            } => Ok(13659u32),
            FacingBlock {
                block_type: FacingBlockType::BlueShulkerBox,
                facing: Direction::West,
            } => Ok(13660u32),
            FacingBlock {
                block_type: FacingBlockType::BlueShulkerBox,
                facing: Direction::Up,
            } => Ok(13661u32),
            FacingBlock {
                block_type: FacingBlockType::BlueShulkerBox,
                facing: Direction::Down,
            } => Ok(13662u32),
            FacingBlock {
                block_type: FacingBlockType::BlueWallBanner,
                facing: Direction::North,
            } => Ok(11948u32),
            FacingBlock {
                block_type: FacingBlockType::BlueWallBanner,
                facing: Direction::South,
            } => Ok(11949u32),
            FacingBlock {
                block_type: FacingBlockType::BlueWallBanner,
                facing: Direction::West,
            } => Ok(11950u32),
            FacingBlock {
                block_type: FacingBlockType::BlueWallBanner,
                facing: Direction::East,
            } => Ok(11951u32),
            FacingBlock {
                block_type: FacingBlockType::BrownGlazedTerracotta,
                facing: Direction::North,
            } => Ok(13735u32),
            FacingBlock {
                block_type: FacingBlockType::BrownGlazedTerracotta,
                facing: Direction::South,
            } => Ok(13736u32),
            FacingBlock {
                block_type: FacingBlockType::BrownGlazedTerracotta,
                facing: Direction::West,
            } => Ok(13737u32),
            FacingBlock {
                block_type: FacingBlockType::BrownGlazedTerracotta,
                facing: Direction::East,
            } => Ok(13738u32),
            FacingBlock {
                block_type: FacingBlockType::BrownShulkerBox,
                facing: Direction::North,
            } => Ok(13663u32),
            FacingBlock {
                block_type: FacingBlockType::BrownShulkerBox,
                facing: Direction::East,
            } => Ok(13664u32),
            FacingBlock {
                block_type: FacingBlockType::BrownShulkerBox,
                facing: Direction::South,
            } => Ok(13665u32),
            FacingBlock {
                block_type: FacingBlockType::BrownShulkerBox,
                facing: Direction::West,
            } => Ok(13666u32),
            FacingBlock {
                block_type: FacingBlockType::BrownShulkerBox,
                facing: Direction::Up,
            } => Ok(13667u32),
            FacingBlock {
                block_type: FacingBlockType::BrownShulkerBox,
                facing: Direction::Down,
            } => Ok(13668u32),
            FacingBlock {
                block_type: FacingBlockType::BrownWallBanner,
                facing: Direction::North,
            } => Ok(11952u32),
            FacingBlock {
                block_type: FacingBlockType::BrownWallBanner,
                facing: Direction::South,
            } => Ok(11953u32),
            FacingBlock {
                block_type: FacingBlockType::BrownWallBanner,
                facing: Direction::West,
            } => Ok(11954u32),
            FacingBlock {
                block_type: FacingBlockType::BrownWallBanner,
                facing: Direction::East,
            } => Ok(11955u32),
            FacingBlock {
                block_type: FacingBlockType::CarvedPumpkin,
                facing: Direction::North,
            } => Ok(6045u32),
            FacingBlock {
                block_type: FacingBlockType::CarvedPumpkin,
                facing: Direction::South,
            } => Ok(6046u32),
            FacingBlock {
                block_type: FacingBlockType::CarvedPumpkin,
                facing: Direction::West,
            } => Ok(6047u32),
            FacingBlock {
                block_type: FacingBlockType::CarvedPumpkin,
                facing: Direction::East,
            } => Ok(6048u32),
            FacingBlock {
                block_type: FacingBlockType::ChippedAnvil,
                facing: Direction::North,
            } => Ok(9920u32),
            FacingBlock {
                block_type: FacingBlockType::ChippedAnvil,
                facing: Direction::South,
            } => Ok(9921u32),
            FacingBlock {
                block_type: FacingBlockType::ChippedAnvil,
                facing: Direction::West,
            } => Ok(9922u32),
            FacingBlock {
                block_type: FacingBlockType::ChippedAnvil,
                facing: Direction::East,
            } => Ok(9923u32),
            FacingBlock {
                block_type: FacingBlockType::CyanGlazedTerracotta,
                facing: Direction::North,
            } => Ok(13723u32),
            FacingBlock {
                block_type: FacingBlockType::CyanGlazedTerracotta,
                facing: Direction::South,
            } => Ok(13724u32),
            FacingBlock {
                block_type: FacingBlockType::CyanGlazedTerracotta,
                facing: Direction::West,
            } => Ok(13725u32),
            FacingBlock {
                block_type: FacingBlockType::CyanGlazedTerracotta,
                facing: Direction::East,
            } => Ok(13726u32),
            FacingBlock {
                block_type: FacingBlockType::CyanShulkerBox,
                facing: Direction::North,
            } => Ok(13645u32),
            FacingBlock {
                block_type: FacingBlockType::CyanShulkerBox,
                facing: Direction::East,
            } => Ok(13646u32),
            FacingBlock {
                block_type: FacingBlockType::CyanShulkerBox,
                facing: Direction::South,
            } => Ok(13647u32),
            FacingBlock {
                block_type: FacingBlockType::CyanShulkerBox,
                facing: Direction::West,
            } => Ok(13648u32),
            FacingBlock {
                block_type: FacingBlockType::CyanShulkerBox,
                facing: Direction::Up,
            } => Ok(13649u32),
            FacingBlock {
                block_type: FacingBlockType::CyanShulkerBox,
                facing: Direction::Down,
            } => Ok(13650u32),
            FacingBlock {
                block_type: FacingBlockType::CyanWallBanner,
                facing: Direction::North,
            } => Ok(11940u32),
            FacingBlock {
                block_type: FacingBlockType::CyanWallBanner,
                facing: Direction::South,
            } => Ok(11941u32),
            FacingBlock {
                block_type: FacingBlockType::CyanWallBanner,
                facing: Direction::West,
            } => Ok(11942u32),
            FacingBlock {
                block_type: FacingBlockType::CyanWallBanner,
                facing: Direction::East,
            } => Ok(11943u32),
            FacingBlock {
                block_type: FacingBlockType::DamagedAnvil,
                facing: Direction::North,
            } => Ok(9924u32),
            FacingBlock {
                block_type: FacingBlockType::DamagedAnvil,
                facing: Direction::South,
            } => Ok(9925u32),
            FacingBlock {
                block_type: FacingBlockType::DamagedAnvil,
                facing: Direction::West,
            } => Ok(9926u32),
            FacingBlock {
                block_type: FacingBlockType::DamagedAnvil,
                facing: Direction::East,
            } => Ok(9927u32),
            FacingBlock {
                block_type: FacingBlockType::EndRod,
                facing: Direction::North,
            } => Ok(13357u32),
            FacingBlock {
                block_type: FacingBlockType::EndRod,
                facing: Direction::East,
            } => Ok(13358u32),
            FacingBlock {
                block_type: FacingBlockType::EndRod,
                facing: Direction::South,
            } => Ok(13359u32),
            FacingBlock {
                block_type: FacingBlockType::EndRod,
                facing: Direction::West,
            } => Ok(13360u32),
            FacingBlock {
                block_type: FacingBlockType::EndRod,
                facing: Direction::Up,
            } => Ok(13361u32),
            FacingBlock {
                block_type: FacingBlockType::EndRod,
                facing: Direction::Down,
            } => Ok(13362u32),
            FacingBlock {
                block_type: FacingBlockType::GrayGlazedTerracotta,
                facing: Direction::North,
            } => Ok(13715u32),
            FacingBlock {
                block_type: FacingBlockType::GrayGlazedTerracotta,
                facing: Direction::South,
            } => Ok(13716u32),
            FacingBlock {
                block_type: FacingBlockType::GrayGlazedTerracotta,
                facing: Direction::West,
            } => Ok(13717u32),
            FacingBlock {
                block_type: FacingBlockType::GrayGlazedTerracotta,
                facing: Direction::East,
            } => Ok(13718u32),
            FacingBlock {
                block_type: FacingBlockType::GrayShulkerBox,
                facing: Direction::North,
            } => Ok(13633u32),
            FacingBlock {
                block_type: FacingBlockType::GrayShulkerBox,
                facing: Direction::East,
            } => Ok(13634u32),
            FacingBlock {
                block_type: FacingBlockType::GrayShulkerBox,
                facing: Direction::South,
            } => Ok(13635u32),
            FacingBlock {
                block_type: FacingBlockType::GrayShulkerBox,
                facing: Direction::West,
            } => Ok(13636u32),
            FacingBlock {
                block_type: FacingBlockType::GrayShulkerBox,
                facing: Direction::Up,
            } => Ok(13637u32),
            FacingBlock {
                block_type: FacingBlockType::GrayShulkerBox,
                facing: Direction::Down,
            } => Ok(13638u32),
            FacingBlock {
                block_type: FacingBlockType::GrayWallBanner,
                facing: Direction::North,
            } => Ok(11932u32),
            FacingBlock {
                block_type: FacingBlockType::GrayWallBanner,
                facing: Direction::South,
            } => Ok(11933u32),
            FacingBlock {
                block_type: FacingBlockType::GrayWallBanner,
                facing: Direction::West,
            } => Ok(11934u32),
            FacingBlock {
                block_type: FacingBlockType::GrayWallBanner,
                facing: Direction::East,
            } => Ok(11935u32),
            FacingBlock {
                block_type: FacingBlockType::GreenGlazedTerracotta,
                facing: Direction::North,
            } => Ok(13739u32),
            FacingBlock {
                block_type: FacingBlockType::GreenGlazedTerracotta,
                facing: Direction::South,
            } => Ok(13740u32),
            FacingBlock {
                block_type: FacingBlockType::GreenGlazedTerracotta,
                facing: Direction::West,
            } => Ok(13741u32),
            FacingBlock {
                block_type: FacingBlockType::GreenGlazedTerracotta,
                facing: Direction::East,
            } => Ok(13742u32),
            FacingBlock {
                block_type: FacingBlockType::GreenShulkerBox,
                facing: Direction::North,
            } => Ok(13669u32),
            FacingBlock {
                block_type: FacingBlockType::GreenShulkerBox,
                facing: Direction::East,
            } => Ok(13670u32),
            FacingBlock {
                block_type: FacingBlockType::GreenShulkerBox,
                facing: Direction::South,
            } => Ok(13671u32),
            FacingBlock {
                block_type: FacingBlockType::GreenShulkerBox,
                facing: Direction::West,
            } => Ok(13672u32),
            FacingBlock {
                block_type: FacingBlockType::GreenShulkerBox,
                facing: Direction::Up,
            } => Ok(13673u32),
            FacingBlock {
                block_type: FacingBlockType::GreenShulkerBox,
                facing: Direction::Down,
            } => Ok(13674u32),
            FacingBlock {
                block_type: FacingBlockType::GreenWallBanner,
                facing: Direction::North,
            } => Ok(11956u32),
            FacingBlock {
                block_type: FacingBlockType::GreenWallBanner,
                facing: Direction::South,
            } => Ok(11957u32),
            FacingBlock {
                block_type: FacingBlockType::GreenWallBanner,
                facing: Direction::West,
            } => Ok(11958u32),
            FacingBlock {
                block_type: FacingBlockType::GreenWallBanner,
                facing: Direction::East,
            } => Ok(11959u32),
            FacingBlock {
                block_type: FacingBlockType::JackOLantern,
                facing: Direction::North,
            } => Ok(6049u32),
            FacingBlock {
                block_type: FacingBlockType::JackOLantern,
                facing: Direction::South,
            } => Ok(6050u32),
            FacingBlock {
                block_type: FacingBlockType::JackOLantern,
                facing: Direction::West,
            } => Ok(6051u32),
            FacingBlock {
                block_type: FacingBlockType::JackOLantern,
                facing: Direction::East,
            } => Ok(6052u32),
            FacingBlock {
                block_type: FacingBlockType::LightBlueGlazedTerracotta,
                facing: Direction::North,
            } => Ok(13699u32),
            FacingBlock {
                block_type: FacingBlockType::LightBlueGlazedTerracotta,
                facing: Direction::South,
            } => Ok(13700u32),
            FacingBlock {
                block_type: FacingBlockType::LightBlueGlazedTerracotta,
                facing: Direction::West,
            } => Ok(13701u32),
            FacingBlock {
                block_type: FacingBlockType::LightBlueGlazedTerracotta,
                facing: Direction::East,
            } => Ok(13702u32),
            FacingBlock {
                block_type: FacingBlockType::LightBlueShulkerBox,
                facing: Direction::North,
            } => Ok(13609u32),
            FacingBlock {
                block_type: FacingBlockType::LightBlueShulkerBox,
                facing: Direction::East,
            } => Ok(13610u32),
            FacingBlock {
                block_type: FacingBlockType::LightBlueShulkerBox,
                facing: Direction::South,
            } => Ok(13611u32),
            FacingBlock {
                block_type: FacingBlockType::LightBlueShulkerBox,
                facing: Direction::West,
            } => Ok(13612u32),
            FacingBlock {
                block_type: FacingBlockType::LightBlueShulkerBox,
                facing: Direction::Up,
            } => Ok(13613u32),
            FacingBlock {
                block_type: FacingBlockType::LightBlueShulkerBox,
                facing: Direction::Down,
            } => Ok(13614u32),
            FacingBlock {
                block_type: FacingBlockType::LightBlueWallBanner,
                facing: Direction::North,
            } => Ok(11916u32),
            FacingBlock {
                block_type: FacingBlockType::LightBlueWallBanner,
                facing: Direction::South,
            } => Ok(11917u32),
            FacingBlock {
                block_type: FacingBlockType::LightBlueWallBanner,
                facing: Direction::West,
            } => Ok(11918u32),
            FacingBlock {
                block_type: FacingBlockType::LightBlueWallBanner,
                facing: Direction::East,
            } => Ok(11919u32),
            FacingBlock {
                block_type: FacingBlockType::LightGrayGlazedTerracotta,
                facing: Direction::North,
            } => Ok(13719u32),
            FacingBlock {
                block_type: FacingBlockType::LightGrayGlazedTerracotta,
                facing: Direction::South,
            } => Ok(13720u32),
            FacingBlock {
                block_type: FacingBlockType::LightGrayGlazedTerracotta,
                facing: Direction::West,
            } => Ok(13721u32),
            FacingBlock {
                block_type: FacingBlockType::LightGrayGlazedTerracotta,
                facing: Direction::East,
            } => Ok(13722u32),
            FacingBlock {
                block_type: FacingBlockType::LightGrayShulkerBox,
                facing: Direction::North,
            } => Ok(13639u32),
            FacingBlock {
                block_type: FacingBlockType::LightGrayShulkerBox,
                facing: Direction::East,
            } => Ok(13640u32),
            FacingBlock {
                block_type: FacingBlockType::LightGrayShulkerBox,
                facing: Direction::South,
            } => Ok(13641u32),
            FacingBlock {
                block_type: FacingBlockType::LightGrayShulkerBox,
                facing: Direction::West,
            } => Ok(13642u32),
            FacingBlock {
                block_type: FacingBlockType::LightGrayShulkerBox,
                facing: Direction::Up,
            } => Ok(13643u32),
            FacingBlock {
                block_type: FacingBlockType::LightGrayShulkerBox,
                facing: Direction::Down,
            } => Ok(13644u32),
            FacingBlock {
                block_type: FacingBlockType::LightGrayWallBanner,
                facing: Direction::North,
            } => Ok(11936u32),
            FacingBlock {
                block_type: FacingBlockType::LightGrayWallBanner,
                facing: Direction::South,
            } => Ok(11937u32),
            FacingBlock {
                block_type: FacingBlockType::LightGrayWallBanner,
                facing: Direction::West,
            } => Ok(11938u32),
            FacingBlock {
                block_type: FacingBlockType::LightGrayWallBanner,
                facing: Direction::East,
            } => Ok(11939u32),
            FacingBlock {
                block_type: FacingBlockType::LimeGlazedTerracotta,
                facing: Direction::North,
            } => Ok(13707u32),
            FacingBlock {
                block_type: FacingBlockType::LimeGlazedTerracotta,
                facing: Direction::South,
            } => Ok(13708u32),
            FacingBlock {
                block_type: FacingBlockType::LimeGlazedTerracotta,
                facing: Direction::West,
            } => Ok(13709u32),
            FacingBlock {
                block_type: FacingBlockType::LimeGlazedTerracotta,
                facing: Direction::East,
            } => Ok(13710u32),
            FacingBlock {
                block_type: FacingBlockType::LimeShulkerBox,
                facing: Direction::North,
            } => Ok(13621u32),
            FacingBlock {
                block_type: FacingBlockType::LimeShulkerBox,
                facing: Direction::East,
            } => Ok(13622u32),
            FacingBlock {
                block_type: FacingBlockType::LimeShulkerBox,
                facing: Direction::South,
            } => Ok(13623u32),
            FacingBlock {
                block_type: FacingBlockType::LimeShulkerBox,
                facing: Direction::West,
            } => Ok(13624u32),
            FacingBlock {
                block_type: FacingBlockType::LimeShulkerBox,
                facing: Direction::Up,
            } => Ok(13625u32),
            FacingBlock {
                block_type: FacingBlockType::LimeShulkerBox,
                facing: Direction::Down,
            } => Ok(13626u32),
            FacingBlock {
                block_type: FacingBlockType::LimeWallBanner,
                facing: Direction::North,
            } => Ok(11924u32),
            FacingBlock {
                block_type: FacingBlockType::LimeWallBanner,
                facing: Direction::South,
            } => Ok(11925u32),
            FacingBlock {
                block_type: FacingBlockType::LimeWallBanner,
                facing: Direction::West,
            } => Ok(11926u32),
            FacingBlock {
                block_type: FacingBlockType::LimeWallBanner,
                facing: Direction::East,
            } => Ok(11927u32),
            FacingBlock {
                block_type: FacingBlockType::Loom,
                facing: Direction::North,
            } => Ok(19427u32),
            FacingBlock {
                block_type: FacingBlockType::Loom,
                facing: Direction::South,
            } => Ok(19428u32),
            FacingBlock {
                block_type: FacingBlockType::Loom,
                facing: Direction::West,
            } => Ok(19429u32),
            FacingBlock {
                block_type: FacingBlockType::Loom,
                facing: Direction::East,
            } => Ok(19430u32),
            FacingBlock {
                block_type: FacingBlockType::MagentaGlazedTerracotta,
                facing: Direction::North,
            } => Ok(13695u32),
            FacingBlock {
                block_type: FacingBlockType::MagentaGlazedTerracotta,
                facing: Direction::South,
            } => Ok(13696u32),
            FacingBlock {
                block_type: FacingBlockType::MagentaGlazedTerracotta,
                facing: Direction::West,
            } => Ok(13697u32),
            FacingBlock {
                block_type: FacingBlockType::MagentaGlazedTerracotta,
                facing: Direction::East,
            } => Ok(13698u32),
            FacingBlock {
                block_type: FacingBlockType::MagentaShulkerBox,
                facing: Direction::North,
            } => Ok(13603u32),
            FacingBlock {
                block_type: FacingBlockType::MagentaShulkerBox,
                facing: Direction::East,
            } => Ok(13604u32),
            FacingBlock {
                block_type: FacingBlockType::MagentaShulkerBox,
                facing: Direction::South,
            } => Ok(13605u32),
            FacingBlock {
                block_type: FacingBlockType::MagentaShulkerBox,
                facing: Direction::West,
            } => Ok(13606u32),
            FacingBlock {
                block_type: FacingBlockType::MagentaShulkerBox,
                facing: Direction::Up,
            } => Ok(13607u32),
            FacingBlock {
                block_type: FacingBlockType::MagentaShulkerBox,
                facing: Direction::Down,
            } => Ok(13608u32),
            FacingBlock {
                block_type: FacingBlockType::MagentaWallBanner,
                facing: Direction::North,
            } => Ok(11912u32),
            FacingBlock {
                block_type: FacingBlockType::MagentaWallBanner,
                facing: Direction::South,
            } => Ok(11913u32),
            FacingBlock {
                block_type: FacingBlockType::MagentaWallBanner,
                facing: Direction::West,
            } => Ok(11914u32),
            FacingBlock {
                block_type: FacingBlockType::MagentaWallBanner,
                facing: Direction::East,
            } => Ok(11915u32),
            FacingBlock {
                block_type: FacingBlockType::OrangeGlazedTerracotta,
                facing: Direction::North,
            } => Ok(13691u32),
            FacingBlock {
                block_type: FacingBlockType::OrangeGlazedTerracotta,
                facing: Direction::South,
            } => Ok(13692u32),
            FacingBlock {
                block_type: FacingBlockType::OrangeGlazedTerracotta,
                facing: Direction::West,
            } => Ok(13693u32),
            FacingBlock {
                block_type: FacingBlockType::OrangeGlazedTerracotta,
                facing: Direction::East,
            } => Ok(13694u32),
            FacingBlock {
                block_type: FacingBlockType::OrangeShulkerBox,
                facing: Direction::North,
            } => Ok(13597u32),
            FacingBlock {
                block_type: FacingBlockType::OrangeShulkerBox,
                facing: Direction::East,
            } => Ok(13598u32),
            FacingBlock {
                block_type: FacingBlockType::OrangeShulkerBox,
                facing: Direction::South,
            } => Ok(13599u32),
            FacingBlock {
                block_type: FacingBlockType::OrangeShulkerBox,
                facing: Direction::West,
            } => Ok(13600u32),
            FacingBlock {
                block_type: FacingBlockType::OrangeShulkerBox,
                facing: Direction::Up,
            } => Ok(13601u32),
            FacingBlock {
                block_type: FacingBlockType::OrangeShulkerBox,
                facing: Direction::Down,
            } => Ok(13602u32),
            FacingBlock {
                block_type: FacingBlockType::OrangeWallBanner,
                facing: Direction::North,
            } => Ok(11908u32),
            FacingBlock {
                block_type: FacingBlockType::OrangeWallBanner,
                facing: Direction::South,
            } => Ok(11909u32),
            FacingBlock {
                block_type: FacingBlockType::OrangeWallBanner,
                facing: Direction::West,
            } => Ok(11910u32),
            FacingBlock {
                block_type: FacingBlockType::OrangeWallBanner,
                facing: Direction::East,
            } => Ok(11911u32),
            FacingBlock {
                block_type: FacingBlockType::PinkGlazedTerracotta,
                facing: Direction::North,
            } => Ok(13711u32),
            FacingBlock {
                block_type: FacingBlockType::PinkGlazedTerracotta,
                facing: Direction::South,
            } => Ok(13712u32),
            FacingBlock {
                block_type: FacingBlockType::PinkGlazedTerracotta,
                facing: Direction::West,
            } => Ok(13713u32),
            FacingBlock {
                block_type: FacingBlockType::PinkGlazedTerracotta,
                facing: Direction::East,
            } => Ok(13714u32),
            FacingBlock {
                block_type: FacingBlockType::PinkShulkerBox,
                facing: Direction::North,
            } => Ok(13627u32),
            FacingBlock {
                block_type: FacingBlockType::PinkShulkerBox,
                facing: Direction::East,
            } => Ok(13628u32),
            FacingBlock {
                block_type: FacingBlockType::PinkShulkerBox,
                facing: Direction::South,
            } => Ok(13629u32),
            FacingBlock {
                block_type: FacingBlockType::PinkShulkerBox,
                facing: Direction::West,
            } => Ok(13630u32),
            FacingBlock {
                block_type: FacingBlockType::PinkShulkerBox,
                facing: Direction::Up,
            } => Ok(13631u32),
            FacingBlock {
                block_type: FacingBlockType::PinkShulkerBox,
                facing: Direction::Down,
            } => Ok(13632u32),
            FacingBlock {
                block_type: FacingBlockType::PinkWallBanner,
                facing: Direction::North,
            } => Ok(11928u32),
            FacingBlock {
                block_type: FacingBlockType::PinkWallBanner,
                facing: Direction::South,
            } => Ok(11929u32),
            FacingBlock {
                block_type: FacingBlockType::PinkWallBanner,
                facing: Direction::West,
            } => Ok(11930u32),
            FacingBlock {
                block_type: FacingBlockType::PinkWallBanner,
                facing: Direction::East,
            } => Ok(11931u32),
            FacingBlock {
                block_type: FacingBlockType::PurpleGlazedTerracotta,
                facing: Direction::North,
            } => Ok(13727u32),
            FacingBlock {
                block_type: FacingBlockType::PurpleGlazedTerracotta,
                facing: Direction::South,
            } => Ok(13728u32),
            FacingBlock {
                block_type: FacingBlockType::PurpleGlazedTerracotta,
                facing: Direction::West,
            } => Ok(13729u32),
            FacingBlock {
                block_type: FacingBlockType::PurpleGlazedTerracotta,
                facing: Direction::East,
            } => Ok(13730u32),
            FacingBlock {
                block_type: FacingBlockType::PurpleShulkerBox,
                facing: Direction::North,
            } => Ok(13651u32),
            FacingBlock {
                block_type: FacingBlockType::PurpleShulkerBox,
                facing: Direction::East,
            } => Ok(13652u32),
            FacingBlock {
                block_type: FacingBlockType::PurpleShulkerBox,
                facing: Direction::South,
            } => Ok(13653u32),
            FacingBlock {
                block_type: FacingBlockType::PurpleShulkerBox,
                facing: Direction::West,
            } => Ok(13654u32),
            FacingBlock {
                block_type: FacingBlockType::PurpleShulkerBox,
                facing: Direction::Up,
            } => Ok(13655u32),
            FacingBlock {
                block_type: FacingBlockType::PurpleShulkerBox,
                facing: Direction::Down,
            } => Ok(13656u32),
            FacingBlock {
                block_type: FacingBlockType::PurpleWallBanner,
                facing: Direction::North,
            } => Ok(11944u32),
            FacingBlock {
                block_type: FacingBlockType::PurpleWallBanner,
                facing: Direction::South,
            } => Ok(11945u32),
            FacingBlock {
                block_type: FacingBlockType::PurpleWallBanner,
                facing: Direction::West,
            } => Ok(11946u32),
            FacingBlock {
                block_type: FacingBlockType::PurpleWallBanner,
                facing: Direction::East,
            } => Ok(11947u32),
            FacingBlock {
                block_type: FacingBlockType::RedGlazedTerracotta,
                facing: Direction::North,
            } => Ok(13743u32),
            FacingBlock {
                block_type: FacingBlockType::RedGlazedTerracotta,
                facing: Direction::South,
            } => Ok(13744u32),
            FacingBlock {
                block_type: FacingBlockType::RedGlazedTerracotta,
                facing: Direction::West,
            } => Ok(13745u32),
            FacingBlock {
                block_type: FacingBlockType::RedGlazedTerracotta,
                facing: Direction::East,
            } => Ok(13746u32),
            FacingBlock {
                block_type: FacingBlockType::RedShulkerBox,
                facing: Direction::North,
            } => Ok(13675u32),
            FacingBlock {
                block_type: FacingBlockType::RedShulkerBox,
                facing: Direction::East,
            } => Ok(13676u32),
            FacingBlock {
                block_type: FacingBlockType::RedShulkerBox,
                facing: Direction::South,
            } => Ok(13677u32),
            FacingBlock {
                block_type: FacingBlockType::RedShulkerBox,
                facing: Direction::West,
            } => Ok(13678u32),
            FacingBlock {
                block_type: FacingBlockType::RedShulkerBox,
                facing: Direction::Up,
            } => Ok(13679u32),
            FacingBlock {
                block_type: FacingBlockType::RedShulkerBox,
                facing: Direction::Down,
            } => Ok(13680u32),
            FacingBlock {
                block_type: FacingBlockType::RedWallBanner,
                facing: Direction::North,
            } => Ok(11960u32),
            FacingBlock {
                block_type: FacingBlockType::RedWallBanner,
                facing: Direction::South,
            } => Ok(11961u32),
            FacingBlock {
                block_type: FacingBlockType::RedWallBanner,
                facing: Direction::West,
            } => Ok(11962u32),
            FacingBlock {
                block_type: FacingBlockType::RedWallBanner,
                facing: Direction::East,
            } => Ok(11963u32),
            FacingBlock {
                block_type: FacingBlockType::ShulkerBox,
                facing: Direction::North,
            } => Ok(13585u32),
            FacingBlock {
                block_type: FacingBlockType::ShulkerBox,
                facing: Direction::East,
            } => Ok(13586u32),
            FacingBlock {
                block_type: FacingBlockType::ShulkerBox,
                facing: Direction::South,
            } => Ok(13587u32),
            FacingBlock {
                block_type: FacingBlockType::ShulkerBox,
                facing: Direction::West,
            } => Ok(13588u32),
            FacingBlock {
                block_type: FacingBlockType::ShulkerBox,
                facing: Direction::Up,
            } => Ok(13589u32),
            FacingBlock {
                block_type: FacingBlockType::ShulkerBox,
                facing: Direction::Down,
            } => Ok(13590u32),
            FacingBlock {
                block_type: FacingBlockType::SoulWallTorch,
                facing: Direction::North,
            } => Ok(6038u32),
            FacingBlock {
                block_type: FacingBlockType::SoulWallTorch,
                facing: Direction::South,
            } => Ok(6039u32),
            FacingBlock {
                block_type: FacingBlockType::SoulWallTorch,
                facing: Direction::West,
            } => Ok(6040u32),
            FacingBlock {
                block_type: FacingBlockType::SoulWallTorch,
                facing: Direction::East,
            } => Ok(6041u32),
            FacingBlock {
                block_type: FacingBlockType::Stonecutter,
                facing: Direction::North,
            } => Ok(19490u32),
            FacingBlock {
                block_type: FacingBlockType::Stonecutter,
                facing: Direction::South,
            } => Ok(19491u32),
            FacingBlock {
                block_type: FacingBlockType::Stonecutter,
                facing: Direction::West,
            } => Ok(19492u32),
            FacingBlock {
                block_type: FacingBlockType::Stonecutter,
                facing: Direction::East,
            } => Ok(19493u32),
            FacingBlock {
                block_type: FacingBlockType::WallTorch,
                facing: Direction::North,
            } => Ok(2402u32),
            FacingBlock {
                block_type: FacingBlockType::WallTorch,
                facing: Direction::South,
            } => Ok(2403u32),
            FacingBlock {
                block_type: FacingBlockType::WallTorch,
                facing: Direction::West,
            } => Ok(2404u32),
            FacingBlock {
                block_type: FacingBlockType::WallTorch,
                facing: Direction::East,
            } => Ok(2405u32),
            FacingBlock {
                block_type: FacingBlockType::WhiteGlazedTerracotta,
                facing: Direction::North,
            } => Ok(13687u32),
            FacingBlock {
                block_type: FacingBlockType::WhiteGlazedTerracotta,
                facing: Direction::South,
            } => Ok(13688u32),
            FacingBlock {
                block_type: FacingBlockType::WhiteGlazedTerracotta,
                facing: Direction::West,
            } => Ok(13689u32),
            FacingBlock {
                block_type: FacingBlockType::WhiteGlazedTerracotta,
                facing: Direction::East,
            } => Ok(13690u32),
            FacingBlock {
                block_type: FacingBlockType::WhiteShulkerBox,
                facing: Direction::North,
            } => Ok(13591u32),
            FacingBlock {
                block_type: FacingBlockType::WhiteShulkerBox,
                facing: Direction::East,
            } => Ok(13592u32),
            FacingBlock {
                block_type: FacingBlockType::WhiteShulkerBox,
                facing: Direction::South,
            } => Ok(13593u32),
            FacingBlock {
                block_type: FacingBlockType::WhiteShulkerBox,
                facing: Direction::West,
            } => Ok(13594u32),
            FacingBlock {
                block_type: FacingBlockType::WhiteShulkerBox,
                facing: Direction::Up,
            } => Ok(13595u32),
            FacingBlock {
                block_type: FacingBlockType::WhiteShulkerBox,
                facing: Direction::Down,
            } => Ok(13596u32),
            FacingBlock {
                block_type: FacingBlockType::WhiteWallBanner,
                facing: Direction::North,
            } => Ok(11904u32),
            FacingBlock {
                block_type: FacingBlockType::WhiteWallBanner,
                facing: Direction::South,
            } => Ok(11905u32),
            FacingBlock {
                block_type: FacingBlockType::WhiteWallBanner,
                facing: Direction::West,
            } => Ok(11906u32),
            FacingBlock {
                block_type: FacingBlockType::WhiteWallBanner,
                facing: Direction::East,
            } => Ok(11907u32),
            FacingBlock {
                block_type: FacingBlockType::YellowGlazedTerracotta,
                facing: Direction::North,
            } => Ok(13703u32),
            FacingBlock {
                block_type: FacingBlockType::YellowGlazedTerracotta,
                facing: Direction::South,
            } => Ok(13704u32),
            FacingBlock {
                block_type: FacingBlockType::YellowGlazedTerracotta,
                facing: Direction::West,
            } => Ok(13705u32),
            FacingBlock {
                block_type: FacingBlockType::YellowGlazedTerracotta,
                facing: Direction::East,
            } => Ok(13706u32),
            FacingBlock {
                block_type: FacingBlockType::YellowShulkerBox,
                facing: Direction::North,
            } => Ok(13615u32),
            FacingBlock {
                block_type: FacingBlockType::YellowShulkerBox,
                facing: Direction::East,
            } => Ok(13616u32),
            FacingBlock {
                block_type: FacingBlockType::YellowShulkerBox,
                facing: Direction::South,
            } => Ok(13617u32),
            FacingBlock {
                block_type: FacingBlockType::YellowShulkerBox,
                facing: Direction::West,
            } => Ok(13618u32),
            FacingBlock {
                block_type: FacingBlockType::YellowShulkerBox,
                facing: Direction::Up,
            } => Ok(13619u32),
            FacingBlock {
                block_type: FacingBlockType::YellowShulkerBox,
                facing: Direction::Down,
            } => Ok(13620u32),
            FacingBlock {
                block_type: FacingBlockType::YellowWallBanner,
                facing: Direction::North,
            } => Ok(11920u32),
            FacingBlock {
                block_type: FacingBlockType::YellowWallBanner,
                facing: Direction::South,
            } => Ok(11921u32),
            FacingBlock {
                block_type: FacingBlockType::YellowWallBanner,
                facing: Direction::West,
            } => Ok(11922u32),
            FacingBlock {
                block_type: FacingBlockType::YellowWallBanner,
                facing: Direction::East,
            } => Ok(11923u32),
            _ => Err(()),
        }
    }
}

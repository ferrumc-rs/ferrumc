#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum PillarBlockType {
    AcaciaLog,
    AcaciaWood,
    BambooBlock,
    Basalt,
    BirchLog,
    BirchWood,
    BoneBlock,
    CherryLog,
    CherryWood,
    CrimsonHyphae,
    CrimsonStem,
    DarkOakLog,
    DarkOakWood,
    Deepslate,
    HayBlock,
    InfestedDeepslate,
    JungleLog,
    JungleWood,
    MangroveLog,
    MangroveWood,
    MuddyMangroveRoots,
    NetherPortal,
    OakLog,
    OakWood,
    OchreFroglight,
    PaleOakLog,
    PaleOakWood,
    PearlescentFroglight,
    PolishedBasalt,
    PurpurPillar,
    QuartzPillar,
    SpruceLog,
    SpruceWood,
    StrippedAcaciaLog,
    StrippedAcaciaWood,
    StrippedBambooBlock,
    StrippedBirchLog,
    StrippedBirchWood,
    StrippedCherryLog,
    StrippedCherryWood,
    StrippedCrimsonHyphae,
    StrippedCrimsonStem,
    StrippedDarkOakLog,
    StrippedDarkOakWood,
    StrippedJungleLog,
    StrippedJungleWood,
    StrippedMangroveLog,
    StrippedMangroveWood,
    StrippedOakLog,
    StrippedOakWood,
    StrippedPaleOakLog,
    StrippedPaleOakWood,
    StrippedSpruceLog,
    StrippedSpruceWood,
    StrippedWarpedHyphae,
    StrippedWarpedStem,
    VerdantFroglight,
    WarpedHyphae,
    WarpedStem,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PillarBlock {
    pub block_type: PillarBlockType,
    pub axis: Axis,
}
impl PillarBlock {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<PillarBlock>();
}
impl TryFrom<u32> for PillarBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            148u32 => Ok(PillarBlock {
                block_type: PillarBlockType::AcaciaLog,
                axis: Axis::X,
            }),
            149u32 => Ok(PillarBlock {
                block_type: PillarBlockType::AcaciaLog,
                axis: Axis::Y,
            }),
            150u32 => Ok(PillarBlock {
                block_type: PillarBlockType::AcaciaLog,
                axis: Axis::Z,
            }),
            213u32 => Ok(PillarBlock {
                block_type: PillarBlockType::AcaciaWood,
                axis: Axis::X,
            }),
            214u32 => Ok(PillarBlock {
                block_type: PillarBlockType::AcaciaWood,
                axis: Axis::Y,
            }),
            215u32 => Ok(PillarBlock {
                block_type: PillarBlockType::AcaciaWood,
                axis: Axis::Z,
            }),
            168u32 => Ok(PillarBlock {
                block_type: PillarBlockType::BambooBlock,
                axis: Axis::X,
            }),
            169u32 => Ok(PillarBlock {
                block_type: PillarBlockType::BambooBlock,
                axis: Axis::Y,
            }),
            170u32 => Ok(PillarBlock {
                block_type: PillarBlockType::BambooBlock,
                axis: Axis::Z,
            }),
            6031u32 => Ok(PillarBlock {
                block_type: PillarBlockType::Basalt,
                axis: Axis::X,
            }),
            6032u32 => Ok(PillarBlock {
                block_type: PillarBlockType::Basalt,
                axis: Axis::Y,
            }),
            6033u32 => Ok(PillarBlock {
                block_type: PillarBlockType::Basalt,
                axis: Axis::Z,
            }),
            142u32 => Ok(PillarBlock {
                block_type: PillarBlockType::BirchLog,
                axis: Axis::X,
            }),
            143u32 => Ok(PillarBlock {
                block_type: PillarBlockType::BirchLog,
                axis: Axis::Y,
            }),
            144u32 => Ok(PillarBlock {
                block_type: PillarBlockType::BirchLog,
                axis: Axis::Z,
            }),
            207u32 => Ok(PillarBlock {
                block_type: PillarBlockType::BirchWood,
                axis: Axis::X,
            }),
            208u32 => Ok(PillarBlock {
                block_type: PillarBlockType::BirchWood,
                axis: Axis::Y,
            }),
            209u32 => Ok(PillarBlock {
                block_type: PillarBlockType::BirchWood,
                axis: Axis::Z,
            }),
            13569u32 => Ok(PillarBlock {
                block_type: PillarBlockType::BoneBlock,
                axis: Axis::X,
            }),
            13570u32 => Ok(PillarBlock {
                block_type: PillarBlockType::BoneBlock,
                axis: Axis::Y,
            }),
            13571u32 => Ok(PillarBlock {
                block_type: PillarBlockType::BoneBlock,
                axis: Axis::Z,
            }),
            151u32 => Ok(PillarBlock {
                block_type: PillarBlockType::CherryLog,
                axis: Axis::X,
            }),
            152u32 => Ok(PillarBlock {
                block_type: PillarBlockType::CherryLog,
                axis: Axis::Y,
            }),
            153u32 => Ok(PillarBlock {
                block_type: PillarBlockType::CherryLog,
                axis: Axis::Z,
            }),
            216u32 => Ok(PillarBlock {
                block_type: PillarBlockType::CherryWood,
                axis: Axis::X,
            }),
            217u32 => Ok(PillarBlock {
                block_type: PillarBlockType::CherryWood,
                axis: Axis::Y,
            }),
            218u32 => Ok(PillarBlock {
                block_type: PillarBlockType::CherryWood,
                axis: Axis::Z,
            }),
            19625u32 => Ok(PillarBlock {
                block_type: PillarBlockType::CrimsonHyphae,
                axis: Axis::X,
            }),
            19626u32 => Ok(PillarBlock {
                block_type: PillarBlockType::CrimsonHyphae,
                axis: Axis::Y,
            }),
            19627u32 => Ok(PillarBlock {
                block_type: PillarBlockType::CrimsonHyphae,
                axis: Axis::Z,
            }),
            19619u32 => Ok(PillarBlock {
                block_type: PillarBlockType::CrimsonStem,
                axis: Axis::X,
            }),
            19620u32 => Ok(PillarBlock {
                block_type: PillarBlockType::CrimsonStem,
                axis: Axis::Y,
            }),
            19621u32 => Ok(PillarBlock {
                block_type: PillarBlockType::CrimsonStem,
                axis: Axis::Z,
            }),
            154u32 => Ok(PillarBlock {
                block_type: PillarBlockType::DarkOakLog,
                axis: Axis::X,
            }),
            155u32 => Ok(PillarBlock {
                block_type: PillarBlockType::DarkOakLog,
                axis: Axis::Y,
            }),
            156u32 => Ok(PillarBlock {
                block_type: PillarBlockType::DarkOakLog,
                axis: Axis::Z,
            }),
            219u32 => Ok(PillarBlock {
                block_type: PillarBlockType::DarkOakWood,
                axis: Axis::X,
            }),
            220u32 => Ok(PillarBlock {
                block_type: PillarBlockType::DarkOakWood,
                axis: Axis::Y,
            }),
            221u32 => Ok(PillarBlock {
                block_type: PillarBlockType::DarkOakWood,
                axis: Axis::Z,
            }),
            25964u32 => Ok(PillarBlock {
                block_type: PillarBlockType::Deepslate,
                axis: Axis::X,
            }),
            25965u32 => Ok(PillarBlock {
                block_type: PillarBlockType::Deepslate,
                axis: Axis::Y,
            }),
            25966u32 => Ok(PillarBlock {
                block_type: PillarBlockType::Deepslate,
                axis: Axis::Z,
            }),
            11614u32 => Ok(PillarBlock {
                block_type: PillarBlockType::HayBlock,
                axis: Axis::X,
            }),
            11615u32 => Ok(PillarBlock {
                block_type: PillarBlockType::HayBlock,
                axis: Axis::Y,
            }),
            11616u32 => Ok(PillarBlock {
                block_type: PillarBlockType::HayBlock,
                axis: Axis::Z,
            }),
            27614u32 => Ok(PillarBlock {
                block_type: PillarBlockType::InfestedDeepslate,
                axis: Axis::X,
            }),
            27615u32 => Ok(PillarBlock {
                block_type: PillarBlockType::InfestedDeepslate,
                axis: Axis::Y,
            }),
            27616u32 => Ok(PillarBlock {
                block_type: PillarBlockType::InfestedDeepslate,
                axis: Axis::Z,
            }),
            145u32 => Ok(PillarBlock {
                block_type: PillarBlockType::JungleLog,
                axis: Axis::X,
            }),
            146u32 => Ok(PillarBlock {
                block_type: PillarBlockType::JungleLog,
                axis: Axis::Y,
            }),
            147u32 => Ok(PillarBlock {
                block_type: PillarBlockType::JungleLog,
                axis: Axis::Z,
            }),
            210u32 => Ok(PillarBlock {
                block_type: PillarBlockType::JungleWood,
                axis: Axis::X,
            }),
            211u32 => Ok(PillarBlock {
                block_type: PillarBlockType::JungleWood,
                axis: Axis::Y,
            }),
            212u32 => Ok(PillarBlock {
                block_type: PillarBlockType::JungleWood,
                axis: Axis::Z,
            }),
            160u32 => Ok(PillarBlock {
                block_type: PillarBlockType::MangroveLog,
                axis: Axis::X,
            }),
            161u32 => Ok(PillarBlock {
                block_type: PillarBlockType::MangroveLog,
                axis: Axis::Y,
            }),
            162u32 => Ok(PillarBlock {
                block_type: PillarBlockType::MangroveLog,
                axis: Axis::Z,
            }),
            222u32 => Ok(PillarBlock {
                block_type: PillarBlockType::MangroveWood,
                axis: Axis::X,
            }),
            223u32 => Ok(PillarBlock {
                block_type: PillarBlockType::MangroveWood,
                axis: Axis::Y,
            }),
            224u32 => Ok(PillarBlock {
                block_type: PillarBlockType::MangroveWood,
                axis: Axis::Z,
            }),
            165u32 => Ok(PillarBlock {
                block_type: PillarBlockType::MuddyMangroveRoots,
                axis: Axis::X,
            }),
            166u32 => Ok(PillarBlock {
                block_type: PillarBlockType::MuddyMangroveRoots,
                axis: Axis::Y,
            }),
            167u32 => Ok(PillarBlock {
                block_type: PillarBlockType::MuddyMangroveRoots,
                axis: Axis::Z,
            }),
            6043u32 => Ok(PillarBlock {
                block_type: PillarBlockType::NetherPortal,
                axis: Axis::X,
            }),
            6044u32 => Ok(PillarBlock {
                block_type: PillarBlockType::NetherPortal,
                axis: Axis::Z,
            }),
            136u32 => Ok(PillarBlock {
                block_type: PillarBlockType::OakLog,
                axis: Axis::X,
            }),
            137u32 => Ok(PillarBlock {
                block_type: PillarBlockType::OakLog,
                axis: Axis::Y,
            }),
            138u32 => Ok(PillarBlock {
                block_type: PillarBlockType::OakLog,
                axis: Axis::Z,
            }),
            201u32 => Ok(PillarBlock {
                block_type: PillarBlockType::OakWood,
                axis: Axis::X,
            }),
            202u32 => Ok(PillarBlock {
                block_type: PillarBlockType::OakWood,
                axis: Axis::Y,
            }),
            203u32 => Ok(PillarBlock {
                block_type: PillarBlockType::OakWood,
                axis: Axis::Z,
            }),
            27623u32 => Ok(PillarBlock {
                block_type: PillarBlockType::OchreFroglight,
                axis: Axis::X,
            }),
            27624u32 => Ok(PillarBlock {
                block_type: PillarBlockType::OchreFroglight,
                axis: Axis::Y,
            }),
            27625u32 => Ok(PillarBlock {
                block_type: PillarBlockType::OchreFroglight,
                axis: Axis::Z,
            }),
            157u32 => Ok(PillarBlock {
                block_type: PillarBlockType::PaleOakLog,
                axis: Axis::X,
            }),
            158u32 => Ok(PillarBlock {
                block_type: PillarBlockType::PaleOakLog,
                axis: Axis::Y,
            }),
            159u32 => Ok(PillarBlock {
                block_type: PillarBlockType::PaleOakLog,
                axis: Axis::Z,
            }),
            22u32 => Ok(PillarBlock {
                block_type: PillarBlockType::PaleOakWood,
                axis: Axis::X,
            }),
            23u32 => Ok(PillarBlock {
                block_type: PillarBlockType::PaleOakWood,
                axis: Axis::Y,
            }),
            24u32 => Ok(PillarBlock {
                block_type: PillarBlockType::PaleOakWood,
                axis: Axis::Z,
            }),
            27629u32 => Ok(PillarBlock {
                block_type: PillarBlockType::PearlescentFroglight,
                axis: Axis::X,
            }),
            27630u32 => Ok(PillarBlock {
                block_type: PillarBlockType::PearlescentFroglight,
                axis: Axis::Y,
            }),
            27631u32 => Ok(PillarBlock {
                block_type: PillarBlockType::PearlescentFroglight,
                axis: Axis::Z,
            }),
            6034u32 => Ok(PillarBlock {
                block_type: PillarBlockType::PolishedBasalt,
                axis: Axis::X,
            }),
            6035u32 => Ok(PillarBlock {
                block_type: PillarBlockType::PolishedBasalt,
                axis: Axis::Y,
            }),
            6036u32 => Ok(PillarBlock {
                block_type: PillarBlockType::PolishedBasalt,
                axis: Axis::Z,
            }),
            13434u32 => Ok(PillarBlock {
                block_type: PillarBlockType::PurpurPillar,
                axis: Axis::X,
            }),
            13435u32 => Ok(PillarBlock {
                block_type: PillarBlockType::PurpurPillar,
                axis: Axis::Y,
            }),
            13436u32 => Ok(PillarBlock {
                block_type: PillarBlockType::PurpurPillar,
                axis: Axis::Z,
            }),
            10046u32 => Ok(PillarBlock {
                block_type: PillarBlockType::QuartzPillar,
                axis: Axis::X,
            }),
            10047u32 => Ok(PillarBlock {
                block_type: PillarBlockType::QuartzPillar,
                axis: Axis::Y,
            }),
            10048u32 => Ok(PillarBlock {
                block_type: PillarBlockType::QuartzPillar,
                axis: Axis::Z,
            }),
            139u32 => Ok(PillarBlock {
                block_type: PillarBlockType::SpruceLog,
                axis: Axis::X,
            }),
            140u32 => Ok(PillarBlock {
                block_type: PillarBlockType::SpruceLog,
                axis: Axis::Y,
            }),
            141u32 => Ok(PillarBlock {
                block_type: PillarBlockType::SpruceLog,
                axis: Axis::Z,
            }),
            204u32 => Ok(PillarBlock {
                block_type: PillarBlockType::SpruceWood,
                axis: Axis::X,
            }),
            205u32 => Ok(PillarBlock {
                block_type: PillarBlockType::SpruceWood,
                axis: Axis::Y,
            }),
            206u32 => Ok(PillarBlock {
                block_type: PillarBlockType::SpruceWood,
                axis: Axis::Z,
            }),
            180u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedAcaciaLog,
                axis: Axis::X,
            }),
            181u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedAcaciaLog,
                axis: Axis::Y,
            }),
            182u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedAcaciaLog,
                axis: Axis::Z,
            }),
            237u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedAcaciaWood,
                axis: Axis::X,
            }),
            238u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedAcaciaWood,
                axis: Axis::Y,
            }),
            239u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedAcaciaWood,
                axis: Axis::Z,
            }),
            198u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedBambooBlock,
                axis: Axis::X,
            }),
            199u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedBambooBlock,
                axis: Axis::Y,
            }),
            200u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedBambooBlock,
                axis: Axis::Z,
            }),
            174u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedBirchLog,
                axis: Axis::X,
            }),
            175u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedBirchLog,
                axis: Axis::Y,
            }),
            176u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedBirchLog,
                axis: Axis::Z,
            }),
            231u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedBirchWood,
                axis: Axis::X,
            }),
            232u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedBirchWood,
                axis: Axis::Y,
            }),
            233u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedBirchWood,
                axis: Axis::Z,
            }),
            183u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedCherryLog,
                axis: Axis::X,
            }),
            184u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedCherryLog,
                axis: Axis::Y,
            }),
            185u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedCherryLog,
                axis: Axis::Z,
            }),
            240u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedCherryWood,
                axis: Axis::X,
            }),
            241u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedCherryWood,
                axis: Axis::Y,
            }),
            242u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedCherryWood,
                axis: Axis::Z,
            }),
            19628u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedCrimsonHyphae,
                axis: Axis::X,
            }),
            19629u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedCrimsonHyphae,
                axis: Axis::Y,
            }),
            19630u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedCrimsonHyphae,
                axis: Axis::Z,
            }),
            19622u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedCrimsonStem,
                axis: Axis::X,
            }),
            19623u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedCrimsonStem,
                axis: Axis::Y,
            }),
            19624u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedCrimsonStem,
                axis: Axis::Z,
            }),
            186u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedDarkOakLog,
                axis: Axis::X,
            }),
            187u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedDarkOakLog,
                axis: Axis::Y,
            }),
            188u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedDarkOakLog,
                axis: Axis::Z,
            }),
            243u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedDarkOakWood,
                axis: Axis::X,
            }),
            244u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedDarkOakWood,
                axis: Axis::Y,
            }),
            245u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedDarkOakWood,
                axis: Axis::Z,
            }),
            177u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedJungleLog,
                axis: Axis::X,
            }),
            178u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedJungleLog,
                axis: Axis::Y,
            }),
            179u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedJungleLog,
                axis: Axis::Z,
            }),
            234u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedJungleWood,
                axis: Axis::X,
            }),
            235u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedJungleWood,
                axis: Axis::Y,
            }),
            236u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedJungleWood,
                axis: Axis::Z,
            }),
            195u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedMangroveLog,
                axis: Axis::X,
            }),
            196u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedMangroveLog,
                axis: Axis::Y,
            }),
            197u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedMangroveLog,
                axis: Axis::Z,
            }),
            249u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedMangroveWood,
                axis: Axis::X,
            }),
            250u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedMangroveWood,
                axis: Axis::Y,
            }),
            251u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedMangroveWood,
                axis: Axis::Z,
            }),
            192u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedOakLog,
                axis: Axis::X,
            }),
            193u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedOakLog,
                axis: Axis::Y,
            }),
            194u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedOakLog,
                axis: Axis::Z,
            }),
            225u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedOakWood,
                axis: Axis::X,
            }),
            226u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedOakWood,
                axis: Axis::Y,
            }),
            227u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedOakWood,
                axis: Axis::Z,
            }),
            189u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedPaleOakLog,
                axis: Axis::X,
            }),
            190u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedPaleOakLog,
                axis: Axis::Y,
            }),
            191u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedPaleOakLog,
                axis: Axis::Z,
            }),
            246u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedPaleOakWood,
                axis: Axis::X,
            }),
            247u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedPaleOakWood,
                axis: Axis::Y,
            }),
            248u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedPaleOakWood,
                axis: Axis::Z,
            }),
            171u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedSpruceLog,
                axis: Axis::X,
            }),
            172u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedSpruceLog,
                axis: Axis::Y,
            }),
            173u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedSpruceLog,
                axis: Axis::Z,
            }),
            228u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedSpruceWood,
                axis: Axis::X,
            }),
            229u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedSpruceWood,
                axis: Axis::Y,
            }),
            230u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedSpruceWood,
                axis: Axis::Z,
            }),
            19611u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedWarpedHyphae,
                axis: Axis::X,
            }),
            19612u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedWarpedHyphae,
                axis: Axis::Y,
            }),
            19613u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedWarpedHyphae,
                axis: Axis::Z,
            }),
            19605u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedWarpedStem,
                axis: Axis::X,
            }),
            19606u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedWarpedStem,
                axis: Axis::Y,
            }),
            19607u32 => Ok(PillarBlock {
                block_type: PillarBlockType::StrippedWarpedStem,
                axis: Axis::Z,
            }),
            27626u32 => Ok(PillarBlock {
                block_type: PillarBlockType::VerdantFroglight,
                axis: Axis::X,
            }),
            27627u32 => Ok(PillarBlock {
                block_type: PillarBlockType::VerdantFroglight,
                axis: Axis::Y,
            }),
            27628u32 => Ok(PillarBlock {
                block_type: PillarBlockType::VerdantFroglight,
                axis: Axis::Z,
            }),
            19608u32 => Ok(PillarBlock {
                block_type: PillarBlockType::WarpedHyphae,
                axis: Axis::X,
            }),
            19609u32 => Ok(PillarBlock {
                block_type: PillarBlockType::WarpedHyphae,
                axis: Axis::Y,
            }),
            19610u32 => Ok(PillarBlock {
                block_type: PillarBlockType::WarpedHyphae,
                axis: Axis::Z,
            }),
            19602u32 => Ok(PillarBlock {
                block_type: PillarBlockType::WarpedStem,
                axis: Axis::X,
            }),
            19603u32 => Ok(PillarBlock {
                block_type: PillarBlockType::WarpedStem,
                axis: Axis::Y,
            }),
            19604u32 => Ok(PillarBlock {
                block_type: PillarBlockType::WarpedStem,
                axis: Axis::Z,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for PillarBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            PillarBlock {
                block_type: PillarBlockType::AcaciaLog,
                axis: Axis::X,
            } => Ok(148u32),
            PillarBlock {
                block_type: PillarBlockType::AcaciaLog,
                axis: Axis::Y,
            } => Ok(149u32),
            PillarBlock {
                block_type: PillarBlockType::AcaciaLog,
                axis: Axis::Z,
            } => Ok(150u32),
            PillarBlock {
                block_type: PillarBlockType::AcaciaWood,
                axis: Axis::X,
            } => Ok(213u32),
            PillarBlock {
                block_type: PillarBlockType::AcaciaWood,
                axis: Axis::Y,
            } => Ok(214u32),
            PillarBlock {
                block_type: PillarBlockType::AcaciaWood,
                axis: Axis::Z,
            } => Ok(215u32),
            PillarBlock {
                block_type: PillarBlockType::BambooBlock,
                axis: Axis::X,
            } => Ok(168u32),
            PillarBlock {
                block_type: PillarBlockType::BambooBlock,
                axis: Axis::Y,
            } => Ok(169u32),
            PillarBlock {
                block_type: PillarBlockType::BambooBlock,
                axis: Axis::Z,
            } => Ok(170u32),
            PillarBlock {
                block_type: PillarBlockType::Basalt,
                axis: Axis::X,
            } => Ok(6031u32),
            PillarBlock {
                block_type: PillarBlockType::Basalt,
                axis: Axis::Y,
            } => Ok(6032u32),
            PillarBlock {
                block_type: PillarBlockType::Basalt,
                axis: Axis::Z,
            } => Ok(6033u32),
            PillarBlock {
                block_type: PillarBlockType::BirchLog,
                axis: Axis::X,
            } => Ok(142u32),
            PillarBlock {
                block_type: PillarBlockType::BirchLog,
                axis: Axis::Y,
            } => Ok(143u32),
            PillarBlock {
                block_type: PillarBlockType::BirchLog,
                axis: Axis::Z,
            } => Ok(144u32),
            PillarBlock {
                block_type: PillarBlockType::BirchWood,
                axis: Axis::X,
            } => Ok(207u32),
            PillarBlock {
                block_type: PillarBlockType::BirchWood,
                axis: Axis::Y,
            } => Ok(208u32),
            PillarBlock {
                block_type: PillarBlockType::BirchWood,
                axis: Axis::Z,
            } => Ok(209u32),
            PillarBlock {
                block_type: PillarBlockType::BoneBlock,
                axis: Axis::X,
            } => Ok(13569u32),
            PillarBlock {
                block_type: PillarBlockType::BoneBlock,
                axis: Axis::Y,
            } => Ok(13570u32),
            PillarBlock {
                block_type: PillarBlockType::BoneBlock,
                axis: Axis::Z,
            } => Ok(13571u32),
            PillarBlock {
                block_type: PillarBlockType::CherryLog,
                axis: Axis::X,
            } => Ok(151u32),
            PillarBlock {
                block_type: PillarBlockType::CherryLog,
                axis: Axis::Y,
            } => Ok(152u32),
            PillarBlock {
                block_type: PillarBlockType::CherryLog,
                axis: Axis::Z,
            } => Ok(153u32),
            PillarBlock {
                block_type: PillarBlockType::CherryWood,
                axis: Axis::X,
            } => Ok(216u32),
            PillarBlock {
                block_type: PillarBlockType::CherryWood,
                axis: Axis::Y,
            } => Ok(217u32),
            PillarBlock {
                block_type: PillarBlockType::CherryWood,
                axis: Axis::Z,
            } => Ok(218u32),
            PillarBlock {
                block_type: PillarBlockType::CrimsonHyphae,
                axis: Axis::X,
            } => Ok(19625u32),
            PillarBlock {
                block_type: PillarBlockType::CrimsonHyphae,
                axis: Axis::Y,
            } => Ok(19626u32),
            PillarBlock {
                block_type: PillarBlockType::CrimsonHyphae,
                axis: Axis::Z,
            } => Ok(19627u32),
            PillarBlock {
                block_type: PillarBlockType::CrimsonStem,
                axis: Axis::X,
            } => Ok(19619u32),
            PillarBlock {
                block_type: PillarBlockType::CrimsonStem,
                axis: Axis::Y,
            } => Ok(19620u32),
            PillarBlock {
                block_type: PillarBlockType::CrimsonStem,
                axis: Axis::Z,
            } => Ok(19621u32),
            PillarBlock {
                block_type: PillarBlockType::DarkOakLog,
                axis: Axis::X,
            } => Ok(154u32),
            PillarBlock {
                block_type: PillarBlockType::DarkOakLog,
                axis: Axis::Y,
            } => Ok(155u32),
            PillarBlock {
                block_type: PillarBlockType::DarkOakLog,
                axis: Axis::Z,
            } => Ok(156u32),
            PillarBlock {
                block_type: PillarBlockType::DarkOakWood,
                axis: Axis::X,
            } => Ok(219u32),
            PillarBlock {
                block_type: PillarBlockType::DarkOakWood,
                axis: Axis::Y,
            } => Ok(220u32),
            PillarBlock {
                block_type: PillarBlockType::DarkOakWood,
                axis: Axis::Z,
            } => Ok(221u32),
            PillarBlock {
                block_type: PillarBlockType::Deepslate,
                axis: Axis::X,
            } => Ok(25964u32),
            PillarBlock {
                block_type: PillarBlockType::Deepslate,
                axis: Axis::Y,
            } => Ok(25965u32),
            PillarBlock {
                block_type: PillarBlockType::Deepslate,
                axis: Axis::Z,
            } => Ok(25966u32),
            PillarBlock {
                block_type: PillarBlockType::HayBlock,
                axis: Axis::X,
            } => Ok(11614u32),
            PillarBlock {
                block_type: PillarBlockType::HayBlock,
                axis: Axis::Y,
            } => Ok(11615u32),
            PillarBlock {
                block_type: PillarBlockType::HayBlock,
                axis: Axis::Z,
            } => Ok(11616u32),
            PillarBlock {
                block_type: PillarBlockType::InfestedDeepslate,
                axis: Axis::X,
            } => Ok(27614u32),
            PillarBlock {
                block_type: PillarBlockType::InfestedDeepslate,
                axis: Axis::Y,
            } => Ok(27615u32),
            PillarBlock {
                block_type: PillarBlockType::InfestedDeepslate,
                axis: Axis::Z,
            } => Ok(27616u32),
            PillarBlock {
                block_type: PillarBlockType::JungleLog,
                axis: Axis::X,
            } => Ok(145u32),
            PillarBlock {
                block_type: PillarBlockType::JungleLog,
                axis: Axis::Y,
            } => Ok(146u32),
            PillarBlock {
                block_type: PillarBlockType::JungleLog,
                axis: Axis::Z,
            } => Ok(147u32),
            PillarBlock {
                block_type: PillarBlockType::JungleWood,
                axis: Axis::X,
            } => Ok(210u32),
            PillarBlock {
                block_type: PillarBlockType::JungleWood,
                axis: Axis::Y,
            } => Ok(211u32),
            PillarBlock {
                block_type: PillarBlockType::JungleWood,
                axis: Axis::Z,
            } => Ok(212u32),
            PillarBlock {
                block_type: PillarBlockType::MangroveLog,
                axis: Axis::X,
            } => Ok(160u32),
            PillarBlock {
                block_type: PillarBlockType::MangroveLog,
                axis: Axis::Y,
            } => Ok(161u32),
            PillarBlock {
                block_type: PillarBlockType::MangroveLog,
                axis: Axis::Z,
            } => Ok(162u32),
            PillarBlock {
                block_type: PillarBlockType::MangroveWood,
                axis: Axis::X,
            } => Ok(222u32),
            PillarBlock {
                block_type: PillarBlockType::MangroveWood,
                axis: Axis::Y,
            } => Ok(223u32),
            PillarBlock {
                block_type: PillarBlockType::MangroveWood,
                axis: Axis::Z,
            } => Ok(224u32),
            PillarBlock {
                block_type: PillarBlockType::MuddyMangroveRoots,
                axis: Axis::X,
            } => Ok(165u32),
            PillarBlock {
                block_type: PillarBlockType::MuddyMangroveRoots,
                axis: Axis::Y,
            } => Ok(166u32),
            PillarBlock {
                block_type: PillarBlockType::MuddyMangroveRoots,
                axis: Axis::Z,
            } => Ok(167u32),
            PillarBlock {
                block_type: PillarBlockType::NetherPortal,
                axis: Axis::X,
            } => Ok(6043u32),
            PillarBlock {
                block_type: PillarBlockType::NetherPortal,
                axis: Axis::Z,
            } => Ok(6044u32),
            PillarBlock {
                block_type: PillarBlockType::OakLog,
                axis: Axis::X,
            } => Ok(136u32),
            PillarBlock {
                block_type: PillarBlockType::OakLog,
                axis: Axis::Y,
            } => Ok(137u32),
            PillarBlock {
                block_type: PillarBlockType::OakLog,
                axis: Axis::Z,
            } => Ok(138u32),
            PillarBlock {
                block_type: PillarBlockType::OakWood,
                axis: Axis::X,
            } => Ok(201u32),
            PillarBlock {
                block_type: PillarBlockType::OakWood,
                axis: Axis::Y,
            } => Ok(202u32),
            PillarBlock {
                block_type: PillarBlockType::OakWood,
                axis: Axis::Z,
            } => Ok(203u32),
            PillarBlock {
                block_type: PillarBlockType::OchreFroglight,
                axis: Axis::X,
            } => Ok(27623u32),
            PillarBlock {
                block_type: PillarBlockType::OchreFroglight,
                axis: Axis::Y,
            } => Ok(27624u32),
            PillarBlock {
                block_type: PillarBlockType::OchreFroglight,
                axis: Axis::Z,
            } => Ok(27625u32),
            PillarBlock {
                block_type: PillarBlockType::PaleOakLog,
                axis: Axis::X,
            } => Ok(157u32),
            PillarBlock {
                block_type: PillarBlockType::PaleOakLog,
                axis: Axis::Y,
            } => Ok(158u32),
            PillarBlock {
                block_type: PillarBlockType::PaleOakLog,
                axis: Axis::Z,
            } => Ok(159u32),
            PillarBlock {
                block_type: PillarBlockType::PaleOakWood,
                axis: Axis::X,
            } => Ok(22u32),
            PillarBlock {
                block_type: PillarBlockType::PaleOakWood,
                axis: Axis::Y,
            } => Ok(23u32),
            PillarBlock {
                block_type: PillarBlockType::PaleOakWood,
                axis: Axis::Z,
            } => Ok(24u32),
            PillarBlock {
                block_type: PillarBlockType::PearlescentFroglight,
                axis: Axis::X,
            } => Ok(27629u32),
            PillarBlock {
                block_type: PillarBlockType::PearlescentFroglight,
                axis: Axis::Y,
            } => Ok(27630u32),
            PillarBlock {
                block_type: PillarBlockType::PearlescentFroglight,
                axis: Axis::Z,
            } => Ok(27631u32),
            PillarBlock {
                block_type: PillarBlockType::PolishedBasalt,
                axis: Axis::X,
            } => Ok(6034u32),
            PillarBlock {
                block_type: PillarBlockType::PolishedBasalt,
                axis: Axis::Y,
            } => Ok(6035u32),
            PillarBlock {
                block_type: PillarBlockType::PolishedBasalt,
                axis: Axis::Z,
            } => Ok(6036u32),
            PillarBlock {
                block_type: PillarBlockType::PurpurPillar,
                axis: Axis::X,
            } => Ok(13434u32),
            PillarBlock {
                block_type: PillarBlockType::PurpurPillar,
                axis: Axis::Y,
            } => Ok(13435u32),
            PillarBlock {
                block_type: PillarBlockType::PurpurPillar,
                axis: Axis::Z,
            } => Ok(13436u32),
            PillarBlock {
                block_type: PillarBlockType::QuartzPillar,
                axis: Axis::X,
            } => Ok(10046u32),
            PillarBlock {
                block_type: PillarBlockType::QuartzPillar,
                axis: Axis::Y,
            } => Ok(10047u32),
            PillarBlock {
                block_type: PillarBlockType::QuartzPillar,
                axis: Axis::Z,
            } => Ok(10048u32),
            PillarBlock {
                block_type: PillarBlockType::SpruceLog,
                axis: Axis::X,
            } => Ok(139u32),
            PillarBlock {
                block_type: PillarBlockType::SpruceLog,
                axis: Axis::Y,
            } => Ok(140u32),
            PillarBlock {
                block_type: PillarBlockType::SpruceLog,
                axis: Axis::Z,
            } => Ok(141u32),
            PillarBlock {
                block_type: PillarBlockType::SpruceWood,
                axis: Axis::X,
            } => Ok(204u32),
            PillarBlock {
                block_type: PillarBlockType::SpruceWood,
                axis: Axis::Y,
            } => Ok(205u32),
            PillarBlock {
                block_type: PillarBlockType::SpruceWood,
                axis: Axis::Z,
            } => Ok(206u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedAcaciaLog,
                axis: Axis::X,
            } => Ok(180u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedAcaciaLog,
                axis: Axis::Y,
            } => Ok(181u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedAcaciaLog,
                axis: Axis::Z,
            } => Ok(182u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedAcaciaWood,
                axis: Axis::X,
            } => Ok(237u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedAcaciaWood,
                axis: Axis::Y,
            } => Ok(238u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedAcaciaWood,
                axis: Axis::Z,
            } => Ok(239u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedBambooBlock,
                axis: Axis::X,
            } => Ok(198u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedBambooBlock,
                axis: Axis::Y,
            } => Ok(199u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedBambooBlock,
                axis: Axis::Z,
            } => Ok(200u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedBirchLog,
                axis: Axis::X,
            } => Ok(174u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedBirchLog,
                axis: Axis::Y,
            } => Ok(175u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedBirchLog,
                axis: Axis::Z,
            } => Ok(176u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedBirchWood,
                axis: Axis::X,
            } => Ok(231u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedBirchWood,
                axis: Axis::Y,
            } => Ok(232u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedBirchWood,
                axis: Axis::Z,
            } => Ok(233u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedCherryLog,
                axis: Axis::X,
            } => Ok(183u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedCherryLog,
                axis: Axis::Y,
            } => Ok(184u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedCherryLog,
                axis: Axis::Z,
            } => Ok(185u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedCherryWood,
                axis: Axis::X,
            } => Ok(240u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedCherryWood,
                axis: Axis::Y,
            } => Ok(241u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedCherryWood,
                axis: Axis::Z,
            } => Ok(242u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedCrimsonHyphae,
                axis: Axis::X,
            } => Ok(19628u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedCrimsonHyphae,
                axis: Axis::Y,
            } => Ok(19629u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedCrimsonHyphae,
                axis: Axis::Z,
            } => Ok(19630u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedCrimsonStem,
                axis: Axis::X,
            } => Ok(19622u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedCrimsonStem,
                axis: Axis::Y,
            } => Ok(19623u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedCrimsonStem,
                axis: Axis::Z,
            } => Ok(19624u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedDarkOakLog,
                axis: Axis::X,
            } => Ok(186u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedDarkOakLog,
                axis: Axis::Y,
            } => Ok(187u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedDarkOakLog,
                axis: Axis::Z,
            } => Ok(188u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedDarkOakWood,
                axis: Axis::X,
            } => Ok(243u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedDarkOakWood,
                axis: Axis::Y,
            } => Ok(244u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedDarkOakWood,
                axis: Axis::Z,
            } => Ok(245u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedJungleLog,
                axis: Axis::X,
            } => Ok(177u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedJungleLog,
                axis: Axis::Y,
            } => Ok(178u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedJungleLog,
                axis: Axis::Z,
            } => Ok(179u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedJungleWood,
                axis: Axis::X,
            } => Ok(234u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedJungleWood,
                axis: Axis::Y,
            } => Ok(235u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedJungleWood,
                axis: Axis::Z,
            } => Ok(236u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedMangroveLog,
                axis: Axis::X,
            } => Ok(195u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedMangroveLog,
                axis: Axis::Y,
            } => Ok(196u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedMangroveLog,
                axis: Axis::Z,
            } => Ok(197u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedMangroveWood,
                axis: Axis::X,
            } => Ok(249u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedMangroveWood,
                axis: Axis::Y,
            } => Ok(250u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedMangroveWood,
                axis: Axis::Z,
            } => Ok(251u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedOakLog,
                axis: Axis::X,
            } => Ok(192u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedOakLog,
                axis: Axis::Y,
            } => Ok(193u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedOakLog,
                axis: Axis::Z,
            } => Ok(194u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedOakWood,
                axis: Axis::X,
            } => Ok(225u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedOakWood,
                axis: Axis::Y,
            } => Ok(226u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedOakWood,
                axis: Axis::Z,
            } => Ok(227u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedPaleOakLog,
                axis: Axis::X,
            } => Ok(189u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedPaleOakLog,
                axis: Axis::Y,
            } => Ok(190u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedPaleOakLog,
                axis: Axis::Z,
            } => Ok(191u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedPaleOakWood,
                axis: Axis::X,
            } => Ok(246u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedPaleOakWood,
                axis: Axis::Y,
            } => Ok(247u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedPaleOakWood,
                axis: Axis::Z,
            } => Ok(248u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedSpruceLog,
                axis: Axis::X,
            } => Ok(171u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedSpruceLog,
                axis: Axis::Y,
            } => Ok(172u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedSpruceLog,
                axis: Axis::Z,
            } => Ok(173u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedSpruceWood,
                axis: Axis::X,
            } => Ok(228u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedSpruceWood,
                axis: Axis::Y,
            } => Ok(229u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedSpruceWood,
                axis: Axis::Z,
            } => Ok(230u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedWarpedHyphae,
                axis: Axis::X,
            } => Ok(19611u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedWarpedHyphae,
                axis: Axis::Y,
            } => Ok(19612u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedWarpedHyphae,
                axis: Axis::Z,
            } => Ok(19613u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedWarpedStem,
                axis: Axis::X,
            } => Ok(19605u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedWarpedStem,
                axis: Axis::Y,
            } => Ok(19606u32),
            PillarBlock {
                block_type: PillarBlockType::StrippedWarpedStem,
                axis: Axis::Z,
            } => Ok(19607u32),
            PillarBlock {
                block_type: PillarBlockType::VerdantFroglight,
                axis: Axis::X,
            } => Ok(27626u32),
            PillarBlock {
                block_type: PillarBlockType::VerdantFroglight,
                axis: Axis::Y,
            } => Ok(27627u32),
            PillarBlock {
                block_type: PillarBlockType::VerdantFroglight,
                axis: Axis::Z,
            } => Ok(27628u32),
            PillarBlock {
                block_type: PillarBlockType::WarpedHyphae,
                axis: Axis::X,
            } => Ok(19608u32),
            PillarBlock {
                block_type: PillarBlockType::WarpedHyphae,
                axis: Axis::Y,
            } => Ok(19609u32),
            PillarBlock {
                block_type: PillarBlockType::WarpedHyphae,
                axis: Axis::Z,
            } => Ok(19610u32),
            PillarBlock {
                block_type: PillarBlockType::WarpedStem,
                axis: Axis::X,
            } => Ok(19602u32),
            PillarBlock {
                block_type: PillarBlockType::WarpedStem,
                axis: Axis::Y,
            } => Ok(19603u32),
            PillarBlock {
                block_type: PillarBlockType::WarpedStem,
                axis: Axis::Z,
            } => Ok(19604u32),
            _ => Err(()),
        }
    }
}

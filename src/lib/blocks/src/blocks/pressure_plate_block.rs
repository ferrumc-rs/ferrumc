#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum PressurePlateBlockType {
    AcaciaPressurePlate,
    BambooPressurePlate,
    BirchPressurePlate,
    CherryPressurePlate,
    CrimsonPressurePlate,
    DarkOakPressurePlate,
    JunglePressurePlate,
    MangrovePressurePlate,
    OakPressurePlate,
    PaleOakPressurePlate,
    PolishedBlackstonePressurePlate,
    SprucePressurePlate,
    StonePressurePlate,
    WarpedPressurePlate,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PressurePlateBlock {
    pub block_type: PressurePlateBlockType,
    pub powered: bool,
}
impl PressurePlateBlock {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<PressurePlateBlock>();
}
impl TryFrom<u32> for PressurePlateBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            5900u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::AcaciaPressurePlate,
                powered: true,
            }),
            5901u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::AcaciaPressurePlate,
                powered: false,
            }),
            5910u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::BambooPressurePlate,
                powered: true,
            }),
            5911u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::BambooPressurePlate,
                powered: false,
            }),
            5896u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::BirchPressurePlate,
                powered: true,
            }),
            5897u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::BirchPressurePlate,
                powered: false,
            }),
            5902u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::CherryPressurePlate,
                powered: true,
            }),
            5903u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::CherryPressurePlate,
                powered: false,
            }),
            19703u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::CrimsonPressurePlate,
                powered: true,
            }),
            19704u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::CrimsonPressurePlate,
                powered: false,
            }),
            5904u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::DarkOakPressurePlate,
                powered: true,
            }),
            5905u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::DarkOakPressurePlate,
                powered: false,
            }),
            5898u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::JunglePressurePlate,
                powered: true,
            }),
            5899u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::JunglePressurePlate,
                powered: false,
            }),
            5908u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::MangrovePressurePlate,
                powered: true,
            }),
            5909u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::MangrovePressurePlate,
                powered: false,
            }),
            5892u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::OakPressurePlate,
                powered: true,
            }),
            5893u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::OakPressurePlate,
                powered: false,
            }),
            5906u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::PaleOakPressurePlate,
                powered: true,
            }),
            5907u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::PaleOakPressurePlate,
                powered: false,
            }),
            21400u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::PolishedBlackstonePressurePlate,
                powered: true,
            }),
            21401u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::PolishedBlackstonePressurePlate,
                powered: false,
            }),
            5894u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::SprucePressurePlate,
                powered: true,
            }),
            5895u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::SprucePressurePlate,
                powered: false,
            }),
            5826u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::StonePressurePlate,
                powered: true,
            }),
            5827u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::StonePressurePlate,
                powered: false,
            }),
            19705u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::WarpedPressurePlate,
                powered: true,
            }),
            19706u32 => Ok(PressurePlateBlock {
                block_type: PressurePlateBlockType::WarpedPressurePlate,
                powered: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for PressurePlateBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            PressurePlateBlock {
                block_type: PressurePlateBlockType::AcaciaPressurePlate,
                powered: true,
            } => Ok(5900u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::AcaciaPressurePlate,
                powered: false,
            } => Ok(5901u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::BambooPressurePlate,
                powered: true,
            } => Ok(5910u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::BambooPressurePlate,
                powered: false,
            } => Ok(5911u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::BirchPressurePlate,
                powered: true,
            } => Ok(5896u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::BirchPressurePlate,
                powered: false,
            } => Ok(5897u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::CherryPressurePlate,
                powered: true,
            } => Ok(5902u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::CherryPressurePlate,
                powered: false,
            } => Ok(5903u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::CrimsonPressurePlate,
                powered: true,
            } => Ok(19703u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::CrimsonPressurePlate,
                powered: false,
            } => Ok(19704u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::DarkOakPressurePlate,
                powered: true,
            } => Ok(5904u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::DarkOakPressurePlate,
                powered: false,
            } => Ok(5905u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::JunglePressurePlate,
                powered: true,
            } => Ok(5898u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::JunglePressurePlate,
                powered: false,
            } => Ok(5899u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::MangrovePressurePlate,
                powered: true,
            } => Ok(5908u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::MangrovePressurePlate,
                powered: false,
            } => Ok(5909u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::OakPressurePlate,
                powered: true,
            } => Ok(5892u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::OakPressurePlate,
                powered: false,
            } => Ok(5893u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::PaleOakPressurePlate,
                powered: true,
            } => Ok(5906u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::PaleOakPressurePlate,
                powered: false,
            } => Ok(5907u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::PolishedBlackstonePressurePlate,
                powered: true,
            } => Ok(21400u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::PolishedBlackstonePressurePlate,
                powered: false,
            } => Ok(21401u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::SprucePressurePlate,
                powered: true,
            } => Ok(5894u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::SprucePressurePlate,
                powered: false,
            } => Ok(5895u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::StonePressurePlate,
                powered: true,
            } => Ok(5826u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::StonePressurePlate,
                powered: false,
            } => Ok(5827u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::WarpedPressurePlate,
                powered: true,
            } => Ok(19705u32),
            PressurePlateBlock {
                block_type: PressurePlateBlockType::WarpedPressurePlate,
                powered: false,
            } => Ok(19706u32),
            _ => Err(()),
        }
    }
}

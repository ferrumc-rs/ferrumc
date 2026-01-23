#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct12 {
    pub axis: Axis,
    pub creaking_heart_state: CreakingHeartState,
    pub natural: bool,
}
impl TryFrom<u32> for GeneratedStruct12 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            2920u32 => Ok(GeneratedStruct12 {
                creaking_heart_state: CreakingHeartState::Uprooted,
                axis: Axis::X,
                natural: true,
            }),
            2921u32 => Ok(GeneratedStruct12 {
                axis: Axis::X,
                natural: false,
                creaking_heart_state: CreakingHeartState::Uprooted,
            }),
            2922u32 => Ok(GeneratedStruct12 {
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: true,
                axis: Axis::X,
            }),
            2923u32 => Ok(GeneratedStruct12 {
                axis: Axis::X,
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: false,
            }),
            2924u32 => Ok(GeneratedStruct12 {
                creaking_heart_state: CreakingHeartState::Awake,
                natural: true,
                axis: Axis::X,
            }),
            2925u32 => Ok(GeneratedStruct12 {
                natural: false,
                axis: Axis::X,
                creaking_heart_state: CreakingHeartState::Awake,
            }),
            2926u32 => Ok(GeneratedStruct12 {
                axis: Axis::Y,
                creaking_heart_state: CreakingHeartState::Uprooted,
                natural: true,
            }),
            2927u32 => Ok(GeneratedStruct12 {
                natural: false,
                creaking_heart_state: CreakingHeartState::Uprooted,
                axis: Axis::Y,
            }),
            2928u32 => Ok(GeneratedStruct12 {
                axis: Axis::Y,
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: true,
            }),
            2929u32 => Ok(GeneratedStruct12 {
                creaking_heart_state: CreakingHeartState::Dormant,
                axis: Axis::Y,
                natural: false,
            }),
            2930u32 => Ok(GeneratedStruct12 {
                creaking_heart_state: CreakingHeartState::Awake,
                natural: true,
                axis: Axis::Y,
            }),
            2931u32 => Ok(GeneratedStruct12 {
                creaking_heart_state: CreakingHeartState::Awake,
                axis: Axis::Y,
                natural: false,
            }),
            2932u32 => Ok(GeneratedStruct12 {
                axis: Axis::Z,
                creaking_heart_state: CreakingHeartState::Uprooted,
                natural: true,
            }),
            2933u32 => Ok(GeneratedStruct12 {
                creaking_heart_state: CreakingHeartState::Uprooted,
                axis: Axis::Z,
                natural: false,
            }),
            2934u32 => Ok(GeneratedStruct12 {
                axis: Axis::Z,
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: true,
            }),
            2935u32 => Ok(GeneratedStruct12 {
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: false,
                axis: Axis::Z,
            }),
            2936u32 => Ok(GeneratedStruct12 {
                axis: Axis::Z,
                creaking_heart_state: CreakingHeartState::Awake,
                natural: true,
            }),
            2937u32 => Ok(GeneratedStruct12 {
                natural: false,
                axis: Axis::Z,
                creaking_heart_state: CreakingHeartState::Awake,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct12 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct12 {
                creaking_heart_state: CreakingHeartState::Uprooted,
                axis: Axis::X,
                natural: true,
            } => Ok(2920u32),
            GeneratedStruct12 {
                axis: Axis::X,
                natural: false,
                creaking_heart_state: CreakingHeartState::Uprooted,
            } => Ok(2921u32),
            GeneratedStruct12 {
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: true,
                axis: Axis::X,
            } => Ok(2922u32),
            GeneratedStruct12 {
                axis: Axis::X,
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: false,
            } => Ok(2923u32),
            GeneratedStruct12 {
                creaking_heart_state: CreakingHeartState::Awake,
                natural: true,
                axis: Axis::X,
            } => Ok(2924u32),
            GeneratedStruct12 {
                natural: false,
                axis: Axis::X,
                creaking_heart_state: CreakingHeartState::Awake,
            } => Ok(2925u32),
            GeneratedStruct12 {
                axis: Axis::Y,
                creaking_heart_state: CreakingHeartState::Uprooted,
                natural: true,
            } => Ok(2926u32),
            GeneratedStruct12 {
                natural: false,
                creaking_heart_state: CreakingHeartState::Uprooted,
                axis: Axis::Y,
            } => Ok(2927u32),
            GeneratedStruct12 {
                axis: Axis::Y,
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: true,
            } => Ok(2928u32),
            GeneratedStruct12 {
                creaking_heart_state: CreakingHeartState::Dormant,
                axis: Axis::Y,
                natural: false,
            } => Ok(2929u32),
            GeneratedStruct12 {
                creaking_heart_state: CreakingHeartState::Awake,
                natural: true,
                axis: Axis::Y,
            } => Ok(2930u32),
            GeneratedStruct12 {
                creaking_heart_state: CreakingHeartState::Awake,
                axis: Axis::Y,
                natural: false,
            } => Ok(2931u32),
            GeneratedStruct12 {
                axis: Axis::Z,
                creaking_heart_state: CreakingHeartState::Uprooted,
                natural: true,
            } => Ok(2932u32),
            GeneratedStruct12 {
                creaking_heart_state: CreakingHeartState::Uprooted,
                axis: Axis::Z,
                natural: false,
            } => Ok(2933u32),
            GeneratedStruct12 {
                axis: Axis::Z,
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: true,
            } => Ok(2934u32),
            GeneratedStruct12 {
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: false,
                axis: Axis::Z,
            } => Ok(2935u32),
            GeneratedStruct12 {
                axis: Axis::Z,
                creaking_heart_state: CreakingHeartState::Awake,
                natural: true,
            } => Ok(2936u32),
            GeneratedStruct12 {
                natural: false,
                axis: Axis::Z,
                creaking_heart_state: CreakingHeartState::Awake,
            } => Ok(2937u32),
            _ => Err(()),
        }
    }
}

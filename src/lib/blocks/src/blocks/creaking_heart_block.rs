#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CreakingHeartBlock {
    pub axis: Axis,
    pub creaking_heart_state: CreakingHeartState,
    pub natural: bool,
}
impl CreakingHeartBlock {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<CreakingHeartBlock>();
}
impl TryFrom<u32> for CreakingHeartBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            2920u32 => Ok(CreakingHeartBlock {
                axis: Axis::X,
                creaking_heart_state: CreakingHeartState::Uprooted,
                natural: true,
            }),
            2921u32 => Ok(CreakingHeartBlock {
                axis: Axis::X,
                creaking_heart_state: CreakingHeartState::Uprooted,
                natural: false,
            }),
            2922u32 => Ok(CreakingHeartBlock {
                axis: Axis::X,
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: true,
            }),
            2923u32 => Ok(CreakingHeartBlock {
                axis: Axis::X,
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: false,
            }),
            2924u32 => Ok(CreakingHeartBlock {
                axis: Axis::X,
                creaking_heart_state: CreakingHeartState::Awake,
                natural: true,
            }),
            2925u32 => Ok(CreakingHeartBlock {
                axis: Axis::X,
                creaking_heart_state: CreakingHeartState::Awake,
                natural: false,
            }),
            2926u32 => Ok(CreakingHeartBlock {
                axis: Axis::Y,
                creaking_heart_state: CreakingHeartState::Uprooted,
                natural: true,
            }),
            2927u32 => Ok(CreakingHeartBlock {
                axis: Axis::Y,
                creaking_heart_state: CreakingHeartState::Uprooted,
                natural: false,
            }),
            2928u32 => Ok(CreakingHeartBlock {
                axis: Axis::Y,
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: true,
            }),
            2929u32 => Ok(CreakingHeartBlock {
                axis: Axis::Y,
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: false,
            }),
            2930u32 => Ok(CreakingHeartBlock {
                axis: Axis::Y,
                creaking_heart_state: CreakingHeartState::Awake,
                natural: true,
            }),
            2931u32 => Ok(CreakingHeartBlock {
                axis: Axis::Y,
                creaking_heart_state: CreakingHeartState::Awake,
                natural: false,
            }),
            2932u32 => Ok(CreakingHeartBlock {
                axis: Axis::Z,
                creaking_heart_state: CreakingHeartState::Uprooted,
                natural: true,
            }),
            2933u32 => Ok(CreakingHeartBlock {
                axis: Axis::Z,
                creaking_heart_state: CreakingHeartState::Uprooted,
                natural: false,
            }),
            2934u32 => Ok(CreakingHeartBlock {
                axis: Axis::Z,
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: true,
            }),
            2935u32 => Ok(CreakingHeartBlock {
                axis: Axis::Z,
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: false,
            }),
            2936u32 => Ok(CreakingHeartBlock {
                axis: Axis::Z,
                creaking_heart_state: CreakingHeartState::Awake,
                natural: true,
            }),
            2937u32 => Ok(CreakingHeartBlock {
                axis: Axis::Z,
                creaking_heart_state: CreakingHeartState::Awake,
                natural: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for CreakingHeartBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            CreakingHeartBlock {
                axis: Axis::X,
                creaking_heart_state: CreakingHeartState::Uprooted,
                natural: true,
            } => Ok(2920u32),
            CreakingHeartBlock {
                axis: Axis::X,
                creaking_heart_state: CreakingHeartState::Uprooted,
                natural: false,
            } => Ok(2921u32),
            CreakingHeartBlock {
                axis: Axis::X,
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: true,
            } => Ok(2922u32),
            CreakingHeartBlock {
                axis: Axis::X,
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: false,
            } => Ok(2923u32),
            CreakingHeartBlock {
                axis: Axis::X,
                creaking_heart_state: CreakingHeartState::Awake,
                natural: true,
            } => Ok(2924u32),
            CreakingHeartBlock {
                axis: Axis::X,
                creaking_heart_state: CreakingHeartState::Awake,
                natural: false,
            } => Ok(2925u32),
            CreakingHeartBlock {
                axis: Axis::Y,
                creaking_heart_state: CreakingHeartState::Uprooted,
                natural: true,
            } => Ok(2926u32),
            CreakingHeartBlock {
                axis: Axis::Y,
                creaking_heart_state: CreakingHeartState::Uprooted,
                natural: false,
            } => Ok(2927u32),
            CreakingHeartBlock {
                axis: Axis::Y,
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: true,
            } => Ok(2928u32),
            CreakingHeartBlock {
                axis: Axis::Y,
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: false,
            } => Ok(2929u32),
            CreakingHeartBlock {
                axis: Axis::Y,
                creaking_heart_state: CreakingHeartState::Awake,
                natural: true,
            } => Ok(2930u32),
            CreakingHeartBlock {
                axis: Axis::Y,
                creaking_heart_state: CreakingHeartState::Awake,
                natural: false,
            } => Ok(2931u32),
            CreakingHeartBlock {
                axis: Axis::Z,
                creaking_heart_state: CreakingHeartState::Uprooted,
                natural: true,
            } => Ok(2932u32),
            CreakingHeartBlock {
                axis: Axis::Z,
                creaking_heart_state: CreakingHeartState::Uprooted,
                natural: false,
            } => Ok(2933u32),
            CreakingHeartBlock {
                axis: Axis::Z,
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: true,
            } => Ok(2934u32),
            CreakingHeartBlock {
                axis: Axis::Z,
                creaking_heart_state: CreakingHeartState::Dormant,
                natural: false,
            } => Ok(2935u32),
            CreakingHeartBlock {
                axis: Axis::Z,
                creaking_heart_state: CreakingHeartState::Awake,
                natural: true,
            } => Ok(2936u32),
            CreakingHeartBlock {
                axis: Axis::Z,
                creaking_heart_state: CreakingHeartState::Awake,
                natural: false,
            } => Ok(2937u32),
            _ => Err(()),
        }
    }
}

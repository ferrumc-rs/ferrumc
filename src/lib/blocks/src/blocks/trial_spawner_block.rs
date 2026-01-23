#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TrialSpawnerBlock {
    pub ominous: bool,
    pub trial_spawner_state: TrialSpawnerState,
}
impl TrialSpawnerBlock {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<TrialSpawnerBlock>();
}
impl TryFrom<u32> for TrialSpawnerBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            27698u32 => Ok(TrialSpawnerBlock {
                ominous: true,
                trial_spawner_state: TrialSpawnerState::Inactive,
            }),
            27699u32 => Ok(TrialSpawnerBlock {
                ominous: true,
                trial_spawner_state: TrialSpawnerState::WaitingForPlayers,
            }),
            27700u32 => Ok(TrialSpawnerBlock {
                ominous: true,
                trial_spawner_state: TrialSpawnerState::Active,
            }),
            27701u32 => Ok(TrialSpawnerBlock {
                ominous: true,
                trial_spawner_state: TrialSpawnerState::WaitingForRewardEjection,
            }),
            27702u32 => Ok(TrialSpawnerBlock {
                ominous: true,
                trial_spawner_state: TrialSpawnerState::EjectingReward,
            }),
            27703u32 => Ok(TrialSpawnerBlock {
                ominous: true,
                trial_spawner_state: TrialSpawnerState::Cooldown,
            }),
            27704u32 => Ok(TrialSpawnerBlock {
                ominous: false,
                trial_spawner_state: TrialSpawnerState::Inactive,
            }),
            27705u32 => Ok(TrialSpawnerBlock {
                ominous: false,
                trial_spawner_state: TrialSpawnerState::WaitingForPlayers,
            }),
            27706u32 => Ok(TrialSpawnerBlock {
                ominous: false,
                trial_spawner_state: TrialSpawnerState::Active,
            }),
            27707u32 => Ok(TrialSpawnerBlock {
                ominous: false,
                trial_spawner_state: TrialSpawnerState::WaitingForRewardEjection,
            }),
            27708u32 => Ok(TrialSpawnerBlock {
                ominous: false,
                trial_spawner_state: TrialSpawnerState::EjectingReward,
            }),
            27709u32 => Ok(TrialSpawnerBlock {
                ominous: false,
                trial_spawner_state: TrialSpawnerState::Cooldown,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for TrialSpawnerBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            TrialSpawnerBlock {
                ominous: true,
                trial_spawner_state: TrialSpawnerState::Inactive,
            } => Ok(27698u32),
            TrialSpawnerBlock {
                ominous: true,
                trial_spawner_state: TrialSpawnerState::WaitingForPlayers,
            } => Ok(27699u32),
            TrialSpawnerBlock {
                ominous: true,
                trial_spawner_state: TrialSpawnerState::Active,
            } => Ok(27700u32),
            TrialSpawnerBlock {
                ominous: true,
                trial_spawner_state: TrialSpawnerState::WaitingForRewardEjection,
            } => Ok(27701u32),
            TrialSpawnerBlock {
                ominous: true,
                trial_spawner_state: TrialSpawnerState::EjectingReward,
            } => Ok(27702u32),
            TrialSpawnerBlock {
                ominous: true,
                trial_spawner_state: TrialSpawnerState::Cooldown,
            } => Ok(27703u32),
            TrialSpawnerBlock {
                ominous: false,
                trial_spawner_state: TrialSpawnerState::Inactive,
            } => Ok(27704u32),
            TrialSpawnerBlock {
                ominous: false,
                trial_spawner_state: TrialSpawnerState::WaitingForPlayers,
            } => Ok(27705u32),
            TrialSpawnerBlock {
                ominous: false,
                trial_spawner_state: TrialSpawnerState::Active,
            } => Ok(27706u32),
            TrialSpawnerBlock {
                ominous: false,
                trial_spawner_state: TrialSpawnerState::WaitingForRewardEjection,
            } => Ok(27707u32),
            TrialSpawnerBlock {
                ominous: false,
                trial_spawner_state: TrialSpawnerState::EjectingReward,
            } => Ok(27708u32),
            TrialSpawnerBlock {
                ominous: false,
                trial_spawner_state: TrialSpawnerState::Cooldown,
            } => Ok(27709u32),
            _ => Err(()),
        }
    }
}

#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct82 {
    pub ominous: bool,
    pub trial_spawner_state: TrialSpawnerState,
}
impl TryFrom<u32> for GeneratedStruct82 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            27698u32 => Ok(GeneratedStruct82 {
                trial_spawner_state: TrialSpawnerState::Inactive,
                ominous: true,
            }),
            27699u32 => Ok(GeneratedStruct82 {
                trial_spawner_state: TrialSpawnerState::WaitingForPlayers,
                ominous: true,
            }),
            27700u32 => Ok(GeneratedStruct82 {
                ominous: true,
                trial_spawner_state: TrialSpawnerState::Active,
            }),
            27701u32 => Ok(GeneratedStruct82 {
                trial_spawner_state: TrialSpawnerState::WaitingForRewardEjection,
                ominous: true,
            }),
            27702u32 => Ok(GeneratedStruct82 {
                ominous: true,
                trial_spawner_state: TrialSpawnerState::EjectingReward,
            }),
            27703u32 => Ok(GeneratedStruct82 {
                trial_spawner_state: TrialSpawnerState::Cooldown,
                ominous: true,
            }),
            27704u32 => Ok(GeneratedStruct82 {
                ominous: false,
                trial_spawner_state: TrialSpawnerState::Inactive,
            }),
            27705u32 => Ok(GeneratedStruct82 {
                ominous: false,
                trial_spawner_state: TrialSpawnerState::WaitingForPlayers,
            }),
            27706u32 => Ok(GeneratedStruct82 {
                ominous: false,
                trial_spawner_state: TrialSpawnerState::Active,
            }),
            27707u32 => Ok(GeneratedStruct82 {
                ominous: false,
                trial_spawner_state: TrialSpawnerState::WaitingForRewardEjection,
            }),
            27708u32 => Ok(GeneratedStruct82 {
                trial_spawner_state: TrialSpawnerState::EjectingReward,
                ominous: false,
            }),
            27709u32 => Ok(GeneratedStruct82 {
                trial_spawner_state: TrialSpawnerState::Cooldown,
                ominous: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct82 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct82 {
                trial_spawner_state: TrialSpawnerState::Inactive,
                ominous: true,
            } => Ok(27698u32),
            GeneratedStruct82 {
                trial_spawner_state: TrialSpawnerState::WaitingForPlayers,
                ominous: true,
            } => Ok(27699u32),
            GeneratedStruct82 {
                ominous: true,
                trial_spawner_state: TrialSpawnerState::Active,
            } => Ok(27700u32),
            GeneratedStruct82 {
                trial_spawner_state: TrialSpawnerState::WaitingForRewardEjection,
                ominous: true,
            } => Ok(27701u32),
            GeneratedStruct82 {
                ominous: true,
                trial_spawner_state: TrialSpawnerState::EjectingReward,
            } => Ok(27702u32),
            GeneratedStruct82 {
                trial_spawner_state: TrialSpawnerState::Cooldown,
                ominous: true,
            } => Ok(27703u32),
            GeneratedStruct82 {
                ominous: false,
                trial_spawner_state: TrialSpawnerState::Inactive,
            } => Ok(27704u32),
            GeneratedStruct82 {
                ominous: false,
                trial_spawner_state: TrialSpawnerState::WaitingForPlayers,
            } => Ok(27705u32),
            GeneratedStruct82 {
                ominous: false,
                trial_spawner_state: TrialSpawnerState::Active,
            } => Ok(27706u32),
            GeneratedStruct82 {
                ominous: false,
                trial_spawner_state: TrialSpawnerState::WaitingForRewardEjection,
            } => Ok(27707u32),
            GeneratedStruct82 {
                trial_spawner_state: TrialSpawnerState::EjectingReward,
                ominous: false,
            } => Ok(27708u32),
            GeneratedStruct82 {
                trial_spawner_state: TrialSpawnerState::Cooldown,
                ominous: false,
            } => Ok(27709u32),
            _ => Err(()),
        }
    }
}

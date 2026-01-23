#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TrialSpawnerBlock {
    pub ominous: bool,
    pub trial_spawner_state: TrialSpawnerState,
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

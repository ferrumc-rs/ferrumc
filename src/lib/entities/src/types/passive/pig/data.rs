use crate::game_entity::GameEntity;
use bevy_ecs::prelude::{Commands, Component};
use ferrumc_state::GlobalState;
use serde::{Deserialize, Serialize};
use typename::TypeName;

/// Pig-specific data component
#[derive(Debug, Default, Clone, Component, Serialize, Deserialize, TypeName)]
pub struct PigData {
    pub saddled: bool,
    pub boost_time: i32,
}

impl GameEntity for PigData {
    fn tick(&mut self, _state: &GlobalState, _commands: &mut Commands) {
        // Decrease boost time if active
        if self.boost_time > 0 {
            self.boost_time -= 1;
        }
        // TODO: Pig-specific AI (wandering, pathfinding, etc.)
    }
}

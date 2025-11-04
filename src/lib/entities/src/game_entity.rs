use bevy_ecs::prelude::Commands;
use ferrumc_state::GlobalState;
use typename::TypeName;

/// Trait for game entities that have behavior/AI
/// Each entity type can implement tick() to update its state
pub trait GameEntity: Send + Sync + TypeName + 'static {
    fn tick(&mut self, _state: &GlobalState, _commands: &mut Commands) {
        // Default: no-op for stateless entities
    }
}

use ferrumc_macros::Event;
use ferrumc_ecs::entities::Entity;

#[derive(Event, Clone)]
pub struct PlayerQuitEvent {
    /// The entity that this event was fired for.
    pub entity: Entity,
}

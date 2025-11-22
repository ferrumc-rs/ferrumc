pub mod collision;
pub mod components;
pub mod entity_lookup;
pub mod events;
pub mod game_entity;
pub mod spawn_command_queue;
pub mod types;

// Re-export principals items for easyer usage
pub use components::{Age, EntityId, EntityType, Health, Persisted, Velocity};
pub use entity_lookup::EntityNetworkIdIndex;
pub use events::{DamageEvent, SpawnEntityEvent};
pub use game_entity::GameEntity;
pub use spawn_command_queue::{SpawnRequest, pop_spawn_request, request_spawn};

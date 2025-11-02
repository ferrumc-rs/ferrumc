pub mod components;
pub mod events;
pub mod game_entity;
pub mod spawn_command_queue;
pub mod types;

// Re-export principals items for easyer usage
pub use components::{Age, EntityId, EntityType, Health, Persisted, Velocity};
pub use events::SpawnEntityEvent;
pub use game_entity::GameEntity;
pub use spawn_command_queue::{SpawnRequest, drain_spawn_requests, request_spawn};

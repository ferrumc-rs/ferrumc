// Core entity components based on ferrumc-data
pub mod combat;
pub mod last_synced_position;
pub mod metadata;
pub mod physical;
pub mod spawn;

// Re-exports
pub use combat::CombatProperties;
pub use last_synced_position::LastSyncedPosition;
pub use metadata::EntityMetadata;
pub use physical::PhysicalProperties;
pub use spawn::SpawnProperties;

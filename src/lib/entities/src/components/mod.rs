// Core entity components based on ferrumc-data
pub mod combat;
pub mod metadata;
pub mod physical;
pub mod spawn;

// Re-exports
pub use combat::CombatProperties;
pub use metadata::EntityMetadata;
pub use physical::PhysicalProperties;
pub use spawn::SpawnProperties;

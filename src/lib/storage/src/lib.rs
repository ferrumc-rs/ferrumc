pub mod chunk;
pub mod db;
pub mod errors;
pub mod import;

// Re-exports for convenience
// Users can use `ferrumc_storage::ChunkStorage` directly
pub use chunk::api::ChunkStorage;
pub use errors::WorldError;

// --- Bevy Plugin Implementation ---
use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use ferrumc_config::DatabaseConfig;
use ferrumc_core::traits::ChunkSource;
use std::sync::Arc;

/// The Bevy Plugin that initializes the storage layer.
pub struct StoragePlugin {
    config: DatabaseConfig,
}

impl StoragePlugin {
    pub fn new(config: DatabaseConfig) -> Self {
        Self { config }
    }
}

impl Plugin for StoragePlugin {
    fn build(&self, app: &mut App) {
        // 1. Initialize the ChunkStorage (Imperative / Blocking I/O)
        // We do this here so if it fails, the server crashes immediately on startup.
        let storage = ChunkStorage::new(
            &self.config.db_path,
            self.config.cache_ttl,
            self.config.cache_capacity,
        )
        .expect("Failed to initialize World Database");

        let storage_arc = Arc::new(storage);

        // 2. Insert as the Concrete Resource (for systems that need specific storage API)
        // app.insert_resource(storage_arc.clone());

        // 3. Insert as the Abstract Resource (for WorldGen / Generic systems)
        // This allows us to swap out the backend later if needed.
        app.insert_resource(GlobalChunkStorage(storage_arc));
    }
}

// Resource Wrapper for the Trait Object
#[derive(Resource, Clone)]
pub struct GlobalChunkStorage(pub Arc<dyn ChunkSource>);

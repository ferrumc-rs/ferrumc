use bevy_ecs::query::QueryEntityError;
use ferrumc_core::errors::CoreError;
use ferrumc_inventories::errors::InventoryError;
use ferrumc_net::errors::NetError;
use ferrumc_plugins::errors::PluginsError;
use ferrumc_storage::errors::StorageError;
use ferrumc_utils::errors::UtilsError;
use ferrumc_world::errors::WorldError;
use thiserror::Error;

/// Errors that could be thrown by the application.
#[derive(Debug, Error)]
pub enum BinaryError {
    /// An error that gets thrown at the core level.
    #[error("Core error: {0}")]
    Core(#[from] CoreError),

    /// An error that occurs when retrieving a specific Entity's query result from Query or QueryState.
    #[error("QueryError error: {0}")]
    QueryError(#[from] QueryEntityError),

    /// An error that gets thrown for connection / networking errors.
    #[error("Net error: {0}")]
    Net(#[from] NetError),

    /// An error if a plugin threw an error
    #[error("Plugins error: {0}")]
    Plugins(#[from] PluginsError),

    /// An error if there was an IO error via the storage.
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),

    /// An error that gets thrown when utils fail.
    #[error("Utils error: {0}")]
    Utils(#[from] UtilsError),

    /// An error that gets thrown when the chunkloading or world files fails.
    #[error("World error: {0}")]
    World(#[from] WorldError),

    /// An error that gets thrown when an Inventory error happens.
    #[error("Inventory error: {0}")]
    Inventory(#[from] InventoryError),

    /// Custom errors, that are not handled in another error.
    #[error("{0}")]
    Custom(String),

    /// Classic IO Error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Error when the path given, cant be correctly handled.
    #[error("Root Path error: {0}")]
    RootPath(#[from] ferrumc_general_purpose::paths::RootPathError),

    /// An error that gets thrown when world-generation fails.
    #[error("WorldGen error: {0}")]
    WorldGen(#[from] ferrumc_world_gen::errors::WorldGenError),
}

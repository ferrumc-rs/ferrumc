use bevy_ecs::query::QueryEntityError;
use ferrumc_components::inventory::errors::InventoryError;
use ferrumc_net::errors::NetError;
use ferrumc_plugins::errors::PluginsError;
use ferrumc_storage::errors::StorageError;
use ferrumc_utils::errors::UtilsError;
use ferrumc_world::errors::WorldError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BinaryError {
    #[error("QueryError error: {0}")]
    QueryError(#[from] QueryEntityError),

    #[error("Net error: {0}")]
    Net(#[from] NetError),

    #[error("Plugins error: {0}")]
    Plugins(#[from] PluginsError),

    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),

    #[error("Utils error: {0}")]
    Utils(#[from] UtilsError),

    #[error("World error: {0}")]
    World(#[from] WorldError),

    #[error("Inventory error: {0}")]
    Inventory(#[from] InventoryError),

    #[error("{0}")]
    Custom(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Root Path error: {0}")]
    RootPath(#[from] ferrumc_general_purpose::paths::RootPathError),

    #[error("WorldGen error: {0}")]
    WorldGen(#[from] ferrumc_world_gen::errors::WorldGenError),
}

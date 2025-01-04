use ferrumc_ecs::{entities::Entity, errors::ECSError};
use ferrumc_net::errors::NetError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InventoryError {
    #[error("Entity [{0}] already has an open inventory. Cannot open another one.")]
    AlreadyOpenedInventory(Entity),

    #[error("Invalid equipment slot for PlayerInventory")]
    InvalidEquipmentSlot,

    #[error("Invalid slot id in Inventory")]
    InvalidSlot,

    #[error("Trying to sync an inventory that isn't syncable! [ID: {0}]")]
    SyncingANonSyncedInventory(u8),

    #[error("Net error: [{0}].")]
    NetError(#[from] NetError),

    #[error("ECS error: [{0}].")]
    ECSError(#[from] ECSError),

    #[error("Unknown error occurred with inventories...")]
    Unknown,
}

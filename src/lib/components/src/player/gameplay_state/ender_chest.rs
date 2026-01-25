use bevy_ecs::prelude::Component;
use bitcode_derive::{Decode, Encode};
use ferrumc_inventories::inventory::Inventory;
use ferrumc_inventories::StorageInventory;

/// The player's 27-slot personal Ender Chest (ECS component).
#[derive(Component, Clone, Debug)]
pub struct EnderChest(pub Inventory);

impl EnderChest {
    pub const ENDERCHEST_SIZE: usize = 27;
}

impl Default for EnderChest {
    fn default() -> Self {
        Self(Inventory::new(EnderChest::ENDERCHEST_SIZE))
    }
}

/// Storage-friendly ender chest for bitcode persistence.
#[derive(Clone, Debug, Encode, Decode)]
pub struct StorageEnderChest(pub StorageInventory);

impl From<&EnderChest> for StorageEnderChest {
    fn from(ec: &EnderChest) -> Self {
        Self(StorageInventory::from(&ec.0))
    }
}

impl From<StorageEnderChest> for EnderChest {
    fn from(storage: StorageEnderChest) -> Self {
        Self(Inventory::from(storage.0))
    }
}

impl Default for StorageEnderChest {
    fn default() -> Self {
        StorageEnderChest::from(&EnderChest::default())
    }
}

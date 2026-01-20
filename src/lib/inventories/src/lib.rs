pub mod builder;
pub mod components;
pub mod defined_slots;
pub mod errors;
pub mod hotbar;
pub mod inventory;
pub mod item;
pub mod slot;
pub mod storage;

pub use builder::ItemBuilder;
pub use inventory::{Inventory, StorageInventory};
pub use storage::{StorageComponent, StorageInventorySlot};

use crate::slot::InventorySlot;
use bevy_ecs::prelude::Entity;
use crossbeam_queue::SegQueue;
use once_cell::sync::Lazy;

pub static INVENTORY_UPDATES_QUEUE: Lazy<SegQueue<InventoryUpdate>> = Lazy::new(SegQueue::new);

pub struct InventoryUpdate {
    pub slot_index: u8,
    pub slot: InventorySlot,
    pub entity: Entity,
}

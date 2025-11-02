pub mod defined_slots;
pub mod errors;
pub mod hotbar;
pub mod inventory;
pub mod item;
pub mod slot;

use crate::slot::InventorySlot;
use bevy_ecs::prelude::Entity;
use crossbeam_queue::SegQueue;
use once_cell::sync::Lazy;

/// Queue to update the inventory.
pub static INVENTORY_UPDATES_QUEUE: Lazy<SegQueue<InventoryUpdate>> = Lazy::new(SegQueue::new);

/// Update for the client to show what changed in the inv.
pub struct InventoryUpdate {
    /// Which slot is affected.
    pub slot_index: u8,
    /// The inventory slot struct to set items and more.
    pub slot: InventorySlot,
    /// Whos inventory is affected.
    pub entity: Entity,
}

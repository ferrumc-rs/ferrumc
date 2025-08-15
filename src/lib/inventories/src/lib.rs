pub mod item;
pub mod inventory;
mod defined_slots;
pub mod errors;
pub mod display;
pub mod slot;

use bevy_ecs::prelude::Entity;
use once_cell::sync::Lazy;
use crossbeam_queue::SegQueue;
use crate::slot::InventorySlot;

pub static INVENTORY_UPDATES_QUEUE: Lazy<SegQueue<InventoryUpdate>> = Lazy::new(SegQueue::new);

pub struct InventoryUpdate {
    pub slot_index: u8,
    pub slot: InventorySlot,
    pub entity: Entity
}

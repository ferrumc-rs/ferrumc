use bevy_ecs::prelude::Entity;
use crossbeam_queue::SegQueue;
use ferrumc_core::items::inventory_slot::InventorySlot;
use once_cell::sync::Lazy;

// The data packet for an inventory change
pub struct InventoryUpdate {
    pub slot_index: u8,
    pub slot: InventorySlot,
    pub entity: Entity,
}

// The global queue.
// Ideally, we would make this a Bevy Resource later, but keeping it static
// makes the refactor easier on me rn
pub static INVENTORY_UPDATES_QUEUE: Lazy<SegQueue<InventoryUpdate>> = Lazy::new(SegQueue::new);

// #[derive(Resource, Default)]
// pub struct InventoryUpdatesQueue(pub SegQueue<InventoryUpdate>);

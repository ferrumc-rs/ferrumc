//! Inventory-related messages for ECS event communication.

use bevy_ecs::prelude::{Entity, Message};
use ferrumc_inventories::sync::EquipmentSlot;

/// Fired after a player's full inventory has been synced to their client.
/// Useful for plugins that need to know when inventory sync is complete.
#[derive(Message, Clone, Debug)]
pub struct InventorySynced {
    pub player: Entity,
}

/// Fired when a player's visible equipment changes.
/// Includes which slots changed so listeners can act on specific equipment.
#[derive(Message, Clone, Debug)]
pub struct EquipmentChanged {
    pub player: Entity,
    pub slots: Vec<EquipmentSlot>,
}

/// Fired when a player changes their selected hotbar slot.
/// The main_hand item may have changed even if both slots were empty.
#[derive(Message, Clone, Debug)]
pub struct HeldItemChanged {
    pub player: Entity,
    pub old_slot: u8,
    pub new_slot: u8,
}

use bevy_ecs::prelude::Component;
use ferrumc_core::items::inventory_slot::InventorySlot;

#[derive(Component, Debug, Clone)]
pub struct Inventory {
    pub slots: Vec<Option<InventorySlot>>, // or Box<[...]>, whatever you prefer
                                           // Add helper methods here: .add_item(), .remove_item(), .clear()
}

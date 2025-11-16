use bevy_ecs::prelude::Component;
use ferrumc_inventories::inventory::Inventory;

/// The player's 27-slot personal Ender Chest.
#[derive(Component, Debug, Clone)]
pub struct EnderChest(pub Inventory);

impl Default for EnderChest {
    fn default() -> Self {
        Self(Inventory::new(27)) // 27 slots
    }
}

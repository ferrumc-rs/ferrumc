use bevy_ecs::prelude::Component;
use ferrumc_inventories::inventory::Inventory;

// TODO: Re-add bitcode Decode/Encode once Inventory Component serialization is implemented.
// Ender chest persistence is temporarily disabled for network component testing.
/// The player's 27-slot personal Ender Chest.
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

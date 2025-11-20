use crate::inventory::storage::Inventory; // Import from local crate
use bevy_ecs::prelude::Component;
use std::ops::{Deref, DerefMut};

/// The player's personal Ender Chest inventory.
/// Always has 27 slots.
#[derive(Component, Debug, Clone)]
pub struct EnderChest(pub Inventory);

impl Default for EnderChest {
    fn default() -> Self {
        // An Ender Chest always has 27 slots (3 rows x 9 columns)
        Self(Inventory::new(27))
    }
}

// Add Deref so you can use it just like an Inventory
impl Deref for EnderChest {
    type Target = Inventory;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EnderChest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

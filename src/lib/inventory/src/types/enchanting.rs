#![warn(dead_code)]

use ferrumc_macros::{Inventory, inventory};

use crate::inventory::Inventory;

#[derive(Inventory, Debug, Clone)]
#[inventory(inventory_type = EnchantmentTable)]
pub struct EnchantingInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = 0)]
    pub item: i32,
    #[slot(id = 1, default_value = 0)]
    pub secondary: i32,
}

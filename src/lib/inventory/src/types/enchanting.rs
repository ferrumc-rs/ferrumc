#![warn(dead_code)]

use ferrumc_macros::{Inventory, inventory_type};

use crate::inventory::Inventory;

#[derive(Inventory, Debug)]
#[inventory_type(value = EnchantmentTable)]
pub struct EnchantingInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = 0)]
    pub item: i32,
    #[slot(id = 1, default_value = 0)]
    pub secondary: i32,
}

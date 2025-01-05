#![warn(dead_code)]

use crate::inventory::Inventory;
use ferrumc_macros::{Inventory, inventory};

#[derive(Inventory, Debug)]
#[inventory(inventory_type = Loom)]
pub struct LoomInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = 0)]
    pub banner: i32,
    #[slot(id = 1, default_value = 0)]
    pub dye: i32,
    #[slot(id = 2, default_value = 0)]
    pub pattern: i32,
    #[slot(id = 3, default_value = 0)]
    pub result: i32,
}

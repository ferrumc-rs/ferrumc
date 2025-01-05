#![warn(dead_code)]

use crate::inventory::Inventory;
use ferrumc_macros::{Inventory, inventory_type};

#[derive(Inventory, Debug)]
#[inventory_type(value = Stonecutter)]
pub struct StoneCutterInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = 0)]
    pub input: i32,
    #[slot(id = 1, default_value = 0)]
    pub result: i32,
}

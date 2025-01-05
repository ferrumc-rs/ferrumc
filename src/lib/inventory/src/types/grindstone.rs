#![warn(dead_code)]

use crate::inventory::Inventory;
use ferrumc_macros::{Inventory, inventory};

#[derive(Inventory, Debug, Clone)]
#[inventory(inventory_type = Grindstone)]
pub struct GrindstoneInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = 0)]
    pub first: i32,
    #[slot(id = 1, default_value = 0)]
    pub second: i32,
    #[slot(id = 2, default_value = 0)]
    pub result: i32,
}

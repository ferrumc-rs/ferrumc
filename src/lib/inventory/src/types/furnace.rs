#![warn(dead_code)]

use crate::inventory::Inventory;
use ferrumc_macros::{Inventory, inventory};

#[derive(Inventory, Debug, Clone)]
#[inventory(inventory_type = Furnace)]
pub struct FurnaceInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = 0)]
    pub ingredient: i32,
    #[slot(id = 1, default_value = 0)]
    pub fuel: i32,
    #[slot(id = 2, default_value = 0)]
    pub output: i32,
}

#[derive(Inventory, Debug, Clone)]
#[inventory(inventory_type = BlastFurnace)]
pub struct BlastFurnaceInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = 0)]
    pub ingredient: i32,
    #[slot(id = 1, default_value = 0)]
    pub fuel: i32,
    #[slot(id = 2, default_value = 0)]
    pub output: i32,
}

#[derive(Inventory, Debug, Clone)]
#[inventory(inventory_type = Smoker)]
pub struct SmokerInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = 0)]
    pub ingredient: i32,
    #[slot(id = 1, default_value = 0)]
    pub fuel: i32,
    #[slot(id = 2, default_value = 0)]
    pub output: i32,
}

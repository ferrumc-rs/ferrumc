use crate::inventory::Inventory;
use ferrumc_macros::{Inventory, inventory_type};

#[derive(Inventory, Debug)]
#[inventory_type(value = Furnace)]
pub struct FurnaceInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = 0)]
    pub ingredient: i32,
    #[slot(id = 1, default_value = 0)]
    pub fuel: i32,
    #[slot(id = 2, default_value = 0)]
    pub output: i32,
}

#[derive(Inventory, Debug)]
#[inventory_type(value = BlastFurnace)]
pub struct BlastFurnaceInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = 0)]
    pub ingredient: i32,
    #[slot(id = 1, default_value = 0)]
    pub fuel: i32,
    #[slot(id = 2, default_value = 0)]
    pub output: i32,
}

#[derive(Inventory, Debug)]
#[inventory_type(value = Smoker)]
pub struct SmokerInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = 0)]
    pub ingredient: i32,
    #[slot(id = 1, default_value = 0)]
    pub fuel: i32,
    #[slot(id = 2, default_value = 0)]
    pub output: i32,
}

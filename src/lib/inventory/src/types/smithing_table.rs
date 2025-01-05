use crate::inventory::Inventory;
use ferrumc_macros::{Inventory, inventory_type};

#[derive(Inventory, Debug)]
#[inventory_type(value = SmithingTable)]
pub struct SmithingTableInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = 0)]
    pub template: i32,
    #[slot(id = 1, default_value = 0)]
    pub base: i32,
    #[slot(id = 2, default_value = 0)]
    pub additional: i32,
    #[slot(id = 3, default_value = 0)]
    pub result: i32,
}

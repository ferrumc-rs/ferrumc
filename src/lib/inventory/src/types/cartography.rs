use crate::inventory::Inventory;
use ferrumc_macros::{Inventory, inventory_type};

#[derive(Inventory, Debug)]
#[inventory_type(value = Cartography)]
pub struct EnchantingInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = 0)]
    pub map: i32,
    #[slot(id = 1, default_value = 0)]
    pub paper: i32,
    #[slot(id = 2, default_value = 0)]
    pub output: i32,
}

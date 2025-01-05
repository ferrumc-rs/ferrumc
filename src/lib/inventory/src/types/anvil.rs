use crate::inventory::Inventory;
use ferrumc_macros::{Inventory, inventory_type};

#[derive(Inventory, Debug)]
#[inventory_type(value = Anvil)]
pub struct EnchantingInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = 0)]
    pub first: i32,
    #[slot(id = 1, default_value = 0)]
    pub second: i32,
    #[slot(id = 2, default_value = 0)]
    pub result: i32,
}

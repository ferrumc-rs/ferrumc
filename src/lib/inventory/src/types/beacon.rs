use ferrumc_macros::{Inventory, inventory_type};

use crate::inventory::Inventory;

#[derive(Inventory)]
#[inventory_type(value = Beacon)]
pub struct BeaconInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = 0)]
    pub powered_item: i32,
}

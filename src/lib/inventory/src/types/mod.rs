use ferrumc_macros::{Inventory, inventory};

use crate::{inventory::Inventory, slot::Slot};

pub mod player;

#[derive(Inventory, Debug, Clone)]
#[inventory(inventory_type = Anvil)]
pub struct AnvilInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = Slot::empty())]
    pub first: Slot,
    #[slot(id = 1, default_value = Slot::empty())]
    pub second: Slot,
    #[slot(id = 2, default_value = Slot::empty())]
    pub result: Slot,
}

#[derive(Inventory, Debug, Clone)]
#[inventory(inventory_type = Beacon)]
pub struct BeaconInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = Slot::empty())]
    pub powered_item: Slot,
}

#[derive(Inventory, Debug, Clone)]
#[inventory(inventory_type = Cartography)]
pub struct EnchantingInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = Slot::empty())]
    pub map: Slot,
    #[slot(id = 1, default_value = Slot::empty())]
    pub paper: Slot,
    #[slot(id = 2, default_value = Slot::empty())]
    pub output: Slot,
}

#[derive(Inventory, Debug, Clone)]
#[inventory(inventory_type = Furnace)]
pub struct FurnaceInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = Slot::empty())]
    pub ingredient: Slot,
    #[slot(id = 1, default_value = Slot::empty())]
    pub fuel: Slot,
    #[slot(id = 2, default_value = Slot::empty())]
    pub output: Slot,
}

#[derive(Inventory, Debug, Clone)]
#[inventory(inventory_type = BlastFurnace)]
pub struct BlastFurnaceInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = Slot::empty())]
    pub ingredient: Slot,
    #[slot(id = 1, default_value = Slot::empty())]
    pub fuel: Slot,
    #[slot(id = 2, default_value = Slot::empty())]
    pub output: Slot,
}

#[derive(Inventory, Debug, Clone)]
#[inventory(inventory_type = Smoker)]
pub struct SmokerInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = Slot::empty())]
    pub ingredient: Slot,
    #[slot(id = 1, default_value = Slot::empty())]
    pub fuel: Slot,
    #[slot(id = 2, default_value = Slot::empty())]
    pub output: Slot,
}

#[derive(Inventory, Debug, Clone)]
#[inventory(inventory_type = Grindstone)]
pub struct GrindstoneInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = Slot::empty())]
    pub first: Slot,
    #[slot(id = 1, default_value = Slot::empty())]
    pub second: Slot,
    #[slot(id = 2, default_value = Slot::empty())]
    pub result: Slot,
}

#[derive(Inventory, Debug)]
#[inventory(inventory_type = Loom)]
pub struct LoomInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = Slot::empty())]
    pub banner: Slot,
    #[slot(id = 1, default_value = Slot::empty())]
    pub dye: Slot,
    #[slot(id = 2, default_value = Slot::empty())]
    pub pattern: Slot,
    #[slot(id = 3, default_value = Slot::empty())]
    pub result: Slot,
}

#[derive(Inventory, Debug, Clone)]
#[inventory(inventory_type = SmithingTable)]
pub struct SmithingTableInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = Slot::empty())]
    pub template: Slot,
    #[slot(id = 1, default_value = Slot::empty())]
    pub base: Slot,
    #[slot(id = 2, default_value = Slot::empty())]
    pub additional: Slot,
    #[slot(id = 3, default_value = Slot::empty())]
    pub result: Slot,
}

#[derive(Inventory, Debug, Clone)]
#[inventory(inventory_type = Stonecutter)]
pub struct StoneCutterInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = Slot::empty())]
    pub input: Slot,
    #[slot(id = 1, default_value = Slot::empty())]
    pub result: Slot,
}

#[derive(Inventory, Debug, Clone)]
#[inventory(inventory_type = BrewingStand)]
pub struct BrewingStandInventory {
    inventory: Inventory,
    #[slot(id = 3, default_value = Slot::empty())]
    pub potion_ingredient: Slot,
    #[slot(id = 4, default_value = Slot::empty())]
    pub blaze_powder: Slot,
}

impl BrewingStandInventory {
    pub fn set_potion_slot<S: Into<Slot> + Copy>(&mut self, index: i16, slot: S) {
        if (0..=2).contains(&index) {
            self.set_slot(index, slot);
        }
    }
}

#[derive(Inventory, Debug, Clone)]
#[inventory(inventory_type = CraftingTable)]
pub struct CraftingTableInventory {
    inventory: Inventory,
    #[slot(id = 0, default_value = Slot::empty())]
    pub output: Slot,
}

impl CraftingTableInventory {
    pub fn set_crafting_input<S: Into<Slot> + Copy>(&mut self, index: i16, slot: S) {
        if (0..=8).contains(&index) {
            self.set_slot(1 + index, slot);
        }
    }
}

#[derive(Inventory, Debug, Clone)]
#[inventory(inventory_type = Hopper)]
pub struct HopperInventory {
    inventory: Inventory,
}

#[derive(Inventory, Debug, Clone)]
#[inventory(inventory_type = ShulkerBox)]
pub struct ShulkerBoxInventory {
    inventory: Inventory,
}

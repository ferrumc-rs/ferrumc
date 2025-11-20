/// Constants defining the slot indices for the Player Inventory window (Window ID 0).
///
/// Layout:
/// - 0: Crafting Output
/// - 1-4: Crafting Input
/// - 5-8: Armor
/// - 9-35: Main Inventory (3 rows)
/// - 36-44: Hotbar
/// - 45: Offhand
pub mod player_inventory_slots {
    pub const CRAFT_OUTPUT: usize = 0;

    pub const CRAFT_INPUT_START: usize = 1;
    pub const CRAFT_INPUT_END: usize = 4;

    pub const HELMET: usize = 5;
    pub const CHESTPLATE: usize = 6;
    pub const LEGGINGS: usize = 7;
    pub const BOOTS: usize = 8;

    pub const MAIN_INVENTORY_START: usize = 9;
    pub const MAIN_INVENTORY_END: usize = 35;

    pub const HOTBAR_START: usize = 36;
    pub const HOTBAR_END: usize = 44;

    pub const OFFHAND: usize = 45;
}

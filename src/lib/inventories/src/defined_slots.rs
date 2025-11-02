/// Player inventory slot indices used in networking or internal inventory logic.
///
/// These constants correspond to the **standard Minecraft inventory slot layout**:
/// - Crafting grid (slots 0–4)
/// - Armor slots (slots 5–8)
/// - Hotbar (slots 36–44)
///
/// They are useful when interacting with packets, containers, or
/// implementing custom inventory logic.
pub mod player {
    /// Output slot of the player's 2×2 crafting grid.
    pub const CRAFT_SLOT_OUTPUT: u8 = 0;

    /// First input slot of the 2×2 crafting grid (top-left).
    pub const CRAFT_SLOT_1: u8 = 1;

    /// Second input slot of the 2×2 crafting grid (top-right).
    pub const CRAFT_SLOT_2: u8 = 2;

    /// Third input slot of the 2×2 crafting grid (bottom-left).
    pub const CRAFT_SLOT_3: u8 = 3;

    /// Fourth input slot of the 2×2 crafting grid (bottom-right).
    pub const CRAFT_SLOT_4: u8 = 4;

    /// Slot for the player’s **helmet** or head armor piece.
    pub const HEAD_SLOT: u8 = 5;

    /// Slot for the player’s **chestplate** or torso armor piece.
    pub const CHEST_SLOT: u8 = 6;

    /// Slot for the player’s **leggings** or leg armor piece.
    pub const LEGS_SLOT: u8 = 7;

    /// Slot for the player’s **boots** or feet armor piece.
    pub const FEET_SLOT: u8 = 8;

    /// First slot in the player's **hotbar** (far-left slot).
    pub const HOTBAR_SLOT_1: u8 = 36;

    /// Second slot in the player's **hotbar**.
    pub const HOTBAR_SLOT_2: u8 = 37;

    /// Third slot in the player's **hotbar**.
    pub const HOTBAR_SLOT_3: u8 = 38;

    /// Fourth slot in the player's **hotbar**.
    pub const HOTBAR_SLOT_4: u8 = 39;

    /// Fifth slot in the player's **hotbar** (center).
    pub const HOTBAR_SLOT_5: u8 = 40;

    /// Sixth slot in the player's **hotbar**.
    pub const HOTBAR_SLOT_6: u8 = 41;

    /// Seventh slot in the player's **hotbar**.
    pub const HOTBAR_SLOT_7: u8 = 42;

    /// Eighth slot in the player's **hotbar**.
    pub const HOTBAR_SLOT_8: u8 = 43;

    /// Ninth slot in the player's **hotbar** (far-right slot).
    pub const HOTBAR_SLOT_9: u8 = 44;
}

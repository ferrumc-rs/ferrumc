use crate::items::inventory_slot::InventorySlot;
use crate::player::gamemode::GameMode;
use crate::player::health::HealthData;
use crate::player::hunger::HungerData;
use crate::transform::position::PositionData;
use crate::transform::rotation::RotationData;

use serde::{Deserialize, Serialize};

/// The complete persisted state of a player.
/// This is what gets written to disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedPlayerData {
    // --- Location ---
    pub position: PositionData,
    pub rotation: RotationData,
    pub dimension_id: String, // "minecraft:overworld"

    // --- State ---
    pub gamemode: GameMode,
    pub health: HealthData,
    pub hunger: HungerData,
    // pub experience: ExperienceData,

    // --- Inventory ---
    // We store inventory as a flat vector of slots.
    // Indices correspond to the protocol slot IDs.
    pub inventory: Vec<Option<InventorySlot>>,
    pub selected_hotbar_slot: u8,

    pub ender_chest: Vec<Option<InventorySlot>>,

    // --- Logic ---
    pub flying: bool,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockData {
    /// The full name (e.g., "minecraft:grass_block")
    pub name: &'static str,

    /// The Protocol ID for the default state of this block.
    pub default_state_id: u32,

    // --- Physics & Gameplay ---
    pub hardness: f32,
    pub blast_resistance: f32,
    pub friction: f32,
    pub speed_factor: f32,
    pub jump_factor: f32,

    // --- Properties ---
    pub light_emission: u8,
    pub is_air: bool,
    pub is_solid: bool,
}

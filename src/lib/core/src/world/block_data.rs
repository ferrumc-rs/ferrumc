use serde::{Deserialize, Serialize};
<<<<<<< HEAD

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
=======
use std::collections::BTreeMap;

/// Represents the data for a specific block state ID.
/// e.g. { "name": "minecraft:grass_block", "properties": { "snowy": "false" } }
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Default)]
pub struct BlockData {
    pub name: String,
    #[serde(default)]
    pub properties: BTreeMap<String, String>,
    #[serde(default)]
    pub default: bool,
>>>>>>> origin/master
}

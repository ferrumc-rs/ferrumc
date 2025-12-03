use crate::world::block_state_id::BlockStateId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// We feature-gate these derives so they only apply if the features are enabled
// (which they are in your workspace default)

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "deepsize", derive(deepsize::DeepSizeOf))]
pub struct Chunk {
    pub x: i32,
    pub z: i32,
    pub dimension: String,
    pub sections: Vec<Section>,
    pub heightmaps: Heightmaps,
}

impl Chunk {
    pub fn new(x: i32, z: i32, dimension: String) -> Self {
        Self {
            x,
            z,
            dimension,
            sections: Vec::new(),
            heightmaps: Heightmaps::default(),
        }
    }
}

// --- Section Data ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "deepsize", derive(deepsize::DeepSizeOf))]
pub struct Section {
    pub y: i8,
    pub block_states: BlockStates,
    pub biome_states: BiomeStates,
    /// Block light array (2048 bytes)
    pub block_light: Vec<u8>,
    /// Sky light array (2048 bytes)
    pub sky_light: Vec<u8>,
}

// --- Block Storage ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "deepsize", derive(deepsize::DeepSizeOf))]
pub struct BlockStates {
    pub non_air_blocks: u16,
    /// We use `i32` here because it is the native backing type of VarInt.
    /// The Network layer will cast this to VarInt during packet construction.
    pub block_data: PaletteType<i32>,
    /// Optimization cache: counts of each block type in this section.
    pub block_counts: HashMap<BlockStateId, i32>,
}

// --- Biome Storage ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "deepsize", derive(deepsize::DeepSizeOf))]
pub struct BiomeStates {
    pub bits_per_biome: u8,
    pub data: Vec<i64>,
    pub palette: Vec<String>, // Biomes use String Registry Names (e.g. "minecraft:plains")
}

// --- Palette ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "deepsize", derive(deepsize::DeepSizeOf))]
pub enum PaletteType<T> {
    Single(T),
    Indirect {
        bits_per_block: u8,
        data: Vec<i64>, // Minecraft stores packed data as Longs (i64)
        palette: Vec<T>,
    },
    Direct,
}

// --- Heightmaps ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "deepsize", derive(deepsize::DeepSizeOf))]
pub struct Heightmaps {
    pub motion_blocking: Vec<i64>,
    pub world_surface: Vec<i64>,
}

use bitcode::{Decode, Encode};
use ferrumc_macros::{NBTDeserialize, NBTSerialize};
use macro_rules_attribute::{apply, attribute_alias};
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Display;

attribute_alias! {
    #[apply(ChunkDerives)] = #[derive(
    NBTSerialize,
    NBTDeserialize,
    Debug,
    Clone,
    PartialEq,
    Encode,
    Serialize,
    Decode,
    Deserialize,
    Eq,
)];
}

#[apply(ChunkDerives)]
#[nbt(is_root)]
#[nbt(rename = "")]
pub(crate) struct VanillaChunk {
    pub dimension: Option<String>,
    #[nbt(rename = "Status")]
    pub status: String,
    #[nbt(rename = "DataVersion")]
    pub data_version: i32,
    #[nbt(rename = "Heightmaps")]
    pub heightmaps: Option<VanillaHeightmaps>,
    #[nbt(rename = "isLightOn")]
    pub is_light_on: Option<i8>,
    #[nbt(rename = "InhabitedTime")]
    pub inhabited_time: Option<i64>,
    #[nbt(rename = "yPos")]
    pub y_pos: i32,
    #[nbt(rename = "xPos")]
    pub x_pos: i32,
    #[nbt(rename = "zPos")]
    pub z_pos: i32,
    pub(crate) structures: Option<Structures>,
    #[nbt(rename = "LastUpdate")]
    pub last_update: Option<i64>,
    pub sections: Option<Vec<Section>>,
}

#[apply(ChunkDerives)]
#[nbt(net_encode)]
pub(crate) struct VanillaHeightmaps {
    // #[nbt(rename = "MOTION_BLOCKING_NO_LEAVES")]
    // pub motion_blocking_no_leaves: Option<Vec<i64>>,
    #[nbt(rename = "MOTION_BLOCKING")]
    pub motion_blocking: Option<Vec<i64>>,
    // #[nbt(rename = "OCEAN_FLOOR")]
    // pub ocean_floor: Option<Vec<i64>>,
    #[nbt(rename = "WORLD_SURFACE")]
    pub world_surface: Option<Vec<i64>>,
}

#[apply(ChunkDerives)]
pub(crate) struct Structures {
    pub starts: Starts,
    #[nbt(rename = "References")]
    pub references: References,
}

#[apply(ChunkDerives)]
pub(crate) struct Starts {}

#[apply(ChunkDerives)]
pub(crate) struct References {}

#[apply(ChunkDerives)]
pub(crate) struct Section {
    #[nbt(rename = "block_states")]
    pub block_states: Option<BlockStates>,
    pub biomes: Option<Biomes>,
    #[nbt(rename = "Y")]
    pub y: i8,
    #[nbt(rename = "BlockLight")]
    pub block_light: Option<Vec<i8>>,
    #[nbt(rename = "SkyLight")]
    pub sky_light: Option<Vec<i8>>,
}

#[apply(ChunkDerives)]
pub(crate) struct BlockStates {
    pub data: Option<Vec<i64>>,
    pub palette: Option<Vec<BlockData>>,
}

/// Information about a block's name and properties.
///
/// This should be used sparingly, as it's much more efficient to use [BlockStateId] where possible.
///
/// If you want to use it as a literal and the convert to a BlockStateId, use the [ferrumc_macros::block_data!] macro.
#[apply(ChunkDerives)]
#[derive(Hash)]
pub struct BlockData {
    #[nbt(rename = "Name")]
    pub name: String,
    #[nbt(rename = "Properties")]
    pub properties: Option<BTreeMap<String, String>>,
}

impl Display for BlockData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for BlockData {
    fn default() -> Self {
        BlockData {
            name: String::from("minecraft:air"),
            properties: None,
        }
    }
}

#[apply(ChunkDerives)]
pub(crate) struct Biomes {
    pub data: Option<Vec<i64>>,
    pub palette: Vec<String>,
}

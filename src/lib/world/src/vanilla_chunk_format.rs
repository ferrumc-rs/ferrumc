use bitcode::{Decode, Encode};
use ferrumc_macros::NBTDeserialize;
use ferrumc_macros::NBTSerialize;
use macro_rules_attribute::{apply, attribute_alias};
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Display;

attribute_alias! {
    #[apply(ChunkDerives)] = #[derive(NBTSerialize, NBTDeserialize,
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
#[derive(deepsize::DeepSizeOf)]
#[nbt(is_root)]
#[nbt(rename = "")]
pub(crate) struct VanillaChunk {
    pub dimension: Option<String>,
    #[nbt(rename = "Status")]
    pub status: String,
    #[nbt(rename = "DataVersion")]
    pub data_version: i32,
    #[nbt(rename = "Heightmaps")]
    pub heightmaps: Option<Heightmaps>,
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
#[derive(deepsize::DeepSizeOf)]
#[nbt(net_encode)]
pub(crate) struct Heightmaps {
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
#[derive(deepsize::DeepSizeOf)]
pub(crate) struct Structures {
    pub starts: Starts,
    #[nbt(rename = "References")]
    pub references: References,
}

#[apply(ChunkDerives)]
#[derive(deepsize::DeepSizeOf)]
pub(crate) struct Starts {}

#[apply(ChunkDerives)]
#[derive(deepsize::DeepSizeOf)]
pub(crate) struct References {}

#[apply(ChunkDerives)]
#[derive(deepsize::DeepSizeOf)]
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
#[derive(deepsize::DeepSizeOf)]
pub(crate) struct BlockStates {
    pub data: Option<Vec<i64>>,
    pub palette: Option<Vec<Palette>>,
}

#[apply(ChunkDerives)]
#[derive(deepsize::DeepSizeOf, Hash)]
pub struct Palette {
    #[nbt(rename = "Name")]
    pub name: String,
    #[nbt(rename = "Properties")]
    pub properties: Option<BTreeMap<String, String>>,
}

impl Display for Palette {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[apply(ChunkDerives)]
#[derive(deepsize::DeepSizeOf)]
pub(crate) struct Properties {
    pub snowy: Option<String>,
    pub level: Option<String>,
    pub east: Option<String>,
    pub waterlogged: Option<String>,
    pub north: Option<String>,
    pub west: Option<String>,
    pub up: Option<String>,
    pub down: Option<String>,
    pub south: Option<String>,
    pub drag: Option<String>,
    pub lit: Option<String>,
    pub axis: Option<String>,
}

#[apply(ChunkDerives)]
#[derive(deepsize::DeepSizeOf)]
pub(crate) struct Biomes {
    pub data: Option<Vec<i64>>,
    pub palette: Vec<String>,
}

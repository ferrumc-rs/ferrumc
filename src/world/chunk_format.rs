use bincode::{Decode, Encode};
use ferrumc_codec::network_types::varint::VarInt;
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;

attribute_alias! {
    #[apply(ChunkDerives)] = #[derive(nbt_lib::NBTSerialize, nbt_lib::NBTDeserialize,
    Debug,
    Clone,
    PartialEq,
    Encode,
    Serialize,
    Decode,
    Deserialize,
    Eq
)];
}

#[apply(ChunkDerives)]
#[derive(deepsize::DeepSizeOf)]
#[nbt(is_root)]
#[nbt(rename = "")]
pub struct Chunk {
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
    pub structures: Option<Structures>,
    #[nbt(rename = "LastUpdate")]
    pub last_update: Option<i64>,
    pub sections: Option<Vec<Section>>,
}

#[apply(ChunkDerives)]
#[derive(deepsize::DeepSizeOf)]
#[nbt(net_encode)]
pub struct Heightmaps {
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
pub struct Structures {
    pub starts: Starts,
    #[nbt(rename = "References")]
    pub references: References,
}

#[apply(ChunkDerives)]
#[derive(deepsize::DeepSizeOf)]
pub struct Starts {}

#[apply(ChunkDerives)]
#[derive(deepsize::DeepSizeOf)]
pub struct References {}

#[apply(ChunkDerives)]
#[derive(deepsize::DeepSizeOf)]
pub struct Section {
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
pub struct BlockStates {
    // These 2 fields don't exist in the chunks stored on disk but will exist when converted to
    // network format
    pub non_air_blocks: Option<i16>,
    pub bits_per_block: Option<i8>,
    pub data: Option<Vec<i64>>,
    // This is the palette for the chunk when stored on disk
    pub palette: Option<Vec<Palette>>,
    // This is the palette for the chunk when converted to network format
    pub net_palette: Option<Vec<VarInt>>,
}

#[apply(ChunkDerives)]
#[derive(deepsize::DeepSizeOf, Hash)]
pub struct Palette {
    #[nbt(rename = "Name")]
    pub name: String,
    #[nbt(rename = "Properties")]
    pub properties: Option<BTreeMap<String, String>>,
}

#[apply(ChunkDerives)]
#[derive(deepsize::DeepSizeOf)]
pub struct Properties {
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
pub struct Biomes {
    pub palette: Vec<String>,
}

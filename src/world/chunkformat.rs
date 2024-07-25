use serde_derive::Deserialize;
use serde_derive::Serialize;
use fastnbt::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chunk {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "block_ticks")]
    pub block_ticks: Vec<Value>,
    #[serde(rename = "DataVersion")]
    pub data_version: i64,
    #[serde(rename = "fluid_ticks")]
    pub fluid_ticks: Vec<Value>,
    #[serde(rename = "Heightmaps")]
    pub heightmaps: Heightmaps,
    pub is_light_on: i64,
    #[serde(rename = "InhabitedTime")]
    pub inhabited_time: i64,
    pub y_pos: i64,
    pub x_pos: i64,
    #[serde(rename = "block_entities")]
    pub block_entities: Vec<Value>,
    pub structures: Structures,
    #[serde(rename = "LastUpdate")]
    pub last_update: i64,
    pub z_pos: i64,
    pub sections: Vec<Section>,
    #[serde(rename = "PostProcessing")]
    pub post_processing: Vec<Vec<Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Heightmaps {
    #[serde(rename = "MOTION_BLOCKING_NO_LEAVES")]
    pub motion_blocking_no_leaves: fastnbt::LongArray,
    #[serde(rename = "MOTION_BLOCKING")]
    pub motion_blocking: fastnbt::LongArray,
    #[serde(rename = "OCEAN_FLOOR")]
    pub ocean_floor: fastnbt::LongArray,
    #[serde(rename = "WORLD_SURFACE")]
    pub world_surface: fastnbt::LongArray,
}

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct MotionBlockingNoLeaves {
//     #[serde(rename = "__fastnbt_long_array")]
//     pub fastnbt_long_array: fastnbt::LongArray,
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MotionBlocking {
    #[serde(rename = "__fastnbt_long_array")]
    pub fastnbt_long_array: fastnbt::LongArray,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OceanFloor {
    #[serde(rename = "__fastnbt_long_array")]
    pub fastnbt_long_array: fastnbt::LongArray,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldSurface {
    #[serde(rename = "__fastnbt_long_array")]
    pub fastnbt_long_array: fastnbt::LongArray,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Structures {
    pub starts: Starts,
    #[serde(rename = "References")]
    pub references: References,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Starts {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct References {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    #[serde(rename = "block_states")]
    pub block_states: BlockStates,
    pub biomes: Biomes,
    #[serde(rename = "Y")]
    pub y: i64,
    #[serde(rename = "BlockLight")]
    pub block_light: Option<fastnbt::ByteArray>,
    #[serde(rename = "SkyLight")]
    pub sky_light: Option<fastnbt::ByteArray>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockStates {
    pub data: Option<fastnbt::LongArray>,
    pub palette: Vec<Palette>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "__fastnbt_long_array")]
    pub fastnbt_long_array: fastnbt::LongArray,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Palette {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Properties")]
    pub properties: Option<Properties>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Biomes {
    pub palette: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockLight {
    #[serde(rename = "__fastnbt_byte_array")]
    pub fastnbt_byte_array: fastnbt::ByteArray,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkyLight {
    #[serde(rename = "__fastnbt_byte_array")]
    pub fastnbt_byte_array: fastnbt::ByteArray,
}

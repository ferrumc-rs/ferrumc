use fastnbt::Value;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chunk {
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "block_ticks")]
    pub block_ticks: Option<Vec<Value>>,
    #[serde(rename = "DataVersion")]
    pub data_version: i64,
    #[serde(rename = "fluid_ticks")]
    pub fluid_ticks: Option<Vec<Value>>,
    #[serde(rename = "CarvingMasks")]
    pub carving_masks: Option<Value>,
    #[serde(rename = "Heightmaps")]
    pub heightmaps: Option<Heightmaps>,
    #[serde(rename = "isLightOn")]
    pub is_light_on: Option<i64>,
    #[serde(rename = "InhabitedTime")]
    pub inhabited_time: Option<i64>,
    pub y_pos: i64,
    pub x_pos: i64,
    #[serde(rename = "block_entities")]
    pub block_entities: Option<Vec<Value>>,
    pub structures: Option<Structures>,
    #[serde(rename = "LastUpdate")]
    pub last_update: Option<i64>,
    pub z_pos: i64,
    pub sections: Vec<Section>,
    #[serde(rename = "PostProcessing")]
    pub post_processing: Option<Value>,
    #[serde(rename = "Lights")]
    pub lights: Option<Value>,
    #[serde(rename = "Entities")]
    pub entities: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Heightmaps {
    #[serde(rename = "MOTION_BLOCKING_NO_LEAVES")]
    pub motion_blocking_no_leaves: Option<MotionBlockingNoLeaves>,
    #[serde(rename = "MOTION_BLOCKING")]
    pub motion_blocking: Option<MotionBlocking>,
    #[serde(rename = "OCEAN_FLOOR")]
    pub ocean_floor: Option<OceanFloor>,
    #[serde(rename = "WORLD_SURFACE")]
    pub world_surface: Option<WorldSurface>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MotionBlockingNoLeaves {
    #[serde(rename = "__fastnbt_long_array")]
    pub fastnbt_long_array: fastnbt::LongArray,
}

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
    pub block_states: Option<BlockStates>,
    pub biomes: Option<Biomes>,
    #[serde(rename = "Y")]
    pub y: i64,
    #[serde(rename = "BlockLight")]
    pub block_light: Option<BlockLight>,
    #[serde(rename = "SkyLight")]
    pub sky_light: Option<SkyLight>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockStates {
    pub data: Option<fastnbt::LongArray>,
    pub palette: Option<Vec<Palette>>,
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
    #[serde(rename = "__fastnbt_long_array")]
    pub fastnbt_byte_array: fastnbt::LongArray,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkyLight {
    #[serde(rename = "__fastnbt_long_array")]
    pub fastnbt_byte_array: fastnbt::LongArray,
}

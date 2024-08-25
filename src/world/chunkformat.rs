use bincode::{Decode, Encode};
use serde_derive::{Deserialize, Serialize};

#[derive(
    nbt_lib::Serialize,
    nbt_lib::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Encode,
    Serialize,
    Decode,
    Deserialize,
)]
#[nbt(is_root)]
#[nbt(rename = "")]
pub struct Chunk {
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

#[derive(
    nbt_lib::Serialize,
    nbt_lib::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Encode,
    Serialize,
    Decode,
    Deserialize,
)]
pub struct Heightmaps {
    #[nbt(rename = "MOTION_BLOCKING_NO_LEAVES")]
    pub motion_blocking_no_leaves: Option<Vec<i64>>,
    #[nbt(rename = "MOTION_BLOCKING")]
    pub motion_blocking: Option<Vec<i64>>,
    #[nbt(rename = "OCEAN_FLOOR")]
    pub ocean_floor: Option<Vec<i64>>,
    #[nbt(rename = "WORLD_SURFACE")]
    pub world_surface: Option<Vec<i64>>,
}

#[derive(
    nbt_lib::Serialize,
    nbt_lib::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Encode,
    Serialize,
    Decode,
    Deserialize,
)]
pub struct Structures {
    pub starts: Starts,
    #[nbt(rename = "References")]
    pub references: References,
}

#[derive(
    nbt_lib::Serialize,
    nbt_lib::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Encode,
    Serialize,
    Decode,
    Deserialize,
)]
pub struct Starts {}
#[derive(
    nbt_lib::Serialize,
    nbt_lib::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Encode,
    Serialize,
    Decode,
    Deserialize,
)]
pub struct References {}

#[derive(
    nbt_lib::Serialize,
    nbt_lib::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Encode,
    Serialize,
    Decode,
    Deserialize,
)]
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

#[derive(
    nbt_lib::Serialize,
    nbt_lib::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Encode,
    Serialize,
    Decode,
    Deserialize,
)]
pub struct BlockStates {
    pub data: Option<Vec<i64>>,
    pub palette: Option<Vec<Palette>>,
}

#[derive(
    nbt_lib::Serialize,
    nbt_lib::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Encode,
    Serialize,
    Decode,
    Deserialize,
)]
pub struct Palette {
    #[nbt(rename = "Name")]
    pub name: String,
    #[nbt(rename = "Properties")]
    pub properties: Option<Properties>,
}

#[derive(
    nbt_lib::Serialize,
    nbt_lib::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Encode,
    Serialize,
    Decode,
    Deserialize,
    Default
)]
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

#[derive(
    nbt_lib::Serialize,
    nbt_lib::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Encode,
    Serialize,
    Decode,
    Deserialize,
)]
pub struct Biomes {
    pub palette: Vec<String>,
}

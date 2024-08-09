use serde_derive::Deserialize;
use serde_derive::Serialize;

use ferrumc_macros::NBTDecode;

use crate::utils::nbt_impls::{ByteArray, LongArray};

#[derive(NBTDecode, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chunk {
    #[rename = "Status"]
    pub status: Option<String>,
    #[rename = "DataVersion"]
    pub data_version: i32,
    #[rename = "Heightmaps"]
    #[nbtcompound]
    pub heightmaps: Option<Heightmaps>,
    #[rename = "isLightOn"]
    pub is_light_on: Option<i64>,
    #[rename = "InhabitedTime"]
    pub inhabited_time: Option<i64>,
    #[rename = "yPos"]
    pub y_pos: i32,
    #[rename = "xPos"]
    pub x_pos: i32,
    #[rename = "zPos"]
    pub z_pos: i32,
    #[nbtcompound]
    pub structures: Option<Structures>,
    #[rename = "LastUpdate"]
    pub last_update: Option<i64>,
    #[nbtcompound]
    pub sections: Option<Vec<Section>>,
    #[rename = "BlockLight"]
    pub block_light: Option<ByteArray>,
    #[rename = "SkyLight"]
    pub sky_light: Option<ByteArray>,
}

#[derive(NBTDecode, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[nbtcompound]
pub struct Heightmaps {
    #[rename = "MOTION_BLOCKING_NO_LEAVES"]
    pub motion_blocking_no_leaves: Option<LongArray>,
    #[rename = "MOTION_BLOCKING"]
    pub motion_blocking: Option<LongArray>,
    #[rename = "OCEAN_FLOOR"]
    pub ocean_floor: Option<LongArray>,
    #[rename = "WORLD_SURFACE"]
    pub world_surface: Option<LongArray>,
}

#[derive(NBTDecode, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[nbtcompound]
pub struct Structures {
    #[nbtcompound]
    pub starts: Starts,
    #[rename = "References"]
    #[nbtcompound]
    pub references: References,
}

#[derive(NBTDecode, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[nbtcompound]
pub struct Starts {}
#[derive(NBTDecode, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[nbtcompound]
pub struct References {}

#[derive(NBTDecode, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[nbtcompound]
pub struct Section {
    #[rename = "block_states"]
    #[nbtcompound]
    pub block_states: Option<BlockStates>,
    #[nbtcompound]
    pub biomes: Option<Biomes>,
    #[rename = "Y"]
    pub y: i64,
    #[rename = "BlockLight"]
    #[nbtcompound]
    pub block_light: Option<ByteArray>,
    #[rename = "SkyLight"]
    #[nbtcompound]
    pub sky_light: Option<ByteArray>,
}

#[derive(NBTDecode, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[nbtcompound]
pub struct BlockStates {
    pub data: Option<LongArray>,
    pub palette: Option<Vec<Palette>>,
}

#[derive(NBTDecode, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[nbtcompound]
pub struct Palette {
    #[rename = "Name"]
    pub name: String,
    #[rename = "Properties"]
    #[nbtcompound]
    pub properties: Option<Properties>,
}

#[derive(NBTDecode, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[nbtcompound]
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

#[derive(NBTDecode, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[nbtcompound]
pub struct Biomes {
    pub palette: Vec<String>,
}

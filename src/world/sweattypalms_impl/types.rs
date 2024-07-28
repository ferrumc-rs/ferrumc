#![allow(dead_code)]
use std::collections::HashMap;
use simdnbt::owned::NbtCompound;
use simdnbt_derive::{Deserialize, Serialize};


#[derive(Deserialize, Debug)]
pub struct Chunk {
    #[simdnbt(rename = "Heightmaps")]
    pub heightmaps: Heightmaps,
    #[simdnbt(rename = "LastUpdate")]
    pub last_update: i64,
    #[simdnbt(rename = "Status")]
    pub status: String,
    #[simdnbt(flatten)]
    pub block_entities: NbtCompound,
    #[simdnbt(rename = "Structures")]
    pub structures: Option<Structures>,
    #[simdnbt(rename = "xPos")]
    pub x: i32,
    #[simdnbt(rename = "yPos")]
    pub y: i32,
    #[simdnbt(rename = "zPos")]
    pub z: i32,
    pub sections: Vec<ChunkSection>,

    /**/


    /*    #[simdnbt(rename = "CarvingMasks")]
        // pub carving_masks: Option<HashMap<String, Vec<i64>>>,
        pub carving_masks: HashMap<String, String>,*/

}

#[derive(Debug, Clone, Deserialize)]
pub struct ChunkSection {
    #[simdnbt(rename = "Y")]
    pub y: i8,
    pub block_states: PalettedContainer,
    pub biomes: NbtCompound,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PalettedContainer {
    pub palette: Vec<Block>,
    pub data: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BlockEntity {
    #[simdnbt(rename = "x")]
    pub x: i32,
    #[simdnbt(rename = "y")]
    pub y: i32,
    #[simdnbt(rename = "z")]
    pub z: i32,
    #[simdnbt(rename = "id")]
    pub id: String,
    // #[simdnbt(flatten)]
    // pub data: HashMap<String, Box<dyn Any>>,
    // pub data: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Heightmaps {
    #[simdnbt(rename = "MOTION_BLOCKING")]
    pub motion_blocking: Option<String>,
    #[simdnbt(rename = "MOTION_BLOCKING_NO_LEAVES")]
    pub motion_blocking_no_leaves: Option<String>,
    #[simdnbt(rename = "OCEAN_FLOOR")]
    pub ocean_floor: Option<Vec<i64>>,
    #[simdnbt(rename = "OCEAN_FLOOR_WG")]
    pub ocean_floor_wg: Option<String>,
    #[simdnbt(rename = "WORLD_SURFACE")]
    pub world_surface: Option<String>,
    #[simdnbt(rename = "WORLD_SURFACE_WG")]
    pub world_surface_wg: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Structures {
    #[simdnbt(rename = "starts")]
    pub starts: HashMap<String, StructureStart>,
    #[simdnbt(rename = "References")]
    pub references: HashMap<String, Vec<i64>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StructureStart {
    #[simdnbt(rename = "id")]
    pub id: String,
    #[simdnbt(rename = "Children")]
    pub children: Vec<StructurePiece>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StructurePiece {
    #[simdnbt(flatten)]
    pub nbt: HashMap<String, NbtCompound>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Block {
    #[simdnbt(rename = "Name")]
    pub name: String,
    #[simdnbt(rename = "Properties")]
    pub properties: Option<HashMap<String, String>>,
}
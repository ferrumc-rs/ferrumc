#![allow(non_snake_case)]

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Chunk {
    pub DataVersion: i32,
    pub xPos: i32,
    pub zPos: i32,
    pub yPos: i32,
    pub LastUpdate: i64,

    #[serde(flatten)]
    pub data: HashMap<String, fastnbt::Value>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Section {
    pub Y: i8,
    pub BlockStates: Vec<u64>,
    pub Palette: Vec<fastnbt::Value>,
    pub BlockLight: Vec<u8>,
    pub SkyLight: Vec<u8>,
}

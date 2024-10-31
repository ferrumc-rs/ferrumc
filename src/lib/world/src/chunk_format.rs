use std::collections::{BTreeMap, HashMap};
use std::io::Read;
use bitcode_derive::{Decode, Encode};
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};


#[cfg(test)]
const BLOCKSFILE: &[u8] = &[0];

// If this file doesn't exist, you'll have to create it yourself. Download the 1.21.3 server from the
// minecraft launcher, extract the blocks data (info here https://wiki.vg/Data_Generators#Blocks_report)
// , put the blocks.json file in the .etc folder, and run the blocks_parser.py script in the scripts
// folder. This will generate the blockmappings.json file that is compressed with bzip2 and included
// in the binary.
#[cfg(not(test))]
const BLOCKSFILE: &[u8] = include_bytes!("../../../../.etc/blockmappings.bz2");

lazy_static! {
    static ref ID2BLOCK: HashMap<i32, Palette> = {
        let mut bzipreader = bzip2::read::BzDecoder::new(BLOCKSFILE);
        let mut output = String::new();
        bzipreader.read_to_string(&mut output).unwrap();
        let string_keys: HashMap<String, Palette> = serde_json::from_str(&output).unwrap();
        string_keys
            .iter()
            .map(|(k, v)| (k.parse::<i32>().unwrap(), v.clone()))
            .collect()
    };
    static ref BLOCK2ID: HashMap<Palette, i32> =
        ID2BLOCK.iter().map(|(k, v)| (v.clone(), *k)).collect();
}

#[derive(Encode, Decode)]
// This is a placeholder for the actual chunk format
pub struct Chunk {
    pub x: i32,
    pub z: i32,
    pub dimension: String,
    pub sections: Vec<Section>,
}

#[derive(Encode, Decode)]
pub struct Section {
    pub y: i8,
    pub block_data: Vec<i64>,
    pub block_palette: Vec<Palette>,
    pub biome_data: Vec<i64>,
    pub biome_palette: Vec<String>,
}
#[derive(Encode, Decode, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct Palette {
    pub name: String,
    pub properties: BTreeMap<String, String>,
}
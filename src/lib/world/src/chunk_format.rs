use crate::errors::WorldError;
use crate::vanilla_chunk_format;
use crate::vanilla_chunk_format::VanillaChunk;
use bitcode_derive::{Decode, Encode};
use ferrumc_net_codec::net_types::var_int::VarInt;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io::Read;
use vanilla_chunk_format::Palette;

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
    pub block_states: BlockStates,
    pub biome_data: Vec<i64>,
    pub biome_palette: Vec<String>,
}
#[derive(Encode, Decode)]
pub struct BlockStates {
    pub bits_per_block: i8,
    pub non_air_blocks: i16,
    pub data: Vec<i64>,
    pub palette: Vec<VarInt>,
}

fn convert_to_net_palette(vanilla_palettes: Vec<Palette>) -> Result<Vec<VarInt>, WorldError> {
    let mut new_palette = Vec::new();
    for palette in vanilla_palettes {
        if let Some(id) = BLOCK2ID.get(&palette) {
            new_palette.push(VarInt::from(*id));
        } else {
            return Err(WorldError::MissingBlockMapping(palette));
        }
    }
    Ok(new_palette)
}


impl VanillaChunk {
    pub fn to_custom_format(&self) -> Result<Chunk, WorldError> {
        let mut sections = Vec::new();
        for section in self.sections.as_ref().unwrap() {
            let y = section.y;
            let (block_data, palette) = if let Some(block_states) = &section.block_states {
                (block_states.data.clone().unwrap_or_default(), block_states.clone().palette.unwrap_or_default())
            } else {
                (vec![], vec![])
            };
            let (biome_data, biome_palette) = if let Some(biomes) = &section.biomes {
                (biomes.data.clone().unwrap_or_default(), biomes.clone().palette)
            } else {
                (vec![], vec![])
            };
           let non_air_blocks= palette.iter().filter(|id| id.name != "air").count() as i16;
            let block_states = BlockStates {
                bits_per_block: (palette.len() as f32).log2().ceil() as i8,
                non_air_blocks,
                data: block_data,
                palette: convert_to_net_palette(palette)?,
            };
            let section = Section {
                y,
                block_states,
                biome_data,
                biome_palette,
            };
            sections.push(section);
            
        }
        Ok(Chunk {
            x: self.x_pos,
            z: self.z_pos,
            dimension: self.clone().dimension.unwrap_or("overworld".to_string()),
            sections,
        })
    }
}
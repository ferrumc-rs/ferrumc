use crate::chunk_palette::{ChunkPalette, IndirectPalette, PaletteType};
use crate::errors::WorldError;
use crate::vanilla_chunk_format;
use crate::vanilla_chunk_format::VanillaChunk;
use bitcode_derive::{Decode, Encode};
use deepsize::DeepSizeOf;
use ferrumc_core::transform::position::Position;
use ferrumc_macros::{NBTDeserialize, NBTSerialize};
use ferrumc_net_codec::net_types::var_int::VarInt;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io::Read;
use tracing::error;
use vanilla_chunk_format::Palette;

#[cfg(test)]
const BLOCKSFILE: &[u8] = &[0];

// If this file doesn't exist, you'll have to create it yourself. Download the 1.21.1 server from the
// minecraft launcher, extract the blocks data (info here https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Data_Generators#Blocks_report)
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

#[derive(Encode, Decode, Clone, DeepSizeOf)]
// This is a placeholder for the actual chunk format
pub struct Chunk {
    pub x: i32,
    pub z: i32,
    pub dimension: String,
    pub sections: Vec<Section>,
    pub heightmaps: Heightmaps,
}

impl Chunk {

    pub fn get_palette(&self) -> &PaletteType {
        &self.sections[0].palette
    }

}

#[derive(Encode, Decode, NBTDeserialize, NBTSerialize, Clone, DeepSizeOf)]
#[nbt(net_encode)]
pub struct Heightmaps {
    #[nbt(rename = "MOTION_BLOCKING")]
    pub motion_blocking: Vec<i64>,
    #[nbt(rename = "WORLD_SURFACE")]
    pub world_surface: Vec<i64>,
}

#[derive(Encode, Decode, Clone, DeepSizeOf)]
pub struct Section {
    pub y: i8,
    pub palette: PaletteType,
    pub biome_data: Vec<i64>,
    pub biome_palette: Vec<String>,
    pub block_light: Vec<u8>,
    pub sky_light: Vec<u8>,
}

fn convert_to_net_palette(vanilla_palettes: Vec<Palette>) -> Result<Vec<VarInt>, WorldError> {
    let mut new_palette = Vec::new();
    for palette in vanilla_palettes {
        if let Some(id) = BLOCK2ID.get(&palette) {
            new_palette.push(VarInt::from(*id));
        } else {
            new_palette.push(VarInt::from(0));
            error!("Could not find block id for palette entry: {:?}", palette);
        }
    }
    Ok(new_palette)
}

impl Heightmaps {
    pub fn new() -> Self {
        Heightmaps {
            motion_blocking: vec![],
            world_surface: vec![],
        }
    }
}

impl Default for Heightmaps {
    fn default() -> Self {
        Heightmaps::new()
    }
}

impl VanillaChunk {
    pub fn to_custom_format(&self) -> Result<Chunk, WorldError> {
        let mut sections = Vec::new();
        for section in self.sections.as_ref().unwrap() {
            let y = section.y;
            let palette = section
                .block_states
                .as_ref()
                .and_then(|bs| bs.palette.clone())
                .unwrap_or_default();
            let biome_data = section
                .biomes
                .as_ref()
                .and_then(|biome_data| biome_data.data.clone())
                .unwrap_or_default();
            let biome_palette = section
                .biomes
                .as_ref()
                .map_or(vec![], |biome_data| biome_data.palette.clone());
            let non_air_blocks = palette.iter().filter(|id| id.name != "air").count() as u16;
            let palette = PaletteType::Indirect(IndirectPalette::empty()); // TODO: Update this to make it into Section
            let block_light = section
                .block_light
                .clone()
                .unwrap_or(vec![0; 2048])
                .iter()
                .map(|x| *x as u8)
                .collect();
            let sky_light = section
                .sky_light
                .clone()
                .unwrap_or(vec![0; 2048])
                .iter()
                .map(|x| *x as u8)
                .collect();
            let section = Section {
                y,
                palette,
                biome_data,
                biome_palette,
                block_light,
                sky_light,
            };
            sections.push(section);
        }
        let heightmaps = if let Some(heightmaps) = &self.heightmaps {
            let motion_blocking = heightmaps.clone().motion_blocking.unwrap_or(vec![]);
            let world_surface = heightmaps.clone().world_surface.unwrap_or(vec![]);
            Heightmaps {
                motion_blocking,
                world_surface,
            }
        } else {
            Heightmaps {
                motion_blocking: vec![],
                world_surface: vec![],
            }
        };
        Ok(Chunk {
            x: self.x_pos,
            z: self.z_pos,
            dimension: self.clone().dimension.unwrap_or("overworld".to_string()),
            sections,
            heightmaps,
        })
    }
}

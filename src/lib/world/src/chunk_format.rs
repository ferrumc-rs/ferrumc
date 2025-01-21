use crate::errors::WorldError;
use crate::vanilla_chunk_format;
use crate::vanilla_chunk_format::VanillaChunk;
use bitcode_derive::{Decode, Encode};
use deepsize::DeepSizeOf;
use ferrumc_general_purpose::data_packing::i32::read_nbit_i32;
use ferrumc_macros::{NBTDeserialize, NBTSerialize};
use ferrumc_net_codec::net_types::var_int::VarInt;
use lazy_static::lazy_static;
use std::cmp::max;
use std::collections::HashMap;
use std::io::Read;
use tracing::error;
use vanilla_chunk_format::BlockData;

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
    pub static ref ID2BLOCK: HashMap<i32, BlockData> = {
        let mut bzipreader = bzip2::read::BzDecoder::new(BLOCKSFILE);
        let mut output = String::new();
        bzipreader.read_to_string(&mut output).unwrap();
        let string_keys: HashMap<String, BlockData> = serde_json::from_str(&output).unwrap();
        string_keys
            .iter()
            .map(|(k, v)| (k.parse::<i32>().unwrap(), v.clone()))
            .collect()
    };
    pub static ref BLOCK2ID: HashMap<BlockData, i32> =
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
    pub block_states: BlockStates,
    pub biome_data: Vec<i64>,
    pub biome_palette: Vec<String>,
    pub block_light: Vec<u8>,
    pub sky_light: Vec<u8>,
}
#[derive(Encode, Decode, Clone, DeepSizeOf)]
pub struct BlockStates {
    pub bits_per_block: u8,
    pub non_air_blocks: u16,
    pub data: Vec<i64>,
    pub palette: Vec<VarInt>,
    pub block_counts: HashMap<BlockData, i32>,
}

fn convert_to_net_palette(vanilla_palettes: Vec<BlockData>) -> Result<Vec<VarInt>, WorldError> {
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
            let block_data = section
                .block_states
                .as_ref()
                .and_then(|bs| bs.data.clone())
                .unwrap_or_default();
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
            let bits_per_block = max((palette.len() as f32).log2().ceil() as u8, 4);
            let mut block_counts = HashMap::new();
            for chunk in &block_data {
                let mut i = 0;
                while i + bits_per_block < 64 {
                    let palette_index = read_nbit_i32(chunk, bits_per_block as usize, i as u32)?;
                    let block = palette
                        .get(palette_index as usize)
                        .unwrap_or(&BlockData::default())
                        .clone();
                    *block_counts.entry(block).or_insert(0) += 1;
                    i += bits_per_block;
                }
            }
            if block_data.is_empty() {
                let single_block = if let Some(block) = palette.first() {
                    block
                } else {
                    &BlockData::default()
                };
                block_counts.insert(single_block.clone(), 4096);
            }
            let non_air_blocks =
                4096 - *block_counts.get(&BlockData::default()).unwrap_or(&0) as u16;
            let block_states = BlockStates {
                bits_per_block,
                block_counts,
                non_air_blocks,
                data: block_data,
                palette: convert_to_net_palette(palette)?,
            };
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
                block_states,
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

impl BlockStates {
    pub fn resize(&mut self, new_bit_size: usize) -> Result<(), WorldError> {
        let max_int_value = (1 << new_bit_size) - 1;

        if self.data.is_empty() {
            let data_size = (4096 * new_bit_size + 63) / 64;
            self.data = vec![0; data_size];
            self.bits_per_block = new_bit_size as u8;
            return Ok(());
        }

        // Step 1: Read existing packed data into a list of normal integers
        let mut normalised_ints = Vec::with_capacity(4096);
        let mut values_read = 0;

        for &long in &self.data {
            let mut bit_offset = 0;

            while bit_offset + self.bits_per_block as usize <= 64 {
                if values_read >= 4096 {
                    break;
                }

                // Extract value at the current bit offset
                let value = read_nbit_i32(&long, self.bits_per_block as usize, bit_offset as u32)?;
                if value > max_int_value {
                    return Err(WorldError::InvalidBlockStateData(format!(
                        "Value {} exceeds maximum value for {}-bit block state",
                        value, new_bit_size
                    )));
                }
                normalised_ints.push(value);
                values_read += 1;

                bit_offset += self.bits_per_block as usize;
            }

            // Stop reading if we’ve already hit 4096 values
            if values_read >= 4096 {
                break;
            }
        }

        // Check if we read exactly 4096 block states
        if normalised_ints.len() != 4096 {
            return Err(WorldError::InvalidBlockStateData(format!(
                "Expected 4096 block states, but got {}",
                normalised_ints.len()
            )));
        }

        // Step 2: Write the normalised integers into the new packed format
        let mut new_data = Vec::new();
        let mut current_long: i64 = 0;
        let mut bit_position = 0;

        for &value in &normalised_ints {
            current_long |= (value as i64) << bit_position;
            bit_position += new_bit_size;

            if bit_position >= 64 {
                new_data.push(current_long);
                current_long = (value as i64) >> (new_bit_size - (bit_position - 64));
                bit_position -= 64;
            }
        }

        // Push any remaining bits in the final long
        if bit_position > 0 {
            new_data.push(current_long);
        }

        // Verify the size of the new data matches expectations
        let expected_size = (4096 * new_bit_size + 63) / 64;
        if new_data.len() != expected_size {
            return Err(WorldError::InvalidBlockStateData(format!(
                "Expected packed data size of {}, but got {}",
                expected_size,
                new_data.len()
            )));
        }
        // Update the chunk with the new packed data and bit size
        self.data = new_data;
        self.bits_per_block = new_bit_size as u8;

        Ok(())
    }
}

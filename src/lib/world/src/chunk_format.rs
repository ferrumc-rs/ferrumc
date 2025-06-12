use crate::edit_batch::EditBatch;
use crate::errors::WorldError::InvalidBlockStateData;
use crate::block_id::{BlockId, BLOCK2ID};
use crate::vanilla_chunk_format;
use crate::vanilla_chunk_format::VanillaChunk;
use crate::{errors::WorldError, vanilla_chunk_format::VanillaHeightmaps};
use bitcode_derive::{Decode, Encode};
use deepsize::DeepSizeOf;
use ferrumc_general_purpose::data_packing::i32::read_nbit_i32;
use ferrumc_macros::{NBTDeserialize, NBTSerialize};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::cmp::max;
use std::collections::HashMap;
use tracing::error;
use vanilla_chunk_format::BlockData;
// #[cfg(test)]
// const BLOCKSFILE: &[u8] = &[0];

// If this file doesn't exist, you'll have to create it yourself. Download the 1.21.1 server from the
// minecraft launcher, extract the blocks data (info here https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Data_Generators#Blocks_report)
// , put the blocks.json file in the .etc folder, and run the blocks_parser.py script in the scripts
// folder. This will generate the blockmappings.json file that is compressed with bzip2 and included
// in the binary.
// #[cfg(not(test))]

#[derive(Encode, Decode, Clone, DeepSizeOf, Eq, PartialEq, Debug)]
// This is a placeholder for the actual chunk format
pub struct Chunk {
    pub x: i32,
    pub z: i32,
    pub dimension: String,
    pub sections: Vec<Section>,
    pub heightmaps: Heightmaps,
    pub real_heightmap: [[i16; 16]; 16],
}

#[derive(Encode, Decode, NBTDeserialize, NBTSerialize, Clone, DeepSizeOf, Debug)]
#[nbt(net_encode)]
#[derive(Eq, PartialEq)]
pub struct Heightmaps {
    #[nbt(rename = "MOTION_BLOCKING")]
    pub motion_blocking: Vec<i64>,
    #[nbt(rename = "WORLD_SURFACE")]
    pub world_surface: Vec<i64>,
}
#[derive(Encode, Decode, Clone, DeepSizeOf, Eq, PartialEq, Debug)]
pub struct Section {
    pub y: i8,
    pub block_states: BlockStates,
    pub biome_states: BiomeStates,
    pub block_light: Vec<u8>,
    pub sky_light: Vec<u8>,
}
#[derive(Encode, Decode, Clone, DeepSizeOf, Eq, PartialEq, Debug)]
pub struct BlockStates {
    pub non_air_blocks: u16,
    pub block_data: PaletteType,
    pub block_counts: HashMap<BlockId, i32>,
}

#[derive(Encode, Decode, Clone, DeepSizeOf, Eq, PartialEq, Debug)]
pub enum PaletteType {
    Single(VarInt),
    Indirect {
        bits_per_block: u8,
        data: Vec<i64>,
        palette: Vec<VarInt>,
    },
    Direct {
        bits_per_block: u8,
        data: Vec<i64>,
    },
}

#[derive(Encode, Decode, Clone, DeepSizeOf, Eq, PartialEq, Debug)]
pub struct BiomeStates {
    pub bits_per_biome: u8,
    pub data: Vec<i64>,
    pub palette: Vec<VarInt>,
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

impl Chunk {
    pub fn get_min_y(&self) -> i16 {
        let mut min_y = i16::MAX;
        for row in self.real_heightmap {
            for y in row {
                if y < min_y {
                    min_y = y;
                }
            }
        }
        min_y
    }
}

impl Default for Heightmaps {
    fn default() -> Self {
        Heightmaps::new()
    }
}

impl From<VanillaHeightmaps> for Heightmaps {
    fn from(value: VanillaHeightmaps) -> Self {
        Self {
            motion_blocking: value.motion_blocking.unwrap_or_default(),
            world_surface: value.world_surface.unwrap_or_default(),
        }
    }
}

impl VanillaChunk {
    pub fn to_custom_format(&self) -> Result<Chunk, WorldError> {
        let mut sections = Vec::new();
        for section in self.sections.as_ref().unwrap() {
            let y = section.y;
            let raw_block_data = section
                .block_states
                .as_ref()
                .and_then(|bs| bs.data.clone())
                .unwrap_or_default();
            let palette = section
                .block_states
                .as_ref()
                .and_then(|bs| bs.palette.clone())
                .unwrap_or_default();
            let bits_per_block = max((palette.len() as f32).log2().ceil() as u8, 4);
            let mut block_counts = HashMap::new();
            for chunk in &raw_block_data {
                let mut i = 0;
                while i + bits_per_block < 64 {
                    let palette_index = read_nbit_i32(chunk, bits_per_block as usize, i as u32)?;
                    let block = match palette.get(palette_index as usize) {
                        Some(block) => block,
                        None => {
                            error!("Could not find block for palette index: {}", palette_index);
                            &BlockData::default()
                        }
                    };

                    if let Some(count) = block_counts.get_mut(&block.to_block_id()) {
                        *count += 1;
                    } else {
                        block_counts.insert(block.to_block_id(), 0);
                    }

                    i += bits_per_block;
                }
            }
            let block_data = if raw_block_data.is_empty() {
                block_counts.insert(BlockId::default(), 4096);
                PaletteType::Single(VarInt::from(0))
            } else {
                PaletteType::Indirect {
                    bits_per_block,
                    data: raw_block_data,
                    palette: convert_to_net_palette(palette)?,
                }
            };
            // Count the number of blocks that are either air, void air, or cave air
            let mut air_blocks = *block_counts.get(&BlockId::default()).unwrap_or(&0) as u16;
            air_blocks += *block_counts
                .get(
                    &BlockData {
                        name: "minecraft:void_air".to_string(),
                        properties: None,
                    }
                    .to_block_id(),
                )
                .unwrap_or(&0) as u16;
            air_blocks += *block_counts
                .get(
                    &BlockData {
                        name: "minecraft:cave_air".to_string(),
                        properties: None,
                    }
                    .to_block_id(),
                )
                .unwrap_or(&0) as u16;
            let non_air_blocks = 4096 - air_blocks;
            let block_states = BlockStates {
                block_counts,
                non_air_blocks,
                block_data,
            };
            let block_light = section
                .block_light
                .as_ref()
                .unwrap_or(&vec![0; 2048])
                .iter()
                .map(|&x| x as u8)
                .collect();
            let sky_light = section
                .sky_light
                .as_ref()
                .unwrap_or(&vec![0; 2048])
                .iter()
                .map(|&x| x as u8)
                .collect();
            let biome_states = BiomeStates {
                // TODO: Implement biome states properly
                bits_per_biome: 4,
                data: vec![],
                palette: vec![VarInt::from(0); 1],
            };
            let section = Section {
                y,
                block_states,
                biome_states,
                block_light,
                sky_light,
            };
            sections.push(section);
        }

        let dimension = self.dimension.clone().unwrap_or("overworld".to_string());

        let heightmaps: Heightmaps = self.heightmaps.clone().map(Into::into).unwrap_or_default();

        Ok(Chunk {
            x: self.x_pos,
            z: self.z_pos,
            dimension,
            sections,
            heightmaps,
            real_heightmap: [[0; 16]; 16],
        })
    }
}

impl Chunk {
    pub fn new(x: i32, z: i32, dimension: String) -> Self {
        let mut sections: Vec<Section> = (-4..20)
            .map(|y| Section {
                y: y as i8,
                block_states: BlockStates {
                    non_air_blocks: 0,
                    block_data: PaletteType::Single(VarInt::from(0)),
                    block_counts: HashMap::from([(BlockId::default(), 4096)]),
                },
                biome_states: BiomeStates {
                    bits_per_biome: 0,
                    data: vec![],
                    palette: vec![VarInt::from(0)],
                },
                block_light: vec![255; 2048],
                sky_light: vec![255; 2048],
            })
            .collect();
        for section in &mut sections {
            section.optimise().expect("Failed to optimise section");
        }
        Chunk {
            x,
            z,
            dimension,
            sections,
            heightmaps: Heightmaps::new(),
            real_heightmap: [[0; 16]; 16],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_set_block() {
        let mut chunk = Chunk::new(0, 0, "overworld".to_string());
        let block = BlockData {
            name: "minecraft:stone".to_string(),
            properties: None,
        }
        .to_block_id();
        chunk.set_block(0, 0, 0, block).unwrap();
        assert_eq!(chunk.get_block(0, 0, 0).unwrap(), block);
    }

    #[test]
    fn test_chunk_fill() {
        let mut chunk = Chunk::new(0, 0, "overworld".to_string());
        let stone_block = BlockData {
            name: "minecraft:stone".to_string(),
            properties: None,
        };
        chunk.fill(stone_block.clone()).unwrap();
        for section in &chunk.sections {
            for (block, count) in &section.block_states.block_counts {
                assert_eq!(*block, stone_block.to_block_id());
                assert_eq!(count, &4096);
            }
        }
    }

    #[test]
    fn test_section_fill() {
        let mut section = Section {
            y: 0,
            block_states: BlockStates {
                non_air_blocks: 0,
                block_data: PaletteType::Single(VarInt::from(0)),
                block_counts: HashMap::from([(BlockId::default(), 4096)]),
            },
            biome_states: BiomeStates {
                bits_per_biome: 0,
                data: vec![],
                palette: vec![VarInt::from(0)],
            },
            block_light: vec![255; 2048],
            sky_light: vec![255; 2048],
        };
        let stone_block = BlockData {
            name: "minecraft:stone".to_string(),
            properties: None,
        };
        section.fill(stone_block.clone()).unwrap();
        assert_eq!(
            section.block_states.block_data,
            PaletteType::Single(VarInt::from(1))
        );
        assert_eq!(
            section
                .block_states
                .block_counts
                .get(&stone_block.to_block_id())
                .unwrap(),
            &4096
        );
    }

    #[test]
    fn test_false_positive() {
        let mut chunk = Chunk::new(0, 0, "overworld".to_string());
        let block = BlockData {
            name: "minecraft:stone".to_string(),
            properties: None,
        }
        .to_block_id();
        chunk.set_block(0, 0, 0, block).unwrap();
        assert_ne!(chunk.get_block(0, 1, 0).unwrap(), block);
    }

    #[test]
    fn test_doesnt_fail() {
        let mut chunk = Chunk::new(0, 0, "overworld".to_string());
        let block = BlockData {
            name: "minecraft:stone".to_string(),
            properties: None,
        };
        assert!(chunk.set_block(0, 0, 0, block.clone()).is_ok());
        assert!(chunk.set_block(0, 0, 0, block.clone()).is_ok());
        assert!(chunk.get_block(0, 0, 0).is_ok());
    }
}

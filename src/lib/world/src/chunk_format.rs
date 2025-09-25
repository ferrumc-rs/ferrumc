use crate::block_id::{BlockId, BLOCK2ID};
use crate::vanilla_chunk_format;
use crate::vanilla_chunk_format::VanillaChunk;
use crate::{errors::WorldError, vanilla_chunk_format::VanillaHeightmaps};
use bitcode_derive::{Decode, Encode};
use deepsize::DeepSizeOf;
use ferrumc_general_purpose::data_packing::i32::read_nbit_i32;
use ferrumc_general_purpose::palette::{Palette, PaletteType};
use ferrumc_macros::{NBTDeserialize, NBTSerialize};
use ferrumc_net_codec::net_types::var_int::VarInt;
use intmap::IntMap;
use std::cmp::max;
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
    pub block_states: Palette<BlockId>,
    pub biome_states: BiomeStates,
    pub block_light: Vec<u8>,
    pub sky_light: Vec<u8>,
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
            let raw_block_data: Vec<u64> = section
                .block_states
                .as_ref()
                .and_then(|bs| {
                    bs.data
                        .clone()
                        .map(|d| d.iter().map(|&x| x as u64).collect())
                })
                .unwrap_or_default();
            let palette = section
                .block_states
                .as_ref()
                .and_then(|bs| bs.palette.clone())
                .unwrap_or_default();
            let bits_per_block = max((palette.len() as f32).log2().ceil() as u8, 4);
            let mut block_states = Palette::new(4096, BlockId(0));

            let mut blocks: Vec<(u8, u8, u8, BlockId)> = Vec::new();
            for chunk in &raw_block_data {
                // let mut i = 0;
                // while i + bits_per_block < 64 {
                //     let palette_index = read_nbit_i32(chunk, bits_per_block as usize, i as u32)?;
                //     let block = match palette.get(palette_index as usize) {
                //         Some(block) => block,
                //         None => {
                //             error!("Could not find block for palette index: {}", palette_index);
                //             &BlockData::default()
                //         }
                //     };
                //
                //     if let Some(count) = block_counts.get_mut(block.to_block_id()) {
                //         *count += 1;
                //     } else {
                //         block_counts.insert(block.to_block_id(), 0);
                //     }
                //
                //     i += bits_per_block;
                // }
                for i in 0..4096u16 {
                    let palette_index = read_nbit_i32(
                        chunk,
                        bits_per_block as usize,
                        (i * bits_per_block as u16) as u32,
                    )?;
                    let block_id = palette
                        .get(palette_index as usize)
                        .cloned()
                        .unwrap_or(BlockData::from(BlockId(0)));
                    let x = (i & 0xF) as u8;
                    let y = ((i >> 8) & 0xF) as u8;
                    let z = ((i >> 4) & 0xF) as u8;
                    blocks.push((x, y, z, BlockId::from(block_id)));
                }
            }
            for (x, y, z, block) in blocks {
                let index = (y as u16) << 8 | (z as u16) << 4 | (x as u16);
                block_states.set(index as usize, block);
            }
            // Count the number of blocks that are either air, void air, or cave air
            let mut air_blocks = block_states.get_count(&BlockId::from(BlockData {
                name: "minecraft:air".to_string(),
                properties: None,
            }));
            air_blocks += block_states.get_count(&BlockId::from(BlockData {
                name: "minecraft:void_air".to_string(),
                properties: None,
            }));
            air_blocks += block_states.get_count(&BlockId::from(BlockData {
                name: "minecraft:cave_air".to_string(),
                properties: None,
            }));
            let non_air_blocks = 4096 - air_blocks;
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
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_math::{IVec2, IVec3};

    #[test]
    fn test_chunk_set_block() {
        let mut chunk = Chunk::new(IVec2::new(0, 0), "overworld".to_string());
        let block = BlockData {
            name: "minecraft:stone".to_string(),
            properties: None,
        }
        .to_block_id();
        chunk.set_block(IVec3::new(0, 0, 0), block).unwrap();
        assert_eq!(chunk.get_block(IVec3::new(0, 0, 0)), block);
    }

    #[test]
    fn test_chunk_fill() {
        let mut chunk = Chunk::new(IVec2::new(0, 0), "overworld".to_string());
        let stone_block = BlockData {
            name: "minecraft:stone".to_string(),
            properties: None,
        }
        .to_block_id();
        chunk.fill(stone_block.clone());
        for section in &chunk.sections {
            for y in 0..16 {
                for z in 0..16 {
                    for x in 0..16 {
                        assert_eq!(
                            chunk.get_block(IVec3::new(x, section.y as i32 * 16 + y, z)),
                            stone_block
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_section_fill() {
        let mut section = Section::new(0);
        let stone_block = BlockData {
            name: "minecraft:stone".to_string(),
            properties: None,
        }
        .to_block_id();
        section.fill(stone_block.clone());
        assert_eq!(
            section.block_states.palette_type,
            PaletteType::Single(BlockId::from(1))
        );
        assert_eq!(section.block_states.get_count(&stone_block), 4096);
    }

    #[test]
    fn test_false_positive() {
        let mut chunk = Chunk::new(IVec2::new(0, 0), "overworld".to_string());
        let block = BlockData {
            name: "minecraft:stone".to_string(),
            properties: None,
        }
        .to_block_id();
        chunk.set_block(IVec3::new(0, 0, 0), block).unwrap();
        assert_ne!(chunk.get_block(IVec3::new(0, 0, 0)), BlockId::from(0));
    }

    #[test]
    fn test_doesnt_fail() {
        let mut chunk = Chunk::new(IVec2::new(0, 0), "overworld".to_string());
        let block = BlockData {
            name: "minecraft:stone".to_string(),
            properties: None,
        }
        .to_block_id();
        chunk.set_block(IVec3::new(15, 255, 15), block).unwrap();
        assert_eq!(chunk.get_block(IVec3::new(15, 255, 15)), block);
    }
}

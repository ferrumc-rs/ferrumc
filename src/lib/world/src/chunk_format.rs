use crate::block_state_id::BlockStateId;
use crate::errors::WorldError;
use crate::vanilla_chunk_format::{VanillaChunk, VanillaHeightmaps};
use bitcode_derive::{Decode, Encode};
use ferrumc_general_purpose::data_packing::i32::read_nbit_i32;
use ferrumc_macros::{block, NBTDeserialize, NBTSerialize};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::cmp::max;
use std::collections::HashMap;
use tracing::error;

// If this file doesn't exist, you'll have to create it yourself. Download the 1.21.1 server from the
// minecraft launcher, extract the blocks data (info here https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Data_Generators#Blocks_report)
// , put the blocks.json file in the .etc folder, and run the blocks_parser.py script in the scripts
// folder. This will generate the blockmappings.json file that is compressed with bzip2 and included
// in the binary.

#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct Chunk {
    pub x: i32,
    pub z: i32,
    pub dimension: String,
    pub sections: Vec<Section>,
    pub heightmaps: Heightmaps,
}

impl Chunk {
    /// Creates a new chunk
    pub fn new(x: i32, z: i32, dimension: String, sections: Vec<Section>) -> Self {
        Chunk {
            x,
            z,
            dimension,
            sections,
            heightmaps: Heightmaps::new(),
        }
    }
    /// Get deep size in bytes for the chunk
    pub fn deep_size(&self) -> usize {
        size_of::<Self>()
            + size_of_val(&self.dimension)
            + self.dimension.capacity()
            + size_of_val(&self.sections)
            + self.sections.capacity() * size_of::<Section>()
            + size_of_val(&self.heightmaps)
            + self.heightmaps.motion_blocking.capacity() * size_of::<i64>()
            + self.heightmaps.world_surface.capacity() * size_of::<i64>()
    }
}

#[derive(Encode, Decode, NBTDeserialize, NBTSerialize, Clone, Debug, Eq, PartialEq)]
#[nbt(net_encode)]
pub struct Heightmaps {
    #[nbt(rename = "MOTION_BLOCKING")]
    pub motion_blocking: Vec<i64>,
    #[nbt(rename = "WORLD_SURFACE")]
    pub world_surface: Vec<i64>,
}

impl Heightmaps {
    /// Creates an empty Heightmaps
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

#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct Section {
    pub y: i8,
    pub block_states: BlockStates,
    pub biome_states: BiomeStates,
    pub block_light: Vec<u8>,
    pub sky_light: Vec<u8>,
}
impl Section {
    /// Creates an empty Section with heigth y
    pub fn empty(y: i8) -> Self {
        Self {
            y,
            block_states: BlockStates::new(),
            biome_states: BiomeStates {
                bits_per_biome: 0,
                data: vec![],
                palette: vec![0.into()],
            },
            block_light: vec![255; 2048],
            sky_light: vec![255; 2048],
        }
    }
}
#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct BlockStates {
    pub non_air_blocks: u16,
    pub block_data: PaletteType,
    pub block_counts: HashMap<BlockStateId, i32>,
}

impl<'a> FromIterator<&'a BlockStateId> for BlockStates {
    fn from_iter<T: IntoIterator<Item = &'a BlockStateId>>(iter: T) -> Self {
        let mut section = Section::empty(0);
        for (index, &block) in iter.into_iter().enumerate() {
            section.set_block_by_index(index, block).unwrap();
        }
        section.block_states
    }
}
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct BlockStatesIter<'a> {
    block_states: &'a BlockStates,
    index: usize,
}

impl<'a> Iterator for BlockStatesIter<'a> {
    type Item = &'a BlockStateId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 4096 {
            None
        } else {
            let block = self.block_states.get_block_by_index(self.index);
            self.index += 1;
            Some(block)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (4096, Some(4096))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        4096
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        Some(self.block_states.get_block_by_index(4095))
    }
}

impl BlockStates {
    pub fn iter(&self) -> BlockStatesIter<'_> {
        BlockStatesIter {
            block_states: self,
            index: 0,
        }
    }

    fn new() -> Self {
        Self {
            non_air_blocks: 0,
            block_data: PaletteType::Empty,
            block_counts: HashMap::from([(BlockStateId::default(), 4096)]),
        }
    }
    /// Palette filled with single block
    pub fn from_single(block_id: BlockStateId) -> Self {
        if matches!(block_id, block!("air")) {
            Self::new()
        } else {
            let mut out = Self::new();
            out.non_air_blocks = if block_id.is_non_air() { 4096 } else { 0 };
            out.block_counts.clear();
            out.block_counts.insert(block_id, 4096);
            out.block_data = PaletteType::Paleted(Box::new(Paletted::U4 {
                palette: {
                    let mut palette = [BlockStateId::default(); 16];
                    palette[0] = block_id;
                    palette
                },
                last: 1,
                data: Box::new([0; _]),
            }));
            out
        }
    }
}
/// This enum represents the block data of the BlockStates of the Section.
#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub enum PaletteType {
    Empty,
    Paleted(Box<Paletted>),
}
impl PaletteType {
    /// Construct a U4 empty palett
    pub fn empty_u4() -> Self {
        Self::Paleted(Box::new(Paletted::U4 {
            palette: [BlockStateId::default(); _],
            last: 1,
            data: Box::new([0; _]),
        }))
    }
    /// Construct a U8 empty palette
    pub fn empty_u8() -> Self {
        Self::Paleted(Box::new(Paletted::U8 {
            palette: [BlockStateId::default(); _],
            last: 1,
            data: Box::new([0; _]),
        }))
    }
    /// Construct a direct empty palette
    pub fn empty_direct() -> Self {
        Self::Paleted(Box::new(Paletted::Direct {
            data: Box::new([BlockStateId::default(); _]),
        }))
    }
}
/// This enum represents the non-empty paletted blockstates
#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub enum Paletted {
    /// palettes with bits per block &le; 4
    U4 {
        palette: [BlockStateId; 16],
        last: u8,
        data: Box<[u8; 2048]>,
    },
    /// palettes with bits per block &le; 8
    U8 {
        palette: [BlockStateId; 256],
        last: u8,
        data: Box<[u8; 4096]>,
    },
    /// direct blockstates
    Direct { data: Box<[BlockStateId; 4096]> },
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct BiomeStates {
    pub bits_per_biome: u8,
    pub data: Vec<i64>,
    pub palette: Vec<VarInt>,
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
            let mut blocks = Vec::with_capacity(4096);
            for chunk in &raw_block_data {
                let mut i = 0;
                while i + bits_per_block < 64 {
                    let palette_index = read_nbit_i32(chunk, bits_per_block as usize, i as u32)?;
                    let block = match palette.get(palette_index as usize) {
                        Some(block) => block.to_block_state_id(),
                        None => {
                            error!("Could not find block for palette index: {}", palette_index);
                            BlockStateId::default()
                        }
                    };
                    blocks.push(block);
                    i += bits_per_block;
                }
            }
            let block_states = blocks.iter().collect();
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
    use bevy_math::IVec3;
    use ferrumc_macros::block;
    #[test]
    fn test_chunk_set_block() {
        let mut chunk = Chunk::new(0, 0, "overworld".to_string(), vec![Section::empty(0)]);
        let block = block!("stone");
        chunk.set_block(IVec3::ZERO, block).unwrap();
        assert_eq!(chunk.get_block(IVec3::ZERO).unwrap(), &block);
    }

    #[test]
    fn test_chunk_fill() {
        let mut chunk = Chunk::new(0, 0, "overworld".to_string(), vec![Section::empty(0)]);
        chunk.fill(block!("stone"));
        for section in &chunk.sections {
            for (block, count) in &section.block_states.block_counts {
                assert_eq!(block, &block!("stone"));
                assert_eq!(count, &4096);
            }
        }
    }

    #[test]
    fn test_section_fill() {
        let mut section = Section::empty(0);
        section.fill(block!("stone"));
        assert_eq!(
            section.block_states.block_data,
            PaletteType::Paleted(Box::new(Paletted::U4 {
                last: 1,
                palette: {
                    let mut palette = [BlockStateId::default(); 16];
                    palette[0] = block!("stone");
                    palette
                },
                data: Box::new([0; _])
            }))
        );
        assert_eq!(
            section.block_states.block_counts.get(&block!("stone")),
            Some(&4096)
        );
    }

    #[test]
    fn test_false_positive() {
        let mut chunk = Chunk::new(0, 0, "overworld".to_string(), vec![Section::empty(0)]);
        let block = block!("stone");
        chunk.set_block(IVec3::ZERO, block).unwrap();
        if let PaletteType::Paleted(palette) = &chunk.sections[0].block_states.block_data {
            if let Paletted::U4 {
                palette,
                last,
                data,
            } = palette.as_ref()
            {
                dbg!(palette);
                dbg!(last);
            }
        }
        assert_ne!(chunk.get_block((0, 1, 0).into()).unwrap(), &block);
    }

    #[test]
    fn test_doesnt_fail() {
        let mut chunk = Chunk::new(0, 0, "overworld".to_string(), vec![Section::empty(0)]);

        let block = block!("stone");
        assert!(chunk.set_block(IVec3::ZERO, block).is_ok());
        assert!(chunk.set_block(IVec3::ZERO, block).is_ok());
        assert!(chunk.get_block(IVec3::ZERO).is_ok());
    }
}

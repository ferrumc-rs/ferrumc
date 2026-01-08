use std::num::NonZeroU16;
use crate::block_state_id::BlockStateId;
use crate::chunk::light::{LightStorage, SectionLightData};
use crate::chunk::section::biome::{BiomeData, BiomeType};
use crate::chunk::section::direct::DirectSection;
use crate::chunk::section::paletted::PalettedSection;
use crate::chunk::section::uniform::UniformSection;
use crate::pos::SectionBlockPos;
use bitcode_derive::{Decode, Encode};
use deepsize::DeepSizeOf;
use ferrumc_macros::block;
use crate::chunk::palette::BlockPalette;
use crate::errors::WorldError;
use crate::vanilla_chunk_format::Section;

mod biome;
mod direct;
pub mod network;
mod paletted;
mod uniform;

pub const CHUNK_SECTION_LENGTH: usize = 16 * 16 * 16;

pub(crate) const AIR: BlockStateId = block!("air");

#[derive(Clone, DeepSizeOf, Encode, Decode)]
pub(crate) enum ChunkSectionType {
    Uniform(UniformSection),
    Paletted(PalettedSection),
    Direct(DirectSection),
}

impl ChunkSectionType {
    #[inline]
    pub fn get_block(&self, pos: SectionBlockPos) -> BlockStateId {
        match self {
            Self::Uniform(data) => data.get_block(),
            Self::Paletted(data) => data.get_block(pos.pack() as _),
            Self::Direct(data) => data.get_block(pos.pack() as _),
        }
    }

    #[inline]
    pub fn set_block(&mut self, pos: SectionBlockPos, id: BlockStateId) {
        let pos = pos.pack() as usize;

        match self {
            Self::Uniform(data) => {
                if id != data.get_block() {
                    let mut new_data = PalettedSection::from(data);
                    new_data.set_block(pos, id);
                    *self = Self::Paletted(new_data);
                }
            }
            Self::Paletted(data) => {
                if data.set_block(pos, id).is_none() {
                    let mut new_data = DirectSection::from(data);
                    new_data.set_block(pos, id);
                    *self = Self::Direct(new_data);
                }
            }
            Self::Direct(data) => data.set_block(pos, id),
        }
    }

    #[inline]
    pub fn fill(&mut self, id: BlockStateId) {
        match self {
            Self::Uniform(data) => data.fill(id),
            _ => *self = Self::Uniform(UniformSection::new_with(id)),
        }
    }

    pub fn block_count(&self) -> u16 {
        match self {
            Self::Uniform(data) => {
                if data.get_block() == block!("air") {
                    0
                } else {
                    4096
                }
            }
            Self::Paletted(data) => data.block_count(),
            Self::Direct(data) => data.block_count(),
        }
    }
}

#[derive(Clone, DeepSizeOf, Encode, Decode)]
pub struct ChunkSection {
    pub(crate) inner: ChunkSectionType,
    pub(crate) light: SectionLightData,
    pub(crate) biome: BiomeData,
    pub dirty: bool,
}

impl ChunkSection {
    pub fn new_uniform(id: BlockStateId) -> Self {
        Self {
            inner: ChunkSectionType::Uniform(UniformSection::new_with(id)),
            light: SectionLightData::default(),
            biome: BiomeData::Uniform(BiomeType(5)),
            dirty: true,
        }
    }

    pub fn with_space_for(unique_blocks: u16) -> Self {
        if unique_blocks <= 1 {
            Self {
                inner: ChunkSectionType::Uniform(UniformSection::air()),
                light: SectionLightData::default(),
                biome: BiomeData::Uniform(BiomeType(5)),
                dirty: true,
            }
        } else if unique_blocks < 256 {
            Self {
                inner: ChunkSectionType::Paletted(PalettedSection::new_with_block_count(
                    unique_blocks as _,
                )),
                light: SectionLightData::default(),
                biome: BiomeData::Uniform(BiomeType(5)),
                dirty: true,
            }
        } else {
            Self {
                inner: ChunkSectionType::Direct(DirectSection::default()),
                light: SectionLightData::default(),
                biome: BiomeData::Uniform(BiomeType(5)),
                dirty: true,
            }
        }
    }

    #[inline]
    pub fn get_block(&self, pos: SectionBlockPos) -> BlockStateId {
        self.inner.get_block(pos)
    }

    #[inline]
    pub fn set_block(&mut self, pos: SectionBlockPos, id: BlockStateId) {
        self.dirty = true;
        self.inner.set_block(pos, id);
    }

    #[inline]
    pub fn fill(&mut self, id: BlockStateId) {
        self.dirty = true;
        self.inner.fill(id);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.fill(block!("air"))
    }

    #[inline]
    pub fn block_count(&self) -> u16 {
        self.inner.block_count()
    }
}

impl TryFrom<&Section> for ChunkSection {
    type Error = WorldError;

    fn try_from(value: &Section) -> Result<Self, Self::Error> {
        let sky_light = value.sky_light.clone().map(LightStorage::from).unwrap_or_default();
        let block_light = value.block_light.clone().map(LightStorage::from).unwrap_or_default();

        let light_data = SectionLightData::with_data(sky_light, block_light);

        let block_data = value.block_states.as_ref().ok_or(WorldError::CorruptedChunkData(0, 0))?;
        let blocks = block_data.data.clone().ok_or(WorldError::CorruptedChunkData(0, 0))?;

        if let Some(palette) = block_data.palette.as_ref() {
            Ok(Self {
                light: light_data,
                biome: BiomeData::Uniform(BiomeType(5)),
                dirty: false,

                inner: ChunkSectionType::Paletted(PalettedSection {
                    block_data: blocks.into_iter().map(|v| v as u64).collect(),
                    bit_width: BlockPalette::bit_width_for_len(palette.len()),
                    palette: BlockPalette {
                        free_count: 0,
                        palette: palette.into_iter().map(|data| Some((BlockStateId::from_block_data(&data), NonZeroU16::MAX))).collect(),
                    },
                })
            })
        } else {
            Ok(Self {
                light: light_data,
                biome: BiomeData::new_uniform(BiomeType(5)),
                dirty: false,

                inner: ChunkSectionType::Uniform(UniformSection::new_with(AIR)),
            })
        }
    }
}
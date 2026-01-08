use crate::block_state_id::BlockStateId;
use crate::chunk::light::SectionLightData;
use crate::chunk::section::biome::{BiomeData, BiomeType};
use crate::chunk::section::direct::DirectSection;
use crate::chunk::section::paletted::PalettedSection;
use crate::chunk::section::uniform::UniformSection;
use crate::pos::SectionBlockPos;
use bitcode_derive::{Decode, Encode};
use deepsize::DeepSizeOf;
use ferrumc_macros::block;

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

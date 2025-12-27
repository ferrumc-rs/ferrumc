use deepsize::DeepSizeOf;
use crate::chunk::section::direct::DirectSection;
use crate::chunk::section::paletted::PalettedSection;
use crate::chunk::section::uniform::UniformSection;
use crate::chunk::BlockStateId;
use crate::chunk::light::SectionLightData;
use crate::pos::SectionBlockPos;

mod uniform;
mod paletted;
mod direct;
pub mod network;

pub const CHUNK_SECTION_LENGTH: usize = 16 * 16 * 16;

#[derive(Clone, DeepSizeOf)]
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
                if let None = data.set_block(pos, id) {
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
            Self::Uniform(data) => if data.get_block() == 0 { 0 } else { 4096 },
            Self::Paletted(data) => data.block_count(),
            Self::Direct(data) => data.block_count(),
        }
    }
}

#[derive(Clone, DeepSizeOf)]
pub struct ChunkSection {
    pub(crate) inner: ChunkSectionType,
    pub(crate) light: SectionLightData,
}

impl ChunkSection {
    pub fn new_uniform(id: BlockStateId) -> Self {
        Self {
            inner: ChunkSectionType::Uniform(UniformSection::new_with(id)),
            light: SectionLightData::new(),
        }
    }

    pub fn with_space_for(unique_blocks: u16) -> Self {
        if unique_blocks <= 1 {
            Self {
                inner: ChunkSectionType::Uniform(UniformSection::air()),
                light: SectionLightData::new(),
            }
        } else if unique_blocks < 256 {
            Self {
                inner: ChunkSectionType::Paletted(PalettedSection::new_with_block_count(unique_blocks as _)),
                light: SectionLightData::new(),
            }
        } else {
            Self {
                inner: ChunkSectionType::Direct(DirectSection::new()),
                light: SectionLightData::new(),
            }
        }
    }

    #[inline]
    pub fn get_block(&self, pos: SectionBlockPos) -> BlockStateId {
        self.inner.get_block(pos)
    }

    #[inline]
    pub fn set_block(&mut self, pos: SectionBlockPos, id: BlockStateId) {
        self.inner.set_block(pos, id);
    }

    #[inline]
    pub fn fill(&mut self, id: BlockStateId) {
        self.inner.fill(id);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.fill(0)
    }

    #[inline]
    pub fn block_count(&self) -> u16 {
        self.inner.block_count()
    }
}
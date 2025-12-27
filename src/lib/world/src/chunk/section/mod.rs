use crate::chunk::section::direct::DirectSection;
use crate::chunk::section::paletted::PalettedSection;
use crate::chunk::section::uniform::UniformSection;
use crate::chunk::{BlockStateId, SectionBlockPos};

mod uniform;
mod paletted;
mod direct;

pub const CHUNK_SECTION_LENGTH: usize = 16 * 16 * 16;

enum ChunkSectionType {
    Uniform(UniformSection),
    Paletted(PalettedSection),
    Direct(DirectSection),
}

impl ChunkSectionType {
    #[inline]
    pub fn get_block(&self, pos: SectionBlockPos) -> BlockStateId {
        match self {
            Self::Uniform(data) => data.get_block(),
            Self::Paletted(data) => data.get_block(pos.into()),
            Self::Direct(data) => data.get_block(pos.into()),
        }
    }

    #[inline]
    pub fn set_block(&mut self, pos: SectionBlockPos, id: BlockStateId) {
        match self {
            Self::Uniform(data) => {
                if id != data.get_block() {
                    let mut new_data = PalettedSection::from(data);
                    new_data.set_block(pos.into(), id);
                    *self = Self::Paletted(new_data);
                }
            }
            Self::Paletted(data) => {
                let pos: usize = pos.into();

                if let None = data.set_block(pos, id) {
                    let mut new_data = DirectSection::from(data);
                    new_data.set_block(pos, id);
                    *self = Self::Direct(new_data);
                }
            }
            Self::Direct(data) => data.set_block(pos.into(), id),
        }
    }

    #[inline]
    pub fn fill(&mut self, id: BlockStateId) {
        match self {
            Self::Uniform(data) => data.fill(id),
            _ => *self = Self::Uniform(UniformSection::new_with(id)),
        }
    }
}

pub struct ChunkSection {
    inner: ChunkSectionType,
    // todo: add light data
}

impl ChunkSection {
    pub fn new_uniform(id: BlockStateId) -> Self {
        Self {
            inner: ChunkSectionType::Uniform(UniformSection::new_with(id)),
        }
    }

    pub fn with_space_for(unique_blocks: u16) -> Self {
        if unique_blocks <= 1 {
            Self {
                inner: ChunkSectionType::Uniform(UniformSection::air()),
            }
        } else if unique_blocks < 256 {
            Self {
                inner: ChunkSectionType::Paletted(PalettedSection::new_with_block_count(unique_blocks as _)),
            }
        } else {
            Self {
                inner: ChunkSectionType::Direct(DirectSection::new()),
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
}
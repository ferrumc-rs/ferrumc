use crate::block_state_id::BlockStateId;
use crate::chunk::light::{LightStorage, SectionLightData};
use crate::chunk::section::biome::{BiomeData, BiomeType};
use crate::chunk::section::direct::DirectSection;
use crate::chunk::section::paletted::{PalettedSection, PalettedSectionResult};
use crate::chunk::section::uniform::UniformSection;
use crate::errors::WorldError;
use crate::pos::SectionBlockPos;
use crate::vanilla_chunk_format::Section;
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
        let pos = pos.pack() as usize;

        match self {
            Self::Uniform(data) => data.get_block(),
            Self::Paletted(data) => data.get_block(pos),
            Self::Direct(data) => data.get_block(pos),
        }
    }

    #[inline]
    pub fn set_block(&mut self, pos: SectionBlockPos, id: BlockStateId) {
        let pos = pos.pack() as usize;

        match self {
            Self::Uniform(data) => {
                // Check if the id doesn't match the block type that fills the section,
                // If not, then create a PalettedSection to hold more than one block type
                if id != data.get_block() {
                    let mut new_data = PalettedSection::from(data);
                    new_data.set_block(pos, id);
                    *self = Self::Paletted(new_data);
                }
            }
            Self::Paletted(data) => match data.set_block(pos, id) {
                // Shrink the PalettedSection into a UniformSection if one block fills the entire section
                PalettedSectionResult::Shrink(block) => {
                    *self = ChunkSectionType::Uniform(UniformSection::new_with(block))
                }
                // Expand the PalettedSection into a DirectSection if more than u8::MAX block types are in the section
                PalettedSectionResult::Expand => {
                    let mut new_data = DirectSection::from(data);
                    new_data.set_block(pos, id);
                    *self = Self::Direct(new_data);
                }
                PalettedSectionResult::Keep => {}
            },
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
        let sky_light = value
            .sky_light
            .clone()
            .map(LightStorage::from)
            .unwrap_or_default();
        let block_light = value
            .block_light
            .clone()
            .map(LightStorage::from)
            .unwrap_or_default();

        let light_data = SectionLightData::with_data(sky_light, block_light);

        if let Some(block_data) = value.block_states.as_ref() {
            let (block_count, block_states) = if let Some(blocks) = block_data.data.as_ref() {
                if let Some(palette) = block_data.palette.as_ref() {
                    let bits_per_block =
                        ((palette.len().saturating_sub(1) as u32).ilog2() + 1).max(4);

                    let mut values = Vec::with_capacity(4096);

                    for i in 0..4096 {
                        values.push(PalettedSection::unpack_value_unaligned(
                            bytemuck::cast_slice(blocks.as_slice()),
                            i,
                            bits_per_block as _,
                        ))
                    }

                    debug_assert_eq!(values.len(), 4096);

                    (
                        if bits_per_block >= 9 {
                            None
                        } else {
                            Some(palette.len())
                        },
                        values
                            .into_iter()
                            .map(|v| {
                                if bits_per_block >= 9 {
                                    BlockStateId::new(v as _)
                                } else {
                                    BlockStateId::from_block_data(&palette[v as usize])
                                }
                            })
                            .collect::<Vec<_>>(),
                    )
                } else {
                    return Err(WorldError::CorruptedChunkData(0, 0));
                }
            } else {
                return Ok(Self {
                    light: light_data,
                    biome: BiomeData::Uniform(BiomeType(5)),
                    dirty: false,

                    inner: ChunkSectionType::Uniform(UniformSection::air()),
                });
            };

            let mut section_data = if let Some(block_count) = block_count {
                ChunkSectionType::Paletted(PalettedSection::new_with_block_count(block_count as _))
            } else {
                ChunkSectionType::Direct(DirectSection::default())
            };

            for (idx, block) in block_states.into_iter().enumerate() {
                section_data.set_block(
                    SectionBlockPos::unpack(idx as _).expect("should be in-bounds"),
                    block,
                )
            }

            Ok(Self {
                light: light_data,
                biome: BiomeData::Uniform(BiomeType(5)),
                dirty: false,
                inner: section_data,
            })
        } else {
            Ok(Self {
                light: light_data,
                biome: BiomeData::Uniform(BiomeType(5)),
                dirty: false,
                inner: ChunkSectionType::Uniform(UniformSection::air()),
            })
        }
    }
}

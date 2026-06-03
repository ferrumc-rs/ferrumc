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

#[cfg(test)]
mod section_tests {
    use super::*;
    use crate::pos::SectionBlockPos;
    use ferrumc_macros::block;

    fn sbp(i: usize) -> SectionBlockPos {
        SectionBlockPos::unpack(i as u16).expect("in bounds")
    }

    /// Regression for the disappearing-section bug. A full sand section (a `UniformSection`, as the
    /// terrain generator produces for the all-sand layer at y=48..63) gets blocks replaced by other
    /// types — exactly what mining or water inflow does. Because `BlockPalette::remove_block` used
    /// to discard its count decrement, sand's palette count never dropped; re-adding sand then
    /// pushed the count past 4096 and `add_block` returned `ConvertToUniform`, collapsing the WHOLE
    /// section to a single block (every other block in it vanished). This drives many replacements
    /// against a uniform-sand section and asserts every cell always reads back exactly what was
    /// written.
    #[test]
    fn uniform_sand_section_survives_many_replacements() {
        let sand = block!("sand");
        let water = block!("water", { level: 0 });
        let air = block!("air");

        let mut section = ChunkSectionType::Uniform(UniformSection::new_with(sand));
        // Reference model: what every cell should hold.
        let mut model = vec![sand; CHUNK_SECTION_LENGTH];

        // Simulate water flowing across the top rows and some mining: replace a swathe of cells
        // with water, then air, then sand again (a cell being filled, drained, refilled), many
        // times over — the churn that used to inflate sand's count.
        for round in 0..4 {
            for i in 0..CHUNK_SECTION_LENGTH {
                let new = match (round + i) % 3 {
                    0 => water,
                    1 => air,
                    _ => sand,
                };
                section.set_block(sbp(i), new);
                model[i] = new;
            }
            // After every round, every cell must still read back the model value.
            for i in 0..CHUNK_SECTION_LENGTH {
                assert_eq!(
                    section.get_block(sbp(i)),
                    model[i],
                    "cell {i} corrupted after round {round} (section collapsed?)"
                );
            }
        }
    }

    /// Tighter focus on the exact collapse trigger: a uniform sand section with a single cell
    /// repeatedly toggled sand→water→sand must never lose its other 4095 sand blocks. Before the
    /// fix, the toggling inflated the count until the section collapsed to uniform water/air.
    #[test]
    fn toggling_one_cell_does_not_collapse_section() {
        let sand = block!("sand");
        let water = block!("water", { level: 0 });

        let mut section = ChunkSectionType::Uniform(UniformSection::new_with(sand));

        for _ in 0..5000 {
            section.set_block(sbp(0), water);
            section.set_block(sbp(0), sand);
        }

        // Every cell is still sand.
        for i in 0..CHUNK_SECTION_LENGTH {
            assert_eq!(
                section.get_block(sbp(i)),
                sand,
                "cell {i} should still be sand"
            );
        }
        assert_eq!(section.block_count(), CHUNK_SECTION_LENGTH as u16);
    }
}

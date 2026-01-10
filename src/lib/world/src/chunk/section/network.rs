use crate::chunk::section::biome::BiomeData;
use crate::chunk::section::direct::DirectSection;
use crate::chunk::section::paletted::PalettedSection;
use crate::chunk::section::uniform::UniformSection;
use crate::chunk::section::{ChunkSection, ChunkSectionType, CHUNK_SECTION_LENGTH};
use ferrumc_macros::NetEncode;
use ferrumc_net_codec::net_types::net_array::NetworkArray;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetEncode)]
pub struct PalettedContainer<'section> {
    bits_per_entry: u8,
    palette: NetworkPalette,
    data_array: NetworkArray<'section, u64>,
}

#[derive(NetEncode)]
pub enum NetworkPalette {
    SingleValued {
        value: VarInt,
    },
    Indirect {
        palette_length: VarInt,
        palette_values: Vec<VarInt>,
    },
    Direct {
        // No values
    },
}

#[derive(NetEncode)]
pub struct NetworkSection<'section> {
    block_count: u16,
    block_states: PalettedContainer<'section>,
    biomes: PalettedContainer<'section>,
}

impl<'section> From<&'section UniformSection> for PalettedContainer<'section> {
    fn from(section: &'section UniformSection) -> Self {
        PalettedContainer {
            bits_per_entry: 0,
            palette: NetworkPalette::SingleValued {
                value: VarInt(section.get_block().raw() as _),
            },
            data_array: NetworkArray::new_owned(vec![]),
        }
    }
}

impl<'section> From<&'section PalettedSection> for PalettedContainer<'section> {
    fn from(section: &'section PalettedSection) -> Self {
        let bits_per_entry = section.bit_width.max(4); // Minecraft supports lowest bit width of 4 for indirect palettes
        let data_array: NetworkArray<u64> = if bits_per_entry != section.bit_width {
            let mut new_buffer = vec![
                0u64;
                (CHUNK_SECTION_LENGTH / (8 / bits_per_entry as usize))
                    / size_of::<u64>()
            ];

            for block in 0..CHUNK_SECTION_LENGTH {
                PalettedSection::pack_value(
                    &mut new_buffer,
                    block,
                    bits_per_entry,
                    PalettedSection::unpack_value(&section.block_data, block, section.bit_width),
                );
            }

            NetworkArray::new_owned(new_buffer)
        } else {
            NetworkArray::new_borrowed(&section.block_data)
        };

        PalettedContainer {
            bits_per_entry,
            palette: NetworkPalette::Indirect {
                palette_length: VarInt(section.palette.len() as _),
                palette_values: section
                    .palette
                    .palette_data()
                    .into_iter()
                    .map(|v| VarInt(v.raw() as _))
                    .collect(),
            },
            data_array,
        }
    }
}

impl<'section> From<&'section DirectSection> for PalettedContainer<'section> {
    fn from(_section: &'section DirectSection) -> Self {
        PalettedContainer {
            bits_per_entry: 16,
            palette: NetworkPalette::Direct {},
            data_array: NetworkArray::new_owned(vec![]), // TODO: fix this to use the data from the section; bytemuck::cast_slice(&section.0)
        }
    }
}

impl<'section> From<&'section ChunkSection> for PalettedContainer<'section> {
    fn from(value: &'section ChunkSection) -> Self {
        match &value.inner {
            ChunkSectionType::Uniform(data) => PalettedContainer::from(data),
            ChunkSectionType::Paletted(data) => PalettedContainer::from(data),
            ChunkSectionType::Direct(data) => PalettedContainer::from(data),
        }
    }
}

impl<'section> From<&'section BiomeData> for PalettedContainer<'section> {
    fn from(value: &'section BiomeData) -> Self {
        match value {
            BiomeData::Uniform(data) => PalettedContainer {
                bits_per_entry: 0,
                palette: NetworkPalette::SingleValued {
                    value: VarInt(data.0 as _),
                },
                data_array: NetworkArray::new_owned(vec![]),
            },
            BiomeData::Mixed(data) => PalettedContainer {
                bits_per_entry: 8,
                palette: NetworkPalette::Direct {},
                data_array: NetworkArray::new_borrowed(bytemuck::cast_slice(data)),
            },
        }
    }
}

impl<'section> From<&'section ChunkSection> for NetworkSection<'section> {
    fn from(value: &'section ChunkSection) -> Self {
        Self {
            block_count: value.block_count(),
            block_states: PalettedContainer::from(value),
            biomes: PalettedContainer::from(&value.biome),
        }
    }
}

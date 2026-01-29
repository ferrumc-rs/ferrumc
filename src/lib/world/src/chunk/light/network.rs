use crate::chunk::Chunk;
use ferrumc_macros::NetEncode;
use ferrumc_net_codec::net_types::bitset::BitSet;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;

const FULL_SECTION_LIGHT: &[u8] = &[u8::MAX; 2048];

#[derive(NetEncode)]
pub struct LightDataArray<'chunk> {
    length: VarInt,
    data: &'chunk [u8],
}

#[derive(NetEncode)]
pub struct NetworkLightData<'chunk> {
    pub sky_light_mask: BitSet,
    pub block_light_mask: BitSet,
    pub empty_sky_light_mask: BitSet,
    pub empty_block_light_mask: BitSet,
    pub sky_light_arrays: LengthPrefixedVec<LightDataArray<'chunk>>,
    pub block_light_arrays: LengthPrefixedVec<LightDataArray<'chunk>>,
}

impl<'chunk> From<&'chunk Chunk> for NetworkLightData<'chunk> {
    fn from(chunk: &'chunk Chunk) -> Self {
        let height = chunk.height;
        let total_sections = (height.height as usize) / 16;

        let mask_size = total_sections + 2;

        let mut sky_light_mask = BitSet::new(mask_size);
        let mut block_light_mask = BitSet::new(mask_size);
        let mut empty_sky_light_mask = BitSet::new(mask_size);
        let mut empty_block_light_mask = BitSet::new(mask_size);

        let mut sky_light_arrays = Vec::with_capacity(total_sections);
        let mut block_light_arrays = Vec::with_capacity(total_sections);

        for (i, section) in chunk.sections.iter().enumerate() {
            let bit_index = i + 1;

            // Skylight
            let sky_light = &section.light.sky_light;
            if sky_light.is_empty() {
                empty_sky_light_mask.set(bit_index, true);
            } else {
                sky_light_mask.set(bit_index, true);
                sky_light_arrays.push(LightDataArray {
                    length: VarInt(2048),
                    data: &*sky_light.light_data,
                });
            }

            // Block Light
            let block_light = &section.light.block_light;
            if block_light.is_empty() {
                empty_block_light_mask.set(bit_index, true);
            } else {
                block_light_mask.set(bit_index, true);
                block_light_arrays.push(LightDataArray {
                    length: VarInt(2048),
                    data: &*block_light.light_data,
                });
            }
        }

        Self {
            sky_light_mask,
            empty_sky_light_mask,
            sky_light_arrays: LengthPrefixedVec::new(sky_light_arrays),

            block_light_mask,
            empty_block_light_mask,
            block_light_arrays: LengthPrefixedVec::new(block_light_arrays),
        }
    }
}

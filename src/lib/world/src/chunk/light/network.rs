use crate::chunk::light::LightStorage;
use crate::chunk::Chunk;
use ferrumc_macros::NetEncode;
use ferrumc_net_codec::net_types::bitset::BitSet;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::ops::Not;

const FULL_SECTION_LIGHT: &[u8] = &[u8::MAX; 2048];
const NUM_SECTIONS: usize = 24;

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
        let mut sky_light_mask = BitSet::new(NUM_SECTIONS + 2);
        let mut block_light_mask = BitSet::new(NUM_SECTIONS + 2);

        let mut sky_light_arrays = Vec::with_capacity(NUM_SECTIONS);
        let mut block_light_arrays = Vec::with_capacity(NUM_SECTIONS);

        for (i, section) in chunk.sections.iter().enumerate() {
            sky_light_mask.set(i, section.light.contains_sky_light());
            block_light_mask.set(i, section.light.contains_block_light());

            if section.light.contains_sky_light() {
                sky_light_arrays.push(LightDataArray {
                    length: VarInt(2048),
                    data: match &section.light.sky_light {
                        LightStorage::Empty => unreachable!(),
                        LightStorage::Full => FULL_SECTION_LIGHT,
                        LightStorage::Mixed { light_data } => light_data,
                    },
                })
            }

            if section.light.contains_block_light() {
                block_light_arrays.push(LightDataArray {
                    length: VarInt(2048),
                    data: match &section.light.block_light {
                        LightStorage::Empty => unreachable!(),
                        LightStorage::Full => FULL_SECTION_LIGHT,
                        LightStorage::Mixed { light_data } => light_data,
                    },
                })
            }
        }

        let empty_sky_light_mask = sky_light_mask.clone().not();
        let empty_block_light_mask = block_light_mask.clone().not();

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

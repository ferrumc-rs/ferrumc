use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;
use ferrumc_net_codec::net_types::bitset::BitSet;

#[derive(NetEncode)]
pub struct BlockEntity {
    pub xz: u8,
    pub y: u16,
    pub entity_type: VarInt,
    pub nbt: Vec<u8>
}

#[derive(NetEncode)]
#[packet(packet_id = 0x27)]
pub struct ChunkAndLightData {
    pub chunk_x: i32,
    pub chunk_z: i32,
    // The binary nbt data
    pub heightmaps: Vec<u8>,
    pub data: LengthPrefixedVec<u8>,
    pub block_entities: LengthPrefixedVec<BlockEntity>,
    pub sky_light_mask: BitSet,
    pub block_light_mask: BitSet,
    pub empty_sky_light_mask: BitSet,
    pub empty_block_light_mask: BitSet,
    pub sky_light_arrays: LengthPrefixedVec<LengthPrefixedVec<u8>>,
    pub block_light_arrays: LengthPrefixedVec<LengthPrefixedVec<u8>>,
}
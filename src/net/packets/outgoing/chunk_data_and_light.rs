use crate::utils::encoding::bitset::BitSet;
use crate::utils::encoding::varint::VarInt;
use ferrumc_macros::Encode;
use nbt_lib::NBTTag;

#[derive(Encode)]
struct ChunkDataAndLight {
    #[encode(default = VarInt::from(0x24))]
    pub packet_id: VarInt,
    chunk_x: i32,
    chunk_z: i32,
    height_maps: NBTTag,
    size: VarInt,
    #[encode(raw_bytes(prepend_length = false))]
    data: Vec<u8>,
    number_of_block_entities: VarInt,
    #[encode(raw_bytes(prepend_length = false))]
    block_entities: Vec<u8>,
    sky_light_mask: BitSet,
    block_light_mark: BitSet,
    empty_sky_light_mask: BitSet,
    empty_block_light_mask: BitSet,
    sky_light_array_count: VarInt,
    #[encode(raw_bytes(prepend_length = false))]
    sky_light_arrays: Vec<u8>,
    block_light_array_count: VarInt,
    #[encode(raw_bytes(prepend_length = false))]
    block_light_arrays: Vec<u8>,
}

impl ChunkDataAndLight {

}
use ferrumc_macros::Encode;
use nbt_lib::NBTTag;
use crate::utils::encoding::varint::VarInt;

#[derive(Encode)]
struct ChunkDataAndLight {
    chunk_x: i32,
    chunk_z: i32,
    height_maps: NBTTag,
    size: VarInt,
    #[encode(raw_bytes(prepend_length = false))]
    data: Vec<u8>,
    number_of_block_entities: VarInt,
    #[encode(raw_bytes(prepend_length = false))]
    block_entities: Vec<u8>,
    sky_light_mask: VarInt,
}
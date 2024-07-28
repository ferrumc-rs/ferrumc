use ferrumc_macros::Encode;
use simdnbt::owned::NbtCompound;
use simdnbt::Serialize;
use crate::utils::encoding::varint::VarInt;
use crate::world::sweattypalms_impl::types::{Chunk, ChunkSection};

#[derive(Encode)]
pub struct ChunkDataAndUpdateLight {
    #[encode(default=VarInt::from(0x24))]
    pub packet_id: VarInt,
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub heightmaps: Vec<u8>, // NBT encoded
    pub data: Vec<u8>,
    pub block_entities: Vec<BlockEntity>,
    pub trust_edges: bool,
    pub sky_light_mask: Vec<i64>,
    pub block_light_mask: Vec<i64>,
    pub empty_sky_light_mask: Vec<i64>,
    pub empty_block_light_mask: Vec<i64>,
    pub sky_light: Vec<Vec<u8>>,
    pub block_light: Vec<Vec<u8>>,
}

#[derive(Encode)]
pub struct BlockEntity {
    pub packed_xz: u8,
    pub y: i16,
    pub type_id: VarInt,
    pub data: Vec<u8>, // NBT encoded
}

impl ChunkDataAndUpdateLight {
    pub fn new(chunk: &Chunk, dimension_height: i32, min_y: i32) -> Self {
        let chunk_x = chunk.x;
        let chunk_z = chunk.z;

        // Create heightmaps NBT
        let heightmaps = chunk.heightmaps.clone().to_nbt();
        
        // Create chunk data
        let mut data: Vec<ChunkSection> = Vec::new();
        /*for section in &chunk.sections {
            let mut section_data = Vec::new();
            section_data.push(section.y as u8);
            section_data.extend_from_slice(&section.block_states.palette);
            section_data.extend_from_slice(&section.block_states.data);
            section_data.extend_from_slice(&section.biomes.palette);
            section_data.extend_from_slice(&section.biomes.data);
            data.extend_from_slice(&section_data);
        }*/

        todo!()
    }
}
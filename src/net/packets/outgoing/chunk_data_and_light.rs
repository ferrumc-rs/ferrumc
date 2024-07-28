use std::io::Cursor;
use ferrumc_macros::Encode;
use simdnbt::Serialize;
use tokio::io::AsyncWriteExt;
use crate::utils::encoding::varint::VarInt;
use crate::utils::type_impls::Encode;
use crate::utils::prelude::*;
use crate::world::sweattypalms_impl::types::{Chunk, ChunkSection};

#[derive(Encode)]
pub struct ChunkDataAndUpdateLight {
    #[encode(default = VarInt::from(0x24))]
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
    pub async fn new(chunk: &Chunk) -> Result<Self> {
        let chunk_x = chunk.x;
        let chunk_z = chunk.z;

        // Create heightmaps NBT
        let heightmaps = chunk.heightmaps.clone().to_nbt();
        let mut heightmaps_buf = Vec::new();
        heightmaps.write(&mut heightmaps_buf);

        // Create chunk data
        let mut buffer = Cursor::new(Vec::new());

        // Always send 24 sections for consistency
        for _ in 0..24 {
            let chunk_section_enc = ChunkSectionEncode {
                block_count: 4096,
                bit_per_entry: 4,
                palette: VarInt::from(1),
                data_array_length: VarInt::from(256), // 4096 / 16
                data_array: vec![0; 256], // All air
                biome_bit_per_entry: 0,
                biome_palette: VarInt::from(1),
                biome_data_array_length: VarInt::from(0),
            };

            chunk_section_enc.encode(&mut buffer).await?;
        }
        let data = buffer.into_inner();

        // Light data
        let sky_light_mask = vec![0xFFFFFFFFFFFFFFFFu64 as i64; 1]; // All sections have skylight
        let block_light_mask = vec![0; 1]; // No sections have blocklight
        let empty_sky_light_mask = vec![0; 1];
        let empty_block_light_mask = vec![0xFFFFFFFFFFFFFFFFu64 as i64; 1]; // All sections have empty blocklight

        let sky_light = vec![vec![0xFF; 2048]; 24]; // Full brightness for all sections
        let block_light = Vec::new(); // No block light data

        Ok(Self {
            packet_id: VarInt::from(0x24),
            chunk_x,
            chunk_z,
            heightmaps: heightmaps_buf,
            data,
            block_entities: Vec::new(),
            trust_edges: true,
            sky_light_mask,
            block_light_mask,
            empty_sky_light_mask,
            empty_block_light_mask,
            sky_light,
            block_light,
        })
    }
}

#[derive(Encode)]
struct ChunkSectionEncode {
    block_count: i16,
    bit_per_entry: u8,
    palette: VarInt,
    data_array_length: VarInt,
    data_array: Vec<u8>,
    biome_bit_per_entry: u8,
    biome_palette: VarInt,
    biome_data_array_length: VarInt,
}
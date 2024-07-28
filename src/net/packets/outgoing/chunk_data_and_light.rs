use std::io::Cursor;
use ferrumc_macros::Encode;
use simdnbt::Serialize;
use tokio::io::AsyncWriteExt;
use crate::utils::encoding::varint::VarInt;
use crate::utils::type_impls::Encode;
use crate::utils::prelude::*;
use crate::world::sweattypalms_impl::types::{Chunk, ChunkSection, PalettedContainer};

#[derive(Encode)]
pub struct ChunkDataAndUpdateLight {
    #[encode(default = VarInt::from(0x24))]
    pub packet_id: VarInt,
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub heightmaps: Vec<u8>, // NBT encoded
    #[encode(prepend_length = true)]
    pub data: Vec<u8>,
    #[encode(prepend_length = true)]
    pub block_entities: Vec<BlockEntity>,
    #[encode(prepend_length = true)]
    pub sky_light_mask: Vec<i64>, // Bitset -> must prepend length
    #[encode(prepend_length = true)]
    pub block_light_mask: Vec<i64>,
    #[encode(prepend_length = true)]
    pub empty_sky_light_mask: Vec<i64>,
    #[encode(prepend_length = true)]
    pub empty_block_light_mask: Vec<i64>,
    pub sky_light_array_count: VarInt,
    pub sky_light: Vec<SkyLightArray>,
    pub block_light_array_count: VarInt,
    pub block_light: Vec<BlockLightArray>,
}

#[derive(Encode)]
pub struct BlockEntity {
    pub packed_xz: u8,
    pub y: i16,
    pub type_id: VarInt,
    pub data: Vec<u8>, // NBT encoded
}

#[derive(Encode)]
pub struct SkyLightArray {
    pub length: VarInt,
    pub data: Vec<u8>,
}

#[derive(Encode)]
pub struct BlockLightArray {
    pub length: VarInt,
    pub data: Vec<u8>,
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

        for chunk_section in chunk.sections.iter() {
            let chunk_section_enc = ChunkSectionEncode {
                block_count: 4096,
                block_states: chunk_section.block_states.clone(),
                biomes: chunk_section.biomes.clone(),
            };

            chunk_section_enc.encode(&mut buffer).await?;
        }
        let data = buffer.into_inner();

        // Light data
        let sky_light_mask = vec![0xFFFFFFFFFFFFFFFFu64 as i64; 1]; // All sections have skylight
        let block_light_mask = vec![0; 1]; // No sections have blocklight
        let empty_sky_light_mask = vec![0; 1];
        let empty_block_light_mask = vec![0xFFFFFFFFFFFFFFFFu64 as i64; 1]; // All sections have empty blocklight

        let sky_light = (0..24).map(|_| SkyLightArray {
            length: VarInt::from(2048),
            data: vec![0xFF; 2048], // Full brightness
        }).collect();

        Ok(Self {
            packet_id: VarInt::from(0x24),
            chunk_x,
            chunk_z,
            heightmaps: heightmaps_buf,
            data,
            block_entities: Vec::new(),
            sky_light_mask,
            block_light_mask,
            empty_sky_light_mask,
            empty_block_light_mask,
            sky_light_array_count: VarInt::from(chunk.sections.len() as i32),
            sky_light,
            block_light_array_count: VarInt::from(0),
            block_light: Vec::new(),
        })
    }
}

#[derive(Encode)]
struct ChunkSectionEncode {
    block_count: i16,
    block_states: PalettedContainer,
    biomes: PalettedContainer
    /*bit_per_entry: u8,
    palette: Vec<VarInt>,
    data_array_length: VarInt,
    data_array: Vec<u8>,
    biome_bit_per_entry: u8,
    biome_palette: Vec<VarInt>,
    biome_data_array_length: VarInt,*/
}
/*use std::io::Cursor;
use ferrumc_macros::Encode;
use simdnbt::Serialize;
use tokio::io::AsyncWriteExt;
use crate::utils::encoding::bitset::BitSet;
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
    #[encode(raw_bytes(prepend_length = true))]
    pub data: Vec<u8>,
    pub block_entities_num: VarInt,
    pub block_entities: Vec<BlockEntity>,
    pub sky_light_mask: BitSet,
    pub block_light_mask: BitSet,
    pub empty_sky_light_mask: BitSet,
    pub empty_block_light_mask: BitSet,
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
    /*pub async fn new(chunk: &Chunk) -> Result<Self> {
        let chunk_x = chunk.x;
        let chunk_z = chunk.z;

        // Create heightmaps NBT
        let heightmaps = chunk.heightmaps.clone().to_nbt();
        let mut heightmaps_buf = Vec::new();
        heightmaps.write(&mut heightmaps_buf);

        // Create chunk data
        let mut buffer = Cursor::new(Vec::new());

        for _ in chunk.sections.iter() {
            let chunk_section_enc = ChunkSectionEncode {
                block_count: 4096,
                bit_per_entry: 0,
                palette: VarInt::from(1),
                data_array_length: VarInt::from(0),
                biome_bit_per_entry: 0,
                biome_palette: VarInt::from(55),
                biome_data_array_length: VarInt::from(0),
            };

            chunk_section_enc.encode(&mut buffer).await?;
        }
        let data = buffer.into_inner();

        // Light data
        let mut sky_light_mask = BitSet::new();
        let block_light_mask = BitSet::new();
        let empty_sky_light_mask = BitSet::new();
        let mut empty_block_light_mask = BitSet::new();

        let mut sky_light = Vec::new();

        // Set sky light for all sections
        for i in 0..24 {
            sky_light_mask.set(i);
            empty_block_light_mask.set(i);
            sky_light.push(SkyLightArray {
                length: VarInt::from(2048),
                data: vec![0xFF; 2048],
            });
        }


        Ok(ChunkDataAndUpdateLight::new_auto(
            chunk_x,
            chunk_z,
            heightmaps_buf,
            data,
            VarInt::from(0),
            Vec::new(),
            sky_light_mask,
            block_light_mask,
            empty_sky_light_mask,
            empty_block_light_mask,
            VarInt::from(24),
            sky_light,
            VarInt::from(0),
            Vec::new()
        ))
    }*/
    pub async fn new(chunk: &Chunk) -> Result<Self> {
        let chunk_x = chunk.x;
        let chunk_z = chunk.z;

        // Create heightmaps NBT
        let heightmaps = chunk.heightmaps.clone().to_nbt();
        let mut heightmaps_buf = Vec::new();
        heightmaps.write(&mut heightmaps_buf);

        // Create chunk data
        let mut buffer = Cursor::new(Vec::new());

        let mut mask = 0u32;
        for (i, section) in chunk.sections.iter().enumerate() {
            if !section.block_states.data.is_none() {
                mask |= 1 << i;
                let chunk_section_enc = ChunkSectionEncode::from(section);
                chunk_section_enc.encode(&mut buffer).await?;
            }
        }

        let data = buffer.into_inner();

        // Light data
        let mut sky_light_mask = BitSet::new();
        let block_light_mask = BitSet::new();
        let empty_sky_light_mask = BitSet::new();
        let mut empty_block_light_mask = BitSet::new();

        let mut sky_light = Vec::new();

        // Set sky light only for non-empty sections
        for i in 0..24 {
            if mask & (1 << i) != 0 {
                sky_light_mask.set(i);
                empty_block_light_mask.set(i);
                sky_light.push(SkyLightArray {
                    length: VarInt::from(2048),
                    data: vec![0xFF; 2048],
                });
            }
        }

        Ok(ChunkDataAndUpdateLight::new_auto(
            chunk_x,
            chunk_z,
            heightmaps_buf,
            data,
            VarInt::from(0),
            Vec::new(),
            sky_light_mask,
            block_light_mask,
            empty_sky_light_mask,
            empty_block_light_mask,
            VarInt::from(sky_light.len() as i32),
            sky_light,
            VarInt::from(0),
            Vec::new()
        ))
    }
}

/*#[derive(Encode)]
struct ChunkSectionEncode {
    block_count: i16,
    bit_per_entry: u8,
    palette: VarInt,
    data_array_length: VarInt,
    biome_bit_per_entry: u8,
    biome_palette: VarInt,
    biome_data_array_length: VarInt,
}*/
#[derive(Encode)]
struct ChunkSectionEncode {
    block_count: i16,
    blocks: PalettedContainer,
    biomes: PalettedContainer,
}

impl From<&ChunkSection> for ChunkSectionEncode {
    fn from(section: &ChunkSection) -> Self {
        Self {
            block_count: 4096,
            blocks: section.block_states.clone(),
            biomes: section.biomes.clone(),
        }
    }
}*/

use std::io::Cursor;

use ferrumc_macros::Encode;
use simdnbt::{Deserialize, Serialize, ToNbtTag};
use simdnbt::owned::{NbtCompound, NbtTag};
use tokio::io::AsyncWriteExt;

use crate::net::systems::chunk_sender::GET_REGION;
use crate::utils::encoding::bitset::BitSet;
use crate::utils::encoding::varint::VarInt;
use crate::utils::prelude::*;
use crate::utils::type_impls::Encode;
use crate::world::sweattypalms_impl::types::{Chunk, Heightmaps};

const STONE_BLOCK_ID: u32 = 1; // Assuming 1 is the ID for stone, adjust if needed

#[derive(Encode)]
pub struct ChunkDataAndUpdateLight {
    #[encode(default = VarInt::from(0x24))]
    pub packet_id: VarInt,
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub heightmaps: Vec<u8>, // NBT encoded
    #[encode(raw_bytes(prepend_length = true))]
    pub data: Vec<u8>,
    pub block_entities_num: VarInt,
    pub block_entities: Vec<BlockEntity>,
    pub sky_light_mask: BitSet,
    pub block_light_mask: BitSet,
    pub empty_sky_light_mask: BitSet,
    pub empty_block_light_mask: BitSet,
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
    pub async fn new(chunk_x: i32, chunk_z: i32) -> Result<Self> {
        // Create dummy heightmaps NBT
        let mut heightmaps = create_dummy_heightmaps().await;
        let mut heightmaps_buf = Vec::new();
        heightmaps.to_nbt().write(&mut heightmaps_buf);

        // Create dummy chunk data
        let mut buffer = Cursor::new(Vec::new());
        for _ in 0..24 { // 24 sections for 1.18+ worlds
            let chunk_section_enc = ChunkSectionEncode::new_stone();
            chunk_section_enc.encode(&mut buffer).await?;
        }
        let data = buffer.into_inner();

        // Light data
        let mut sky_light_mask = BitSet::new();
        let block_light_mask = BitSet::new();
        let empty_sky_light_mask = BitSet::new();
        let mut empty_block_light_mask = BitSet::new();

        let mut sky_light = Vec::new();

        // Set sky light for all sections
        for i in 0..24 {
            sky_light_mask.set(i);
            empty_block_light_mask.set(i);
            sky_light.push(SkyLightArray {
                length: VarInt::from(2048),
                data: vec![0xFF; 2048], // Full sky light
            });
        }

        Ok(ChunkDataAndUpdateLight {
            packet_id: VarInt::from(0x24),
            chunk_x,
            chunk_z,
            heightmaps: heightmaps_buf,
            data,
            block_entities_num: VarInt::from(0),
            block_entities: Vec::new(),
            sky_light_mask,
            block_light_mask,
            empty_sky_light_mask,
            empty_block_light_mask,
            sky_light_array_count: VarInt::from(24),
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
    biomes: PalettedContainer,
}

impl ChunkSectionEncode {
    fn new_stone() -> Self {
        ChunkSectionEncode {
            block_count: 4096, // All blocks are stone (non-air)
            block_states: PalettedContainer::new_single_state(STONE_BLOCK_ID),
            biomes: PalettedContainer::new_single_biome(1), // Assuming 1 is a valid biome ID, adjust if needed
        }
    }
}

#[derive(Encode)]
struct PalettedContainer {
    bits_per_entry: u8,
    palette: Vec<VarInt>,
    data_array_length: VarInt,
    data_array: Vec<u64>,
}

impl PalettedContainer {
    fn new_single_state(state_id: u32) -> Self {
        PalettedContainer {
            bits_per_entry: 0,
            palette: vec![VarInt::from(state_id as i32)],
            data_array_length: VarInt::from(0),
            data_array: Vec::new(),
        }
    }

    fn new_single_biome(biome_id: u32) -> Self {
        PalettedContainer {
            bits_per_entry: 0,
            palette: vec![VarInt::from(biome_id as i32)],
            data_array_length: VarInt::from(0),
            data_array: Vec::new(),
        }
    }
}

async fn create_dummy_heightmaps() -> Heightmaps {
    let mut  chunk_read = GET_REGION().write().await;
    let chunk = chunk_read.read_chunk(15, 30).unwrap(); // Get a chunk from the region
    let chunk = chunk.expect("Failed to read chunk data");
    drop(chunk_read);

    /*// let mut compound = simdnbt::Compound::new();
    let mut compound = NbtCompound::new();

    // Create a dummy MOTION_BLOCKING heightmap (all at max height)
    let motion_blocking: Vec<i64> = vec![0x7FFFFFFFFFFFFFFF; 36]; // 36 longs for a 16x16 chunk
    compound.insert("MOTION_BLOCKING", NbtTag::LongArray(motion_blocking));

    // You might want to add WORLD_SURFACE as well, but it's not strictly necessary
    compound*/

    let base_nbt = simdnbt::borrow::read(&mut Cursor::new(&chunk)).expect("Failed to parse chunk data^1").unwrap();

    let chunk = Chunk::from_nbt(&base_nbt).expect("Failed to parse chunk data^2");


    chunk.heightmaps
}

#[tokio::test]
async fn try_load_heightmaps() {
    let heightmaps = create_dummy_heightmaps().await;
    println!("{:?}", heightmaps);
    let nbt = heightmaps.to_nbt();

    let mut buf = Vec::new();
    nbt.write(&mut buf);
    println!("{:?}", buf);

    /*let nbt = heightmaps.to_compound();
    // convert nbt into bytes
    let mut buf = Vec::new();
    nbt.write(&mut buf);

    println!("{:?}", buf);*/
}

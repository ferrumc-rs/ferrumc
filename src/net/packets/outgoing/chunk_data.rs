/*use std::io::{Cursor};
use ferrumc_macros::Encode;
use tokio::io::{AsyncWriteExt, AsyncWrite};
use nbt_lib::NBTDeserializeBytes;
use crate::database::Database;
use crate::state::GlobalState;
use crate::utils::encoding::bitset::BitSet;
use crate::world::chunkformat::{Section, BlockStates, Chunk};
use crate::utils::error::Error;
use crate::utils::encoding::varint::VarInt;
use crate::utils::impls::type_impls::Encode;
use crate::utils::prelude::*;

const SECTION_WIDTH: usize = 16;
const SECTION_HEIGHT: usize = 16;
const CHUNK_HEIGHT: usize = 256; // Adjust if your world height is different

#[derive(Encode)]
pub struct ChunkDataPacket {
    #[encode(default=VarInt::from(0x20))]
    pub packet_id: VarInt,
    pub chunk_x: i32,
    pub chunk_z: i32,
    #[encode(default=vec![&0])]// empty heightmaps
    pub height_maps: Vec<u8>,
    #[encode(raw_bytes(prepend_length = true))]
    pub data: Vec<u8>,
    #[encode(default=VarInt::from(0))]
    pub number_of_block_entities: VarInt,
    pub sky_light_mask: BitSet,
    pub block_light_mask: BitSet,
    pub empty_sky_light_mask: BitSet,
    pub empty_block_light_mask: BitSet,
    #[encode(default=VarInt::from(0))]
    pub sky_light_array_count: VarInt,
    #[encode(default=VarInt::from(0))]
    pub block_light_array_count: VarInt,
}

impl ChunkDataPacket {
    pub async fn new(_state: GlobalState, x: i32, z: i32) -> Result<Self> {
        /*let db = &state.database;
        let chunk = db.get_chunk(x, z, "overworld").await?
            .ok_or(Error::Generic(format!("Chunk {} {} not found", x, z)))?;*/
        let chunk = get_region_file_and_read_chunk(x, z).await?;

        let mut data = Cursor::new(Vec::new());
        let mut column_buffer = Cursor::new(Vec::new());
        // let mut mask = 0;
        let mut mask = BitSet::new(0);

        for y in 0..(CHUNK_HEIGHT / SECTION_HEIGHT) {
            if let Some(section) = chunk.sections.as_ref().and_then(|sections| sections.get(y)) {
                if !is_section_empty(section) {
                    // mask |= 1 << y;
                    mask.set(y);
                    write_chunk_section(section, &mut column_buffer).await?;
                }
            }
        }

        // Write primary bit mask
        VarInt::from(mask).encode(&mut data).await?;

        // Write heightmaps (empty for now)
        // data.extend_from_slice(&[0]); // Empty compound tag
        VarInt::from(0).encode(&mut data).await?;

        // Write biomes (using a placeholder value for now)
        for _ in 0..SECTION_WIDTH * SECTION_WIDTH {
            VarInt::from(127).encode(&mut column_buffer).await?; // Use 127 as a placeholder biome ID
        }

        // Write data size and data
        VarInt::from(column_buffer.get_ref().len() as i32).encode(&mut data).await?;
        // data.extend_from_slice(&column_buffer);
        // data.write_all(&column_buffer).await?;
        data.write_all(&column_buffer.get_ref()).await?;

        // Write number of block entities (0 for now)
        VarInt::from(0).encode(&mut data).await?;

        /*Ok(ChunkDataPacket {
            chunk_x: x,
            chunk_z: z,
            data: data.into_inner(),
        })*/
        Ok(ChunkDataPacket::new_auto(
            x,
            z,
            data.into_inner(),
            mask,
            BitSet::new(0),
            BitSet::new(0),
            BitSet::new(0),
        ))
    }
}

fn is_section_empty(section: &Section) -> bool {
    section.block_states.as_ref()
        .and_then(|bs| bs.data.as_ref())
        .map_or(true, |data| data.iter().all(|&x| x == 0))
}

async fn write_chunk_section(section: &Section, buffer: &mut Cursor<Vec<u8>>) -> Result<()> {
    // Write block count (assuming non-empty for simplicity)
    // buffer.extend_from_slice(&(4096_u16).to_be_bytes());
    buffer.write_all(&(4096_u16).to_be_bytes()).await?;

    if let Some(block_states) = &section.block_states {
        write_block_states(block_states, buffer).await?;
    } else {
        // Write empty block states
        write_empty_block_states(buffer).await?;
    }

    // Skipping light data for simplicity
    Ok(())
}

async fn write_block_states(block_states: &BlockStates, buffer: &mut Cursor<Vec<u8>>) -> Result<()> {
    // Using global palette, so 15 bits per block
    let bits_per_block = 15;
    // buffer.push(bits_per_block);
    buffer.write_all(&[bits_per_block]).await?;

    // No palette for global palette
    VarInt::from(0).encode(buffer).await?;

    // Write block state data
    if let Some(data) = &block_states.data {
        VarInt::from(data.len() as i32).encode(buffer).await?;
        for &long in data {
            // buffer.extend_from_slice(&long.to_be_bytes());
            buffer.write_all(&long.to_be_bytes()).await?;
        }
    } else {
        // Write empty data
        VarInt::from(0).encode(buffer).await?;
    }

    Ok(())
}

async fn write_empty_block_states(buffer: &mut Cursor<Vec<u8>>) -> Result<()> {
    // Using global palette, so 15 bits per block
    let bits_per_block = 15;
    // buffer.push(bits_per_block);
    buffer.write_all(&[bits_per_block]).await?;

    // No palette for global palette
    VarInt::from(0).encode(buffer).await?;

    // Write empty data (all air blocks)
    let data_length = (SECTION_WIDTH * SECTION_WIDTH * SECTION_HEIGHT * 15) / 64;
    VarInt::from(data_length as i32).encode(buffer).await?;
    // buffer.extend(std::iter::repeat(0_u8).take(data_length * 8));
    buffer.write_all(&std::iter::repeat(0_u8).take(data_length * 8).collect::<Vec<u8>>()).await?;

    Ok(())
}

async fn get_region_file_and_read_chunk(x: i32, z: i32) -> Result<Chunk> {
    let file_name = "import/r.0.0.mca".to_string();
    let exe_path = std::env::current_exe()?;
    // let file = std::fs::File::open(file_name)?;
    let parent = exe_path.parent().expect("Failed to get parent directory");
    let file = parent.join(file_name);
    let file = std::fs::File::open(file)?;
    let mut region = fastanvil::Region::from_stream(file)?;
    /*for chunk in region.iter() {
        let chunk = chunk?;
        if chunk.x == x as usize && chunk.z == z as usize {
            return Ok(chunk.data);
        }
    }*/
    let chunk = region.iter()
        .filter_map(|c| c.ok())
        .find(|c| c.x == x as usize && c.z == z as usize)
        .expect("Chunk not found");

    let chunk = Chunk::read_from_bytes(&mut Cursor::new(chunk.data))?;

    Ok(chunk)
}*/
use crate::utils::encoding::bitset::BitSet;
use crate::utils::encoding::varint::VarInt;
use crate::utils::error::Error;
use crate::utils::impls::type_impls::Encode;
use crate::utils::prelude::*;
use crate::world::chunkformat::{BlockStates, Chunk, Section};
use ferrumc_macros::Encode;
use nbt_lib::NBTDeserializeBytes;
use std::io::Cursor;
use tokio::io::AsyncWriteExt;

const SECTION_WIDTH: usize = 16;
const SECTION_HEIGHT: usize = 16;
const CHUNK_HEIGHT: usize = 256; // Adjust if your world height is different

#[derive(Encode)]
pub struct ChunkDataPacket {
    #[encode(default = VarInt::from(0x27))]
    pub packet_id: VarInt,
    pub chunk_x: i32,
    pub chunk_z: i32,
    #[encode(default = vec![0])] // empty heightmaps
    pub height_maps: Vec<u8>,
    #[encode(raw_bytes(prepend_length = true))]
    pub data: Vec<u8>,
    #[encode(default = VarInt::from(0))]
    pub number_of_block_entities: VarInt,
    pub sky_light_mask: BitSet,
    pub block_light_mask: BitSet,
    pub empty_sky_light_mask: BitSet,
    pub empty_block_light_mask: BitSet,
    #[encode(default = VarInt::from(0))]
    pub sky_light_array_count: VarInt,
    #[encode(default = VarInt::from(0))]
    pub block_light_array_count: VarInt,
}

impl ChunkDataPacket {
    pub async fn new(x: i32, z: i32) -> Result<Self> {
        let chunk = get_region_file_and_read_chunk(x, z).await?;

        let mut data = Cursor::new(Vec::new());
        let mut mask = BitSet::new(CHUNK_HEIGHT / SECTION_HEIGHT);

        // Write primary bit mask placeholder (we'll update it later)
        let mask_position = data.position();
        VarInt::from(0).encode(&mut data).await?;

        // Write heightmaps (empty for now)
        VarInt::from(0).encode(&mut data).await?;

        for y in 0..(CHUNK_HEIGHT / SECTION_HEIGHT) {
            if let Some(section) = chunk.sections.as_ref().and_then(|sections| sections.get(y)) {
                if !is_section_empty(section) {
                    mask.set(y);
                    write_chunk_section(section, &mut data).await?;
                }
            }
        }

        // Write biomes (using a placeholder value for now)
        for _ in 0..SECTION_WIDTH * SECTION_WIDTH {
            VarInt::from(127).encode(&mut data).await?; // Use 127 as a placeholder biome ID
        }

        // Go back and write the actual mask
        let end_position = data.position();
        data.set_position(mask_position);
        VarInt::from(mask.count_ones() as i32).encode(&mut data).await?;
        data.set_position(end_position);

        /*Ok(ChunkDataPacket {
            packet_id: VarInt::from(0x27),
            chunk_x: x,
            chunk_z: z,
            height_maps: vec![0], // Empty heightmaps
            data: data.into_inner(),
            number_of_block_entities: VarInt::from(0),
            sky_light_mask: BitSet::new(0),
            block_light_mask: BitSet::new(0),
            empty_sky_light_mask: BitSet::new(0),
            empty_block_light_mask: BitSet::new(0),
            sky_light_array_count: VarInt::from(0),
            block_light_array_count: VarInt::from(0),
        })*/
        Ok(ChunkDataPacket::new_auto(
            x,
            z,
            data.into_inner(),
            mask,
            BitSet::new(0),
            BitSet::new(0),
            BitSet::new(0),
        ))
    }
}

fn is_section_empty(section: &Section) -> bool {
    section.block_states.as_ref()
        .and_then(|bs| bs.data.as_ref())
        .map_or(true, |data| data.iter().all(|&x| x == 0))
}

async fn write_chunk_section(section: &Section, buffer: &mut Cursor<Vec<u8>>) -> Result<()> {
    // Write block count (assuming non-empty for simplicity)
    buffer.write_all(&(4096_u16).to_be_bytes()).await?;

    if let Some(block_states) = &section.block_states {
        write_block_states(block_states, buffer).await?;
    } else {
        write_empty_block_states(buffer).await?;
    }

    Ok(())
}

async fn write_block_states(block_states: &BlockStates, buffer: &mut Cursor<Vec<u8>>) -> Result<()> {
    let bits_per_block = 15;
    buffer.write_all(&[bits_per_block]).await?;

    // No palette for global palette
    VarInt::from(0).encode(buffer).await?;

    if let Some(data) = &block_states.data {
        VarInt::from(data.len() as i32).encode(buffer).await?;
        for &long in data {
            buffer.write_all(&long.to_be_bytes()).await?;
        }
    } else {
        VarInt::from(0).encode(buffer).await?;
    }

    Ok(())
}

async fn write_empty_block_states(buffer: &mut Cursor<Vec<u8>>) -> Result<()> {
    let bits_per_block = 15;
    buffer.write_all(&[bits_per_block]).await?;

    VarInt::from(0).encode(buffer).await?;

    let data_length = (SECTION_WIDTH * SECTION_WIDTH * SECTION_HEIGHT * 15) / 64;
    VarInt::from(data_length as i32).encode(buffer).await?;
    buffer.write_all(&vec![0; data_length * 8]).await?;

    Ok(())
}

async fn get_region_file_and_read_chunk(x: i32, z: i32) -> Result<Chunk> {
    let file_name = "import/r.0.0.mca".to_string();
    let exe_path = std::env::current_exe()?;
    let parent = exe_path.parent().expect("Failed to get parent directory");
    let file = parent.join(file_name);
    let file = std::fs::File::open(file)?;
    let mut region = fastanvil::Region::from_stream(file)?;

    let chunk = region.iter()
        .filter_map(|c| c.ok())
        .find(|c| c.x == x as usize && c.z == z as usize)
        .ok_or_else(|| Error::Generic(format!("Chunk not found at {}, {}", x, z)))?;

    let chunk = Chunk::read_from_bytes(&mut Cursor::new(chunk.data))?;

    Ok(chunk)
}
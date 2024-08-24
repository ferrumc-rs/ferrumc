use ferrumc_macros::Encode;
use nbt_lib::{NBTTag};
use crate::utils::encoding::bitset::BitSet;
use ferrumc_codec::network_types::varint::VarInt;
use ferrumc_codec::enc::Encode;
use tracing::debug;
use crate::world::chunkformat::{Heightmaps};
use crate::Result;
use crate::state::GlobalState;
use crate::utils::encoding::position::Position;
use crate::utils::error::Error;

#[derive(Encode)]
pub struct ChunkDataAndUpdateLight {
    #[encode(default=VarInt::from(0x24))]
    pub packet_id: VarInt,
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub heightmaps: Heightmaps,
    #[encode(raw_bytes(prepend_length = true))]
    pub data: Vec<u8>,
    pub block_entities_count: VarInt,
    pub block_entities: Vec<BlockEntity>,
    pub sky_light_mask: BitSet,
    pub block_light_mask: BitSet,
    pub empty_sky_light_mask: BitSet,
    pub empty_block_light_mask: BitSet,
    pub sky_light_array_count: VarInt,
    pub sky_light_arrays: Vec<LightArray>,
    pub block_light_array_count: VarInt,
    pub block_light_arrays: Vec<LightArray>,
}

#[derive(Encode)]
pub struct BlockEntity {
    pub packed_xz: u8,
    pub y: i16,
    pub type_id: VarInt,
    pub data: NBTTag,
}

#[derive(Encode, Clone)]
pub struct LightArray {
    #[encode(raw_bytes(prepend_length = true))]
    pub data: Vec<u8>,
}

impl ChunkDataAndUpdateLight {
    pub async fn new(state: GlobalState, x: i32, z: i32) -> Result<Self> {
        debug!("Sending chunk at {}, {}", x, z);

        let mut data = Vec::new();

        // Generate heightmaps for a solid stone chunk up to y=64
        let heightmaps = generate_heightmaps(64);

        // Encode 4 sections of solid stone (up to y=64)
        for _ in 0..4 {
            encode_stone_section(&mut data).await.expect("Failed to encode stone section");
        }

        // Simple light data (full bright)
        let light_array = vec![0xFF; 2048];

        let packet = ChunkDataAndUpdateLight {
            packet_id: VarInt::from(0x24),
            chunk_x: x,
            chunk_z: z,
            heightmaps,
            data,
            block_entities_count: VarInt::from(0),
            block_entities: Vec::new(),
            sky_light_mask: BitSet::from_iter(vec![0b1111]), // 4 sections
            block_light_mask: BitSet::from_iter(vec![0b1111]), // 4 sections
            empty_sky_light_mask: BitSet::empty(),
            empty_block_light_mask: BitSet::empty(),
            sky_light_array_count: VarInt::from(4),
            sky_light_arrays: vec![LightArray { data: light_array.clone() }; 4],
            block_light_array_count: VarInt::from(4),
            block_light_arrays: vec![LightArray { data: light_array }; 4],
        };

        debug!("Chunk data packet prepared");
        debug!("Data size: {}", packet.data.len());
        debug!("Sky light arrays total size: {}", packet.sky_light_arrays.iter().map(|a| a.data.len()).sum::<usize>());
        debug!("Block light arrays total size: {}", packet.block_light_arrays.iter().map(|a| a.data.len()).sum::<usize>());

        Ok(packet)
    }
}

async fn encode_stone_section(data: &mut Vec<u8>) -> Result<()> {
    // Non-air block count (16x16x16 = 4096)
    VarInt::from(4096).encode(data).await?;

    // Bits per block
    data.push(0);

    // Palette
    VarInt::from(1).encode(data).await?; // Palette length
    VarInt::from(1).encode(data).await?; // Stone block state ID

    // Block data (all zeros, since 0 in palette = stone)
    let block_data_length = 4096 * 4 / 64; // 4 bits per block, 64 bits per long
    VarInt::from(block_data_length as i32).encode(data).await?;
    data.extend(vec![0u8; block_data_length * 8]); // 8 bytes per long

    // Biomes (single biome for simplicity)
    VarInt::from(1).encode(data).await?; // Palette length
    VarInt::from(127).encode(data).await?; // Plains biome ID
    data.extend(vec![0u8; 64]); // 64 bytes for 4x4x4 biomes

    Ok(())
}
fn generate_heightmaps(height: i32) -> Heightmaps {
    let mut motion_blocking = vec![0i64; 37];

    for z in 0..16 {
        for x in 0..16 {
            let index = z * 16 + x;
            let long_index = index / 7;
            let bit_index = (index % 7) * 9;

            let value = height as i64 & 0x1FF; // Ensure we only use 9 bits
            motion_blocking[long_index] |= value << bit_index;

            // If the shift would overflow, put the remaining bits in the next long
            if bit_index > 55 { // 64 - 9 = 55
                let remaining_bits = bit_index + 9 - 64;
                motion_blocking[long_index + 1] |= value >> (9 - remaining_bits);
            }
        }
    }


    Heightmaps {
        motion_blocking_no_leaves: None,
        motion_blocking: Some(motion_blocking),
        ocean_floor: None,
        world_surface: None,
    }
}
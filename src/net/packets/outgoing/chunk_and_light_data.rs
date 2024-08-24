use ferrumc_macros::Encode;
use nbt_lib::{NBTTag};
use crate::utils::encoding::bitset::BitSet;
use ferrumc_codec::network_types::varint::VarInt;
use ferrumc_codec::enc::Encode;
use crate::world::chunkformat::{Heightmaps};
use crate::Result;
use crate::state::GlobalState;
use crate::utils::error::Error;

#[derive(Encode)]
pub struct ChunkDataAndUpdateLight<'a> {
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

#[derive(Encode)]
pub struct LightArray {
    pub length: VarInt,
    pub data: Vec<u8>,
}

impl<'a> ChunkDataAndUpdateLight<'a> {
    pub async fn new(state: GlobalState) -> Result<Self> {
        let x = 1;
        let z = 1;



        let chunk = state
            .database
            .get_chunk(x, z, "overworld")
            .await?
            .ok_or(Error::ChunkNotFound(x, z))?;

        let mut data = Vec::new();

        let heightmaps = chunk.heightmaps.as_ref().expect("Chunk heightmaps missing");

        // Simple light data (full bright)
        let light_array = vec![0xFF; 2048];

        encode_stone_section(&mut data).await.expect("Failed to encode stone section");


        let packet = ChunkDataAndUpdateLight {
            packet_id: VarInt::from(0x24),
            chunk_x: chunk.x_pos,
            chunk_z: chunk.z_pos,
            heightmaps: heightmaps.clone(),
            data,
            block_entities_count: VarInt::from(0),
            block_entities: Vec::new(),
            sky_light_mask: BitSet::from_iter(vec![1]), // Only one section
            block_light_mask: BitSet::from_iter(vec![1]), // Only one section
            empty_sky_light_mask: BitSet::empty(),
            empty_block_light_mask: BitSet::empty(),
            sky_light_array_count: VarInt::from(1),
            sky_light_arrays: vec![LightArray {
                length: VarInt::from(2048),
                data: light_array.clone(),
            }],
            block_light_array_count: VarInt::from(1),
            block_light_arrays: vec![LightArray {
                length: VarInt::from(2048),
                data: light_array,
            }],
        };

        Ok(packet)
    }
}

async fn encode_stone_section(data: &mut Vec<u8>) -> Result<()> {
    // Non-air block count (16x16x16 = 4096)
    // data.extend_from_slice(&VarInt::from(4096).e());
    VarInt::from(4096).encode(data).await?;

    // Bits per block (4 is enough for just stone)
    data.push(4);

    // Palette
    // Palette length
    VarInt::from(1).encode(data).await?;
    // Stone block state ID
    VarInt::from(1).encode(data).await?;

    // Block data (all zeros, since 0 in palette = stone)
    let block_data_length = 4096 * 4 / 64; // 4 bits per block, 64 bits per long
    VarInt::from(block_data_length as i32).encode(data).await?;
    data.extend(vec![0u8; block_data_length * 8]); // 8 bytes per long

    // Biomes (single biome for simplicity)
    // data.extend_from_slice(&VarInt::from(1).to_bytes()); // Palette length
    // data.extend_from_slice(&VarInt::from(1).to_bytes()); // Plains biome ID
    VarInt::from(1).encode(data).await?;
    VarInt::from(1).encode(data).await?;
    data.extend(vec![0u8; 64]); // 64 bytes for 16x16 biomes

    Ok(())
}
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::{Cursor, Write};
use bitreader::BitReader;
use tokio::io::AsyncWriteExt;
use tracing::warn;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::bitset::BitSet;
use ferrumc_world::chunk_format::Chunk;
use crate::errors::{ChunkError, NetError};

#[derive(NetEncode)]
pub struct BlockEntity {
    pub xz: u8,
    pub y: u16,
    pub entity_type: VarInt,
    pub nbt: Vec<u8>,
}

#[derive(NetEncode)]
#[packet(packet_id = 0x27)]
pub struct ChunkAndLightData {
    pub chunk_x: i32,
    pub chunk_z: i32,
    // The binary nbt data
    pub heightmaps: Vec<u8>,
    pub data: Vec<u8>,
    pub block_entities: LengthPrefixedVec<BlockEntity>,
    pub sky_light_mask: BitSet,
    pub block_light_mask: BitSet,
    pub empty_sky_light_mask: BitSet,
    pub empty_block_light_mask: BitSet,
    pub sky_light_arrays: LengthPrefixedVec<LengthPrefixedVec<u8>>,
    pub block_light_arrays: LengthPrefixedVec<LengthPrefixedVec<u8>>,
}

impl ChunkAndLightData {
    async fn from_chunk(chunk: &Chunk) -> Result<Self, NetError> {
        let mut data = Cursor::new(Vec::new());
        for section in &chunk.sections {
            data.write_u16(section.block_states.non_air_blocks).await?;
            
            let bits_per_block = section.block_states.bits_per_block;
            data.write_u8(bits_per_block).await?;
            // If bits_per_block is 0, the section is using the single-value palette format
            // If bits_per_block is greater than 0, the section is using the indirect palette format
            if bits_per_block > 0 {
                // Write the palette
                VarInt::new(section.block_states.palette.len() as i32).write_async(&mut data)?;
                for palette_entry in &section.block_states.palette {
                    palette_entry.write_async(&mut data)?;
                }
                
                // Write the data
                VarInt::new(section.block_states.data.len() as i32).write_async(&mut data)?;
                for data_entry in &section.block_states.data {
                    data_entry.write_async(&mut data)?;
                }
            } else {
                // The 0s for air blocks and bits_per_block are already written
                // Get the only palette entry
                match section.block_states.palette.get(0) {
                    Some(palette_entry) => {
                        palette_entry.write_async(&mut data)?;
                    }
                    // If there is no palette entry, write a 0 (air) and log a warning
                    None => {
                        VarInt::new(0).write_async(&mut data)?;
                        warn!("No palette entry found for section at {chunk.x}, {chunk.z}");
                    }
                }
                // Write the empty data section's length (0)
                VarInt::new(0).write_async(&mut data)?;
            }
        } 
        Ok(ChunkAndLightData {
            chunk_x: chunk.x,
            chunk_z: chunk.z,
            heightmaps: chunk.heightmaps.serialize_as_network(),
            data: data.into_inner(),
            block_entities: LengthPrefixedVec::new(),
            sky_light_mask: BitSet::new(),
            block_light_mask: BitSet::new(),
            empty_sky_light_mask: BitSet::new(),
            empty_block_light_mask: BitSet::new(),
            sky_light_arrays: LengthPrefixedVec::new(),
            block_light_arrays: LengthPrefixedVec::new(),
        })
    }
}
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::{Cursor, Write};
use std::ops::Not;
use tokio::io::AsyncWriteExt;
use tracing::{debug, warn};
use ferrumc_net_codec::net_types::bitset::BitSet;
use ferrumc_world::chunk_format::{Chunk, Heightmaps};
use crate::errors::{NetError};


const SECTIONS: usize = 24; // Number of sections, adjust for your Y range (-64 to 319)

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
    pub data: LengthPrefixedVec<u8>,
    pub block_entities: LengthPrefixedVec<BlockEntity>,
    pub sky_light_mask: BitSet,
    pub block_light_mask: BitSet,
    pub empty_sky_light_mask: BitSet,
    pub empty_block_light_mask: BitSet,
    pub sky_light_arrays: LengthPrefixedVec<LengthPrefixedVec<u8>>,
    pub block_light_arrays: LengthPrefixedVec<LengthPrefixedVec<u8>>,
}

impl ChunkAndLightData {
    
    pub async fn empty(chunk_x: i32, chunk_z: i32) -> Self {
        let sky_light_arrays = (0..SECTIONS).map(|_| LengthPrefixedVec::new(vec![0; 2048])).collect();
        let block_light_arrays = (0..SECTIONS).map(|_| LengthPrefixedVec::new(vec![0; 2048])).collect();
        let mut empty_sky_light_mask = BitSet::new(SECTIONS + 2);
        empty_sky_light_mask.set_all(false);
        let mut empty_block_light_mask = BitSet::new(SECTIONS + 2);
        empty_block_light_mask.set_all(false);
        ChunkAndLightData {
            chunk_x,
            chunk_z,
            heightmaps: Heightmaps::new().serialize_as_network(),
            data: LengthPrefixedVec::new(vec![0; SECTIONS * 10]),
            block_entities: LengthPrefixedVec::new(Vec::new()),
            sky_light_mask: BitSet::new(SECTIONS),
            block_light_mask: BitSet::new(SECTIONS),
            empty_sky_light_mask,
            empty_block_light_mask,
            sky_light_arrays: LengthPrefixedVec::new(sky_light_arrays),
            block_light_arrays: LengthPrefixedVec::new(block_light_arrays),
        }
    }
    
    
    pub async fn from_chunk(chunk: &Chunk) -> Result<Self, NetError> {
        let mut data = Cursor::new(Vec::new());
        let mut sky_light_data = Vec::new();
        let mut block_light_data = Vec::new();
        for section in &chunk.sections {
            let section_sky_light_data = if section.sky_light.len() != 2048 {
                warn!("Sky light data for section at {}, {} is not 2048 bytes long", chunk.x, chunk.z);
                vec![255; 2048]
            } else {
                section.sky_light.clone()
            };
            sky_light_data.push(section_sky_light_data);
            let section_block_light_data = if section.block_light.len() != 2048 {
                warn!("Block light data for section at {}, {} is not 2048 bytes long", chunk.x, chunk.z);
                vec![255; 2048]
            } else {
                section.block_light.clone()
            };
            block_light_data.push(section_block_light_data);


            data.write_u16(section.block_states.non_air_blocks).await?;
            
            let bits_per_block = section.block_states.bits_per_block;
            data.write_u8(bits_per_block).await?;
            // If bits_per_block is 0, the section is using the single-value palette format
            // If bits_per_block is greater than 0, the section is using the indirect palette format
            if bits_per_block > 0 && !section.block_states.palette.is_empty() && !section.block_states.data.is_empty() {
                // Write the palette
                VarInt::new(section.block_states.palette.len() as i32).write_async(&mut data).await?;
                for palette_entry in &section.block_states.palette {
                    palette_entry.write_async(&mut data).await?;
                }
                
                // Write the data
                VarInt::new(section.block_states.data.len() as i32).write_async(&mut data).await?;
                for data_entry in &section.block_states.data {
                    data.write_i64(*data_entry).await?;
                }
            } else {
                // The 0s for air blocks and bits_per_block are already written
                // Get the only palette entry
                match section.block_states.palette.first() {
                    Some(palette_entry) => {
                        palette_entry.write_async(&mut data).await?;
                    }
                    // If there is no palette entry, write a 0 (air) and log a warning
                    None => {
                        VarInt::new(0).write_async(&mut data).await?;
                        warn!("No palette entry found for section at {}, {}", chunk.x, chunk.z);
                    }
                }
                // Write the empty data section's length (0)
                VarInt::new(0).write_async(&mut data).await?;
            }
            // Empty biome data for now
            data.write_u8(0).await?;
            data.write_u8(0).await?;
            data.write_u8(0).await?;

        }
        let mut sky_light_mask = BitSet::new(SECTIONS + 2);
        let mut block_light_mask = BitSet::new(SECTIONS + 2);

        // Populate masks based on light data
        for (i, section) in chunk.sections.iter().enumerate() {
            if !section.sky_light.is_empty() && section.sky_light.len() == 2048 {
                sky_light_mask.set(i, true);
            }
            if !section.block_light.is_empty() && section.block_light.len() == 2048 {
                block_light_mask.set(i, true);
            }
        }

        // Invert masks to create empty masks
        let empty_sky_light_mask = sky_light_mask.clone().not();
        let empty_block_light_mask = block_light_mask.clone().not();

        // Align light arrays with masks
        let sky_light_arrays = chunk.sections
            .iter()
            .filter(|section| !section.sky_light.is_empty())
            .map(|section| LengthPrefixedVec::new(section.sky_light.clone()))
            .collect();

        let block_light_arrays = chunk.sections
            .iter()
            .filter(|section| !section.block_light.is_empty())
            .map(|section| LengthPrefixedVec::new(section.block_light.clone()))
            .collect();
        Ok(ChunkAndLightData {
            chunk_x: chunk.x,
            chunk_z: chunk.z,
            heightmaps: chunk.heightmaps.serialize_as_network(),
            data: LengthPrefixedVec::new(data.into_inner()),
            block_entities: LengthPrefixedVec::new(Vec::new()),
            sky_light_mask,
            block_light_mask,
            empty_sky_light_mask,
            empty_block_light_mask,
            sky_light_arrays: LengthPrefixedVec::new(sky_light_arrays),
            block_light_arrays: LengthPrefixedVec::new(block_light_arrays),
        })
    }
}
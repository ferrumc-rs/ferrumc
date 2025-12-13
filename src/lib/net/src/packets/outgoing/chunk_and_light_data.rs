use ferrumc_world::block_state_id::BlockStateId;
use crate::errors::NetError;
use byteorder::{BigEndian, WriteBytesExt};
use ferrumc_macros::{block, packet, NetEncode};
use ferrumc_net_codec::net_types::bitset::BitSet;
use ferrumc_net_codec::net_types::byte_array::ByteArray;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_world::chunk_format::{Chunk, PaletteType};
use ferrumc_world::pos::ChunkPos;
use std::io::Cursor;
use std::ops::Not;
use tracing::warn;

/// Number of sections in a chunk for the current world height (-64 to 320 = 384 blocks = 24 sections = 16 blocks per section)
const SECTIONS: usize = 24;

#[derive(NetEncode)]
pub struct BlockEntity {
    pub xz: u8,
    pub y: u16,
    pub entity_type: VarInt,
    pub nbt: Vec<u8>,
}

#[derive(NetEncode)]
pub struct NetHeightmap {
    pub id: VarInt,
    pub data: LengthPrefixedVec<i64>,
}

#[derive(NetEncode)]
#[packet(packet_id = "level_chunk_with_light", state = "play")]
pub struct ChunkAndLightData {
    pub chunk_x: i32,
    pub chunk_z: i32,
    // The binary nbt data
    pub heightmaps: LengthPrefixedVec<NetHeightmap>,
    pub data: ByteArray,
    pub block_entities: LengthPrefixedVec<BlockEntity>,
    pub sky_light_mask: BitSet,
    pub block_light_mask: BitSet,
    pub empty_sky_light_mask: BitSet,
    pub empty_block_light_mask: BitSet,
    pub sky_light_arrays: LengthPrefixedVec<ByteArray>,
    pub block_light_arrays: LengthPrefixedVec<ByteArray>,
}

impl ChunkAndLightData {
    /// Creates a flat chunk packet with a solid floor of a given height.
    ///
    /// # Arguments
    /// * `chunk_x` - The chunk X coordinate.
    /// * `chunk_z` - The chunk Z coordinate.
    /// * `floor_sections` - The number of sections from the bottom to fill with Bedrock.
    ///   (e.g., 4 sections = -64 to 0 if starting at -64).
    pub fn flat(chunk_x: i32, chunk_z: i32, floor_sections: usize) -> Result<Self, NetError> {
        let mut chunk_sections_data = Vec::with_capacity(SECTIONS * 8);

        // --- Block IDs (1.21 Vanilla) ---
        let air_id = block!("air");
        let bedrock_id = block!("bedrock");
        let biome_id = 1; // Plains (i think; cba checking the registry)

        for i in 0..SECTIONS {
            let is_solid = i < floor_sections;

            // 1. Block Count (u16 Big Endian)
            // 4096 if full, 0 if empty
            let block_count = if is_solid { 4096 } else { 0 };
            chunk_sections_data
                .write_u16::<BigEndian>(block_count)
                .map_err(|e| NetError::Misc(e.to_string()))?;

            // 2. Block States (Paletted Container)
            // Format: [BitsPerEntry(u8)] + [Palette(VarInt Array)] + [DataArray(Long Array)]

            // Bits Per Entry = 0 (Single Value Palette)
            chunk_sections_data
                .write_u8(0)
                .map_err(|e| NetError::Misc(e.to_string()))?;

            // Palette Value (VarInt)
            let block = if is_solid { bedrock_id } else { air_id };
            block.to_varint()
                .write(&mut chunk_sections_data)
                .map_err(|e| NetError::Misc(format!("VarInt write error: {:?}", e)))?;

            // 3. Biomes (Paletted Container)
            // Bits Per Entry = 0 (Single Value)
            chunk_sections_data
                .write_u8(0)
                .map_err(|e| NetError::Misc(e.to_string()))?;
            // Palette Value (VarInt)
            VarInt::new(biome_id)
                .write(&mut chunk_sections_data)
                .map_err(|e| NetError::Misc(format!("VarInt write error: {:?}", e)))?;
        }

        // --- Lighting Masks ---
        // Create masks indicating all sections are "empty" of light data
        // to prevent the client from waiting for light updates.
        let mut empty_sky = BitSet::new(SECTIONS + 2);
        let mut empty_block = BitSet::new(SECTIONS + 2);
        empty_sky.set_all(true);
        empty_block.set_all(true);

        Ok(ChunkAndLightData {
            chunk_x,
            chunk_z,
            // Empty heightmaps - client will default to min-height/recalculate
            heightmaps: LengthPrefixedVec::new(vec![]),
            data: ByteArray::new(chunk_sections_data),
            block_entities: LengthPrefixedVec::new(vec![]),
            // Light masks (empty implies all 0s, but we flag them as explicitly empty in the empty_* masks)
            sky_light_mask: BitSet::new(SECTIONS + 2),
            block_light_mask: BitSet::new(SECTIONS + 2),
            empty_sky_light_mask: empty_sky,
            empty_block_light_mask: empty_block,
            sky_light_arrays: LengthPrefixedVec::new(vec![]),
            block_light_arrays: LengthPrefixedVec::new(vec![]),
        })
    }

    pub fn empty(chunk_x: i32, chunk_z: i32) -> Self {
        // Reuse the flat generator with 0 solid sections for a completely empty chunk
        // It shouldn't fail because I said so
        Self::flat(chunk_x, chunk_z, 0).expect("this is a bug. empty chunk generation failed.")
    }

    pub fn from_chunk(pos: ChunkPos, chunk: &Chunk) -> Result<Self, NetError> {
        let mut raw_data = Cursor::new(Vec::new());
        let mut sky_light_data = Vec::new();
        let mut block_light_data = Vec::new();
        for section in &chunk.sections {
            let section_sky_light_data = if section.sky_light.len() != 2048 {
                warn!(
                    "Sky light data for section at {} is not 2048 bytes long",
                    pos
                );
                vec![255; 2048]
            } else {
                section.sky_light.clone()
            };
            sky_light_data.push(section_sky_light_data);
            let section_block_light_data = if section.block_light.len() != 2048 {
                warn!(
                    "Block light data for section at {} is not 2048 bytes long",
                    pos
                );
                vec![255; 2048]
            } else {
                section.block_light.clone()
            };
            block_light_data.push(section_block_light_data);

            raw_data.write_u16::<BigEndian>(section.block_states.non_air_blocks)?;

            match &section.block_states.block_data {
                PaletteType::Single(val) => {
                    // debug!("Single palette type: {:?}", (chunk.x, chunk.z));
                    raw_data.write_u8(0)?;
                    val.write(&mut raw_data)?;
                    // VarInt::new(0).write(&mut raw_data)?;
                }
                PaletteType::Indirect {
                    bits_per_block,
                    data,
                    palette,
                } => {
                    // debug!("Indirect palette type: {:?}", (chunk.x, chunk.z));
                    raw_data.write_u8(*bits_per_block)?;
                    VarInt::new(palette.len() as i32).write(&mut raw_data)?;
                    for palette_entry in palette {
                        palette_entry.write(&mut raw_data)?;
                    }
                    // VarInt::new(data.len() as i32).write(&mut raw_data)?;
                    for data_entry in data {
                        raw_data.write_i64::<BigEndian>(*data_entry)?;
                    }
                }
                PaletteType::Direct { .. } => {
                    todo!("Direct palette type")
                }
            }

            // Empty biome data for now
            raw_data.write_u8(0)?;
            // Forest biome id
            raw_data.write_u8(21)?;
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
        let sky_light_arrays = chunk
            .sections
            .iter()
            .filter(|section| !section.sky_light.is_empty())
            .map(|section| ByteArray::new(section.sky_light.clone()))
            .collect();

        let block_light_arrays = chunk
            .sections
            .iter()
            .filter(|section| !section.block_light.is_empty())
            .map(|section| ByteArray::new(section.block_light.clone()))
            .collect();
        let heightmaps = vec![
            NetHeightmap {
                id: VarInt::new(1), // Placeholder for heightmap ID
                data: LengthPrefixedVec::new(chunk.heightmaps.world_surface.clone()),
            },
            NetHeightmap {
                id: VarInt::new(4), // Placeholder for heightmap ID
                data: LengthPrefixedVec::new(chunk.heightmaps.motion_blocking.clone()),
            },
        ];

        Ok(ChunkAndLightData {
            chunk_x: pos.x(),
            chunk_z: pos.z(),
            heightmaps: LengthPrefixedVec::new(heightmaps),
            data: ByteArray::new(raw_data.into_inner()),
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

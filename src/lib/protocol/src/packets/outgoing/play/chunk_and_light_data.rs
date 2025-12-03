use crate::codec::net_types::bitset::BitSet;
use crate::codec::net_types::byte_array::ByteArray;
use crate::codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use crate::codec::net_types::var_int::VarInt;
use crate::error::ProtocolError;
use crate::ids;
use crate::types::nbt::RawNbt;
use ferrumc_core::world::chunk_format::{Chunk, PaletteType};
use ferrumc_macros::NBTSerialize;
use ferrumc_macros::{NetEncode, packet};

use byteorder::{BigEndian, WriteBytesExt};
use std::io::Cursor;

const SECTIONS: usize = 24; // Number of sections, adjust for your Y range (-64 to 319)

#[derive(NetEncode)]
pub struct BlockEntity {
    pub xz: u8,
    pub y: u16,
    pub entity_type: VarInt,
    pub nbt: Vec<u8>,
}

/// Internal helper struct to serialize Heightmaps to NBT.
/// This is specific to this packet's requirements.
#[derive(Debug, Clone, NBTSerialize)]
#[nbt(rename = "")] // Root compound
struct HeightmapsNbt {
    #[nbt(rename = "MOTION_BLOCKING")]
    pub motion_blocking: Vec<i64>,
    #[nbt(rename = "WORLD_SURFACE")]
    pub world_surface: Vec<i64>,
}

// --- The Main Packet ---

#[derive(NetEncode)]
#[packet(id = ids::PLAY_CLIENTBOUND_LEVEL_CHUNK_WITH_LIGHT, state = "play")]
pub struct ChunkAndLightData {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub heightmaps: RawNbt,
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
    pub fn empty(chunk_x: i32, chunk_z: i32) -> Self {
        // 1. Create Empty Light Arrays
        let sky_light_arrays = (0..SECTIONS)
            .map(|_| ByteArray::new(vec![0; 2048]))
            .collect();
        let block_light_arrays = (0..SECTIONS)
            .map(|_| ByteArray::new(vec![0; 2048]))
            .collect();

        // 2. Setup Empty Masks (All set to true indicating "Empty")
        let mut empty_sky_light_mask = BitSet::new(SECTIONS + 2);
        empty_sky_light_mask.set_all(true);
        let mut empty_block_light_mask = BitSet::new(SECTIONS + 2);
        empty_block_light_mask.set_all(true);

        // 3. Create Default Heightmaps (NBT)
        // Protocol requires 37 longs for these arrays, even if empty.
        let heightmaps_struct = HeightmapsNbt {
            motion_blocking: vec![0i64; 37],
            world_surface: vec![0i64; 37],
        };

        let heightmaps_bytes = heightmaps_struct.serialize_with_header();

        // 4. Construct Valid Chunk Data (Safer Generator)
        let mut raw_data = Cursor::new(Vec::new());
        for _ in 0..SECTIONS {
            raw_data.write_u16::<BigEndian>(0).unwrap(); // Non-air count
            raw_data.write_u8(0).unwrap(); // Block States (Single)
            VarInt::new(0).write(&mut raw_data).unwrap(); // Palette ID 0 (Air)
            VarInt::new(0).write(&mut raw_data).unwrap(); // Data Length 0

            raw_data.write_u8(0).unwrap(); // Biomes (Single)
            VarInt::new(1).write(&mut raw_data).unwrap(); // Palette ID 1 (Plains)
            VarInt::new(0).write(&mut raw_data).unwrap(); // Data Length 0
        }

        ChunkAndLightData {
            chunk_x,
            chunk_z,
            heightmaps: RawNbt(heightmaps_bytes),
            data: ByteArray::new(raw_data.into_inner()),
            block_entities: LengthPrefixedVec::new(Vec::new()),
            sky_light_mask: BitSet::new(SECTIONS),
            block_light_mask: BitSet::new(SECTIONS),
            empty_sky_light_mask,
            empty_block_light_mask,
            sky_light_arrays: LengthPrefixedVec::new(sky_light_arrays),
            block_light_arrays: LengthPrefixedVec::new(block_light_arrays),
        }
    }

    pub fn from_chunk(chunk: &Chunk) -> Result<Self, ProtocolError> {
        let mut raw_data = Cursor::new(Vec::new());

        // --- 1. Block Data & Biomes ---
        for section in &chunk.sections {
            // A. Non-air blocks count (u16)
            raw_data.write_u16::<BigEndian>(section.block_states.non_air_blocks)?;

            // B. Block States (Convert Core i32 -> Protocol VarInt)
            match &section.block_states.block_data {
                PaletteType::Single(val) => {
                    raw_data.write_u8(0)?; // Bits per block (0 for Single)
                    VarInt(*val).write(&mut raw_data)?; // Palette ID
                    VarInt(0).write(&mut raw_data)?; // Data Length (0)
                }
                PaletteType::Indirect {
                    bits_per_block,
                    data,
                    palette,
                } => {
                    raw_data.write_u8(*bits_per_block)?;

                    // Palette Length
                    VarInt(palette.len() as i32).write(&mut raw_data)?;

                    // Write Palette Entries (Convert i32 -> VarInt)
                    for &block_id in palette {
                        VarInt(block_id).write(&mut raw_data)?;
                    }

                    // Data Array Length (Number of Longs)
                    VarInt(data.len() as i32).write(&mut raw_data)?;

                    // Write Data (i64s)
                    for &long in data {
                        raw_data.write_i64::<BigEndian>(long)?;
                    }
                }
                PaletteType::Direct => {
                    // Direct palettes (15 bits+) don't have a palette header, just data.
                    todo!("Direct palette serialization not yet implemented");
                }
            }

            // C. Biomes
            // For now, we default to Single(Forest/Plains) if biome data isn't fully populated.
            // TODO: Hook up real biome reading from `section.biome_states`
            raw_data.write_u8(0)?; // 0 bits
            VarInt(21).write(&mut raw_data)?; // ID 21 = Forest
            VarInt(0).write(&mut raw_data)?; // Data Len 0
        }

        // --- 2. Light Data ---
        let mut sky_light_mask = BitSet::new(SECTIONS + 2);
        let mut block_light_mask = BitSet::new(SECTIONS + 2);
        let mut empty_sky_light_mask = BitSet::new(SECTIONS + 2);
        let mut empty_block_light_mask = BitSet::new(SECTIONS + 2);

        let mut sky_light_arrays_vec = Vec::new();
        let mut block_light_arrays_vec = Vec::new();

        for (i, section) in chunk.sections.iter().enumerate() {
            // Offset +1 because bit 0 is the section BELOW the world
            let mask_index = i + 1;

            if section.sky_light.len() == 2048 {
                sky_light_mask.set(mask_index, true);
                sky_light_arrays_vec.push(ByteArray::new(section.sky_light.clone()));
            } else {
                empty_sky_light_mask.set(mask_index, true);
            }

            if section.block_light.len() == 2048 {
                block_light_mask.set(mask_index, true);
                block_light_arrays_vec.push(ByteArray::new(section.block_light.clone()));
            } else {
                empty_block_light_mask.set(mask_index, true);
            }
        }

        // --- 3. Heightmaps (NBT) ---
        // Protocol requires exactly 37 longs. If data is missing/corrupt, pad it.
        let motion_blocking = if chunk.heightmaps.motion_blocking.len() >= 37 {
            chunk.heightmaps.motion_blocking.clone()
        } else {
            vec![0i64; 37]
        };

        let world_surface = if chunk.heightmaps.world_surface.len() >= 37 {
            chunk.heightmaps.world_surface.clone()
        } else {
            vec![0i64; 37]
        };

        // Construct the helper struct for NBT serialization
        let heightmaps_struct = HeightmapsNbt {
            motion_blocking,
            world_surface,
        };

        // Serialize using ferrumc-nbt (Standard "With Header" format for Chunks)
        let heightmaps_bytes = heightmaps_struct.serialize_with_header();

        // --- 4. Final Assembly ---
        Ok(ChunkAndLightData {
            chunk_x: chunk.x,
            chunk_z: chunk.z,
            heightmaps: RawNbt(heightmaps_bytes),
            data: ByteArray::new(raw_data.into_inner()),
            block_entities: LengthPrefixedVec::new(Vec::new()), // Empty for now
            sky_light_mask,
            block_light_mask,
            empty_sky_light_mask,
            empty_block_light_mask,
            sky_light_arrays: LengthPrefixedVec::new(sky_light_arrays_vec),
            block_light_arrays: LengthPrefixedVec::new(block_light_arrays_vec),
        })
    }
}

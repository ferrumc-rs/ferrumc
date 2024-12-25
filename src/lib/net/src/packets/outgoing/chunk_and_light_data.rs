use crate::errors::NetError;
use byteorder::{BigEndian, WriteBytesExt};
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::bitset::BitSet;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_world::chunk_format::{Chunk, Heightmaps};
use ferrumc_world::chunk_palette::{ChunkPalette, PaletteType};
use std::io::{Cursor, Write};
use std::ops::Not;
use tracing::warn;

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
    pub fn empty(chunk_x: i32, chunk_z: i32) -> Self {
        let sky_light_arrays = (0..SECTIONS)
            .map(|_| LengthPrefixedVec::new(vec![0; 2048]))
            .collect();
        let block_light_arrays = (0..SECTIONS)
            .map(|_| LengthPrefixedVec::new(vec![0; 2048]))
            .collect();
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

    pub fn from_chunk(chunk: &Chunk) -> Result<Self, NetError> {
        let palette = chunk.get_palette();
        let mut data: Cursor<Vec<u8>> = Cursor::new(vec![]);
        let mut sky_light_data = Vec::new();
        let mut block_light_data = Vec::new();
        for section in &chunk.sections {
            let section_sky_light_data = if section.sky_light.len() != 2048 {
                warn!(
                    "Sky light data for section at {}, {} is not 2048 bytes long",
                    chunk.x, chunk.z
                );
                vec![255; 2048]
            } else {
                section.sky_light.clone()
            };
            sky_light_data.push(section_sky_light_data);
            let section_block_light_data = if section.block_light.len() != 2048 {
                warn!(
                    "Block light data for section at {}, {} is not 2048 bytes long",
                    chunk.x, chunk.z
                );
                vec![255; 2048]
            } else {
                section.block_light.clone()
            };
            block_light_data.push(section_block_light_data);

            match palette {
                PaletteType::SingleValued(palette) => {
                    data.write_u16::<BigEndian>(palette.non_air_blocks)?;

                    // bits 
                    data.write_u8(palette.bits_per_block())?;

                    // palette
                    palette.value.write(&mut data)?;

                    // data array
                    palette.data.length.write(&mut data)?;
                    data.write(palette.data.data.as_slice())?;

                },
                PaletteType::Indirect(palette) => {
                    data.write_u16::<BigEndian>(palette.non_air_blocks)?;

                    // bits 
                    data.write_u8(palette.bits_per_block())?;

                    // palette
                    palette.palette.length.write(&mut data)?;
                    palette.palette.data.iter().for_each(|i| i.write(&mut data).unwrap());

                    // data array
                    palette.data.length.write(&mut data)?;
                    data.write(palette.data.data.as_slice())?;
                },
                PaletteType::Direct(palette) => {
                    data.write_u16::<BigEndian>(palette.non_air_blocks)?;

                    // bits 
                    data.write_u8(palette.bits_per_block())?;

                    // data array
                    palette.data.length.write(&mut data)?;
                    data.write(palette.data.data.as_slice())?;
                }
            }

            // Empty biome data for now
            data.write_u8(0)?;
            data.write_u8(0)?;
            data.write_u8(0)?;
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
            .map(|section| LengthPrefixedVec::new(section.sky_light.clone()))
            .collect();

        let block_light_arrays = chunk
            .sections
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

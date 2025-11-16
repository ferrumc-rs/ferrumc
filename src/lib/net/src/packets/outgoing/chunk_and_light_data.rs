use crate::errors::NetError;
use byteorder::{BigEndian, WriteBytesExt};
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::bitset::BitSet;
use ferrumc_net_codec::net_types::byte_array::ByteArray;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_world::chunk_format::{Chunk, PaletteType, Paletted};
use std::io::Cursor;
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
pub struct NetHeightmap {
    // Define the structure of your heightmaps here
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
    pub fn empty(chunk_x: i32, chunk_z: i32) -> Self {
        let sky_light_arrays = (0..SECTIONS)
            .map(|_| ByteArray::new(vec![0; 2048]))
            .collect();
        let block_light_arrays = (0..SECTIONS)
            .map(|_| ByteArray::new(vec![0; 2048]))
            .collect();
        let mut empty_sky_light_mask = BitSet::new(SECTIONS + 2);
        empty_sky_light_mask.set_all(false);
        let mut empty_block_light_mask = BitSet::new(SECTIONS + 2);
        empty_block_light_mask.set_all(false);
        ChunkAndLightData {
            chunk_x,
            chunk_z,
            heightmaps: LengthPrefixedVec::default(),
            data: ByteArray::new(vec![0; SECTIONS * 10]),
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
        let mut raw_data = Cursor::new(Vec::new());
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

            raw_data.write_u16::<BigEndian>(section.block_states.non_air_blocks)?;

            match &section.block_states.block_data {
                PaletteType::Empty => {
                    raw_data.write_u8(0)?;
                    VarInt::new(0).write(&mut raw_data)?;
                }
                // TODO: this dois not work, client drops connection on trying to get index.
                PaletteType::Paleted(paleted) => {
                    // TODO: check non-air single sections
                    match paleted.as_ref() {
                        Paletted::U4 { palette, data, .. } => {
                            raw_data.write_u8(4)?;
                            VarInt::new(16).write(&mut raw_data)?;
                            for &block in palette {
                                VarInt::from(block).write(&mut raw_data)?;
                            }
                            // TODO: pls pls pls let me use transmute i promise it's safe this is literally noop
                            // let data = unsafe { std::mem::transmute::<_, &[i64; 256]>(data) };
                            let data = {
                                let mut out = [0; 256];
                                let mut tmp = [0; 8];
                                for i in 0..256 {
                                    tmp.copy_from_slice(&data[i * 8..i * 8 + 8]);
                                    out[i] = i64::from_le_bytes(tmp);
                                }
                                out
                            };
                            for data_entry in data {
                                raw_data.write_i64::<BigEndian>(data_entry)?
                            }
                        }
                        Paletted::U8 { palette, data, .. } => {
                            raw_data.write_u8(8)?;
                            VarInt::new(256).write(&mut raw_data)?;
                            for &block in palette {
                                VarInt::from(block).write(&mut raw_data)?;
                            }
                            let data = {
                                let mut out = [0; 256];
                                let mut tmp = [0; 8];
                                for i in 0..256 {
                                    tmp.copy_from_slice(&data[i * 8..i * 8 + 8]);
                                    out[i] = i64::from_le_bytes(tmp);
                                }
                                out
                            };
                            for data_entry in data {
                                raw_data.write_i64::<BigEndian>(data_entry)?
                            }
                        }
                        Paletted::Direct { data } => todo!(),
                    }
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
            chunk_x: chunk.x,
            chunk_z: chunk.z,
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

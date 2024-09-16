use crate::state::GlobalState;
use crate::utils::encoding::bitset::BitSet;
use crate::utils::error::Error;
use crate::world::chunk_format::Heightmaps;
use crate::Result;
use ferrumc_codec::enc::NetEncode;
use ferrumc_codec::network_types::varint::VarInt;
use ferrumc_macros::NetEncode;
use nbt_lib::NBTTag;
use std::io::Cursor;
use tracing::warn;

const _SECTION_WIDTH: usize = 16;
const _SECTION_HEIGHT: usize = 16;

// Seperated light data from chunk data since clippy was complaining about the size of the struct
#[derive(NetEncode)]
pub struct ChunkDataAndUpdateLight {
/*    #[encode(default=VarInt::from(0x24))]
    pub packet_id: VarInt,
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub heightmaps: Heightmaps,
    #[encode(raw_bytes(prepend_length = true))]
    pub data: Vec<u8>,
    pub block_entities: Vec<BlockEntity>,
    pub light_data: LightData,*/
    #[encode(default=VarInt::from(0x24))]
    pub packet_id: VarInt,
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub heightmaps: Heightmaps,
    #[encode(raw_bytes(prepend_length = true))]
    pub data: Vec<u8>,
    pub block_entities_count: VarInt,
    pub block_entities: Vec<BlockEntity>,
    pub light_data: LightData,
}

#[derive(NetEncode)]
pub struct LightData {
    pub sky_light_mask: BitSet,
    pub block_light_mask: BitSet,
    pub empty_sky_light_mask: BitSet,
    pub empty_block_light_mask: BitSet,
    pub sky_light_array_count: VarInt,
    pub sky_light_arrays: Vec<LightArray>,
    pub block_light_array_count: VarInt,
    pub block_light_arrays: Vec<LightArray>,
}

#[derive(NetEncode)]
pub struct BlockEntity {
    pub packed_xz: u8,
    pub y: i16,
    pub type_id: VarInt,
    pub data: NBTTag,
}

#[derive(NetEncode, Clone)]
pub struct LightArray {
    #[encode(raw_bytes(prepend_length = true))]
    pub data: Vec<u8>,
}

impl ChunkDataAndUpdateLight {
    pub async fn new(state: GlobalState, chunk_x: i32, chunk_z: i32) -> Result<Self> {
        let chunk = state
            .database
            .get_chunk(chunk_x, chunk_z, "overworld".to_string())
            .await?
            .ok_or(Error::ChunkNotFound(chunk_x, chunk_z))?;

        // Serialize the chunk data
        let mut data = Cursor::new(Vec::new());

        if let Some(sections) = &chunk.sections {
            for section in sections {
                section.net_encode(&mut data).await?;
                serialize_biomes().await?.net_encode(&mut data).await?;
            }
        } else {
            return Err(Error::InvalidChunk(
                chunk.x_pos,
                chunk.z_pos,
                "Chunk is missing sections".to_string(),
            ));
        }

        // 24 is the number of sections in a chunk

        // -4 to 20
        const SECTIONS: usize = 24;

        // let sky_light_mask = BitSet::from_iter((0..SECTIONS + 2).map(|_| 1));
        // let block_light_mask = BitSet::from_iter((0..SECTIONS + 2).map(|_| 1));
        // let empty_sky_light_mask = BitSet::from_iter((0..SECTIONS + 2).map(|_| 0));
        // let empty_block_light_mask = BitSet::from_iter((0..SECTIONS + 2).map(|_| 0));

        let mut sky_light_mask = BitSet::new(SECTIONS + 2);
        sky_light_mask.set_all();
        let mut block_light_mask = BitSet::new(SECTIONS + 2);
        block_light_mask.set_all();
        let empty_sky_light_mask = BitSet::new(SECTIONS + 2);
        let empty_block_light_mask = BitSet::new(SECTIONS + 2);

        // Create light arrays
        let mut sky_light_arrays = Vec::new();
        let mut block_light_arrays = Vec::new();

        for section in chunk.sections.as_ref().unwrap() {
            sky_light_arrays.push(if let Some(sky_light) = &section.sky_light {
                LightArray {
                    data: sky_light.iter().take(2048).map(|&x| x as u8).collect(),
                }
            } else {
                LightArray {
                    data: vec![0; 2048],
                }
            });
            block_light_arrays.push(if let Some(block_light) = &section.block_light {
                LightArray {
                    data: block_light.iter().take(2048).map(|&x| x as u8).collect(),
                }
            } else {
                LightArray {
                    data: vec![0; 2048],
                }
            });
        }
        block_light_arrays.push(LightArray {
            data: vec![0; 2048],
        });
        sky_light_arrays.push(LightArray {
            data: vec![0; 2048],
        });
        block_light_arrays.push(LightArray {
            data: vec![0; 2048],
        });
        sky_light_arrays.push(LightArray {
            data: vec![0; 2048],
        });

        let heightmaps = chunk.heightmaps.unwrap_or_else(|| {
            warn!("Chunk is missing heightmaps, creating default heightmaps");
            Heightmaps {
                //motion_blocking_no_leaves: None,
                motion_blocking: Some(vec![i64::MAX; 37]),
                //ocean_floor: None,
                world_surface: Some(vec![i64::MAX; 37]),
            }
        });

        let res = ChunkDataAndUpdateLight {
            packet_id: VarInt::from(0x24),
            chunk_x,
            chunk_z,
            heightmaps,
            data: data.into_inner(),
            block_entities_count: VarInt::from(0),
            block_entities: Vec::new(),
            light_data: LightData {
                sky_light_mask,
                block_light_mask,
                empty_sky_light_mask,
                empty_block_light_mask,
                sky_light_array_count: VarInt::from(sky_light_arrays.len() as i32),
                sky_light_arrays,
                block_light_array_count: VarInt::from(block_light_arrays.len() as i32),
                block_light_arrays,
            },
        };
        Ok(res)
    }
}
/*
async fn serialize_block_states(block_states: &BlockStates) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    let bits_per_block = 15; // direct palette
    data.push(bits_per_block);

    // No palette serialization for direct palette

    // Serialize the block data
    let block_data = block_states.data.as_ref().unwrap();
    VarInt::from(block_data.len() as i32)
        .net_encode(&mut data)
        .await?;

    for long in block_data {
        long.net_encode(&mut data).await?;
    }

    Ok(data)
}*/
async fn serialize_biomes() -> Result<Vec<u8>> {
    let mut data: Vec<u8> = Vec::new();
    let bits_per_biome = 6;
    data.push(bits_per_biome);

    // Direct biome encoding, no palette
    let biome_data = vec![0u64; 64 * (bits_per_biome as usize) / 64]; // 64 biomes per section
    VarInt::from(biome_data.len() as i32)
        .net_encode(&mut data)
        .await?;

    for long in biome_data {
        long.net_encode(&mut data).await?;
    }

    Ok(data)
}
/*
fn create_basic_chunk(chunk_x: i32, chunk_z: i32) -> Chunk {
    let _rng = rand::thread_rng();
    let mut sections = Vec::with_capacity(24); // 24 sections for -64 to 320 world height
    for y in -4..=20 {
        // let possible_values = vec![1, 9, 131]; // stone, grass, oak log
        let chunk_data = vec![9; 16 * 16 * 16];

        /*if y > 1 && y < 6 {
            for x in 0..16 {
                for z in 0..16 {
                    let random_block = rng.gen_range(0..20_000);
                    // let random_value = possible_values[rng.gen_range(0..possible_values.len())];
                    chunk_data[x * 16 + z * 256] = random_block;
                }
            }
        }*/

        let block_states = create_block_states(&chunk_data, 15);

        let section = Section {
            block_states: Some(block_states.clone()),
            biomes: Some(Biomes {
                palette: vec!["minecraft:plains".to_string()],
            }),
            y: y as i8,
            block_light: Some(vec![0xf; 2048]),
            sky_light: Some(vec![0xf; 2048]),
        };
        sections.push(section);
    }

    // Set heightmap to the top of the world (320 + 1)
    let heightmap = vec![321i64; 256];

    Chunk {
        dimension: Some("overworld".to_string()),
        status: "full".to_string(),
        data_version: 3465,
        heightmaps: Some(Heightmaps {
            // motion_blocking_no_leaves: None,
            motion_blocking: Some(heightmap.clone()),
            // ocean_floor: None,
            world_surface: Some(heightmap),
        }),
        is_light_on: Some(1),
        inhabited_time: Some(0),
        y_pos: -4,
        x_pos: chunk_x,
        z_pos: chunk_z,
        structures: Some(Structures {
            starts: Starts {},
            references: References {},
        }),
        last_update: Some(0),
        sections: Some(sections),
    }
}
*/
/*fn create_block_states(chunk_data: &[u32], bits_per_entry: u8) -> BlockStates {
    let packed_data = pack_entries(chunk_data, bits_per_entry);

    BlockStates {
        non_air_blocks: None,
        bits_per_block: None,
        data: Some(packed_data),
        palette: if bits_per_entry < 15 {
            Some(Vec::new()) // We'll need to populate this for indirect encoding
        } else {
            None // Direct encoding, no palette
        },
        net_palette: None,
        // default: None,
    }
}*//*
fn pack_entries(entries: &[u32], bits_per_entry: u8) -> Vec<i64> {
    let entries_per_long = 64 / bits_per_entry as usize;
    let mask = (1u64 << bits_per_entry) - 1;
    let total_longs = (entries.len() + entries_per_long - 1) / entries_per_long;

    let mut packed_data = Vec::with_capacity(total_longs);
    let mut current_long = 0u64;
    let mut entries_in_long = 0;

    for &entry in entries {
        current_long |= (entry as u64 & mask) << (bits_per_entry as u64 * entries_in_long as u64);
        entries_in_long += 1;

        if entries_in_long == entries_per_long {
            packed_data.push(current_long as i64);
            current_long = 0;
            entries_in_long = 0;
        }
    }

    // Handle padding for the last long if necessary
    if entries_in_long > 0 {
        packed_data.push(current_long as i64);
    }

    packed_data
}*/

// fn get_block_state_id(block_name: &str) -> i32 {
//     // This should be replaced with a proper block state registry lookup
//     match block_name {
//         "minecraft:air" => 0,
//         "minecraft:stone" => 1,
//         "minecraft:grass_block" => 9,
//         "minecraft:oak_log" => 131,
//         _ => 0,
//     }
// }

// fn get_biome_id(biome: &str) -> i32 {
//     // This should be replaced with a proper biome registry lookup
//     match biome {
//         "minecraft:plains" => 127,
//         _ => 0,
//     }
// }

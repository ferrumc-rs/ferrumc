use crate::state::GlobalState;
use crate::utils::encoding::bitset::BitSet;
use crate::utils::error::Error;
use crate::world::chunkformat::{Biomes, BlockStates, Chunk, Heightmaps, Palette, Properties, References, Section, Starts, Structures};
use crate::Result;
use ferrumc_codec::enc::Encode;
use ferrumc_codec::network_types::varint::VarInt;
use ferrumc_macros::Encode;
use nbt_lib::NBTTag;

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
    pub async fn new(_state: GlobalState, chunk_x: i32, chunk_z: i32) -> Result<Self> {
        let chunk = create_basic_chunk(chunk_x, chunk_z);

        // Serialize the chunk data
        let mut data = Vec::new();
        for section in chunk.sections.as_ref().unwrap() {
            let Some(block_states) = &section.block_states else {
                return Err(Error::MissingBlockStates)
            };
            // data.extend(serialize_block_states(block_states)?);
            let block_states_data = serialize_block_states(block_states).await?;
            data.extend(block_states_data);

            let Some(biomes) = &section.biomes else {
                return Err(Error::MissingBlockStates)
            };

            let biomes_data = serialize_biomes(biomes).await?;
            data.extend(biomes_data);
        }

        let sky_light_mask = BitSet::from_iter((0..24).map(|_| 1));
        let block_light_mask = BitSet::from_iter((0..24).map(|_| 1));
        let empty_sky_light_mask = BitSet::empty();
        let empty_block_light_mask = BitSet::empty();

        // Create light arrays
        let sky_light_arrays = vec![LightArray { data: vec![0xFF; 2048] }; 24];
        let block_light_arrays = vec![LightArray { data: vec![0xFF; 2048] }; 24];

        Ok(ChunkDataAndUpdateLight {
            packet_id: VarInt::from(0x24),
            chunk_x,
            chunk_z,
            heightmaps: chunk.heightmaps.unwrap(),
            data,
            block_entities_count: VarInt::from(0),
            block_entities: Vec::new(),
            sky_light_mask,
            block_light_mask,
            empty_sky_light_mask,
            empty_block_light_mask,
            sky_light_array_count: VarInt::from(24),
            sky_light_arrays,
            block_light_array_count: VarInt::from(24),
            block_light_arrays,
        })
    }
}

async fn serialize_block_states(block_states: &BlockStates) -> Result<Vec<u8>> {
    let mut data = Vec::new();

    let palettes = block_states.palette.as_ref().ok_or(Error::MissingBlockStates)?;
    let palette_len = palettes.len();
    let bits_per_block = (palette_len as f32).log2().ceil() as u8;

    data.push(bits_per_block);

    // Serialize palette
    VarInt::from(palette_len as i32).encode(&mut data).await?;
    for palette_entry in palettes {
        // data.extend(palette_entry.)
        let block_state_id = get_block_state_id(&palette_entry.name);
        VarInt::from(block_state_id).encode(&mut data).await?;
    }

    // Serialize the block data
    let block_data = block_states.data.as_ref().unwrap();
    VarInt::from(block_data.len() as i32).encode(&mut data).await?;
    for long in block_data {
        // data.extend(long.to_be_bytes());
        long.encode(&mut data).await?;
    }

    Ok(data)
}
async fn serialize_biomes(biomes: &Biomes) -> Result<Vec<u8>> {
    let mut data = Vec::new();

    let palette_len = biomes.palette.len();
    let bits_per_biome = (palette_len as f32).log2().ceil().max(1.0) as u8;

    data.push(bits_per_biome);

    // Serialize palette
    VarInt::from(palette_len as i32).encode(&mut data).await?;
    for palette_entry in &biomes.palette {
        let biome_id = get_biome_id(palette_entry);
        VarInt::from(biome_id).encode(&mut data).await?;
    }

    // Set all biomes to the first biome in the palette (For simplicity)
    let biome_data = vec![0u64; 1];
    VarInt::from(biome_data.len() as i32).encode(&mut data).await?;
    for long in &biome_data {
        long.encode(&mut data).await?;
    }

    Ok(data)
}

fn create_basic_chunk(chunk_x: i32, chunk_z: i32) -> Chunk {
    let air_palette = Palette {
        name: "minecraft:air".to_string(),
        properties: None,
    };
    let stone_palette = Palette {
        name: "minecraft:stone".to_string(),
        properties: None,
    };
    let dirt_palette = Palette {
        name: "minecraft:dirt".to_string(),
        properties: None,
    };
    let grass_palette = Palette {
        name: "minecraft:grass_block".to_string(),
        properties: Some(Properties {
            snowy: Some("false".to_string()),
            ..Default::default()
        }),
    };

    // Create block states for a single section
    let block_states = BlockStates {
        // data: Some(vec![1]),
        data: Some(vec![
            /*            0x3333333333333333, 0x3333333333333333, // Grass layer
                        0x2222222222222222, 0x2222222222222222, // Dirt layer
                        0x1111111111111111, 0x1111111111111111, // Stone layers
                        0x1111111111111111, 0x1111111111111111,*/
            8; 8
        ]),
        palette: Some(vec![
            air_palette,
            stone_palette,
            // dirt_palette,
            grass_palette,
        ]),
    };

    let mut sections = Vec::with_capacity(24); // 24 sections for -64 to 320 world height
    for y in -4..20 {
        let section = Section {
            block_states: Some(block_states.clone()),
            biomes: Some(Biomes {
                palette: vec!["minecraft:plains".to_string()]
            }),
            y: y as i8, // Bottom most section
            block_light: Some(vec![0xf; 2048]), // Full bright (0-15)
            sky_light: Some(vec![0xf; 2048]), // Full bright (0-15)
        };
        sections.push(section);
    }

    let mut heightmap = vec![1i64; 37];


    Chunk {
        status: "full".to_string(),
        data_version: 3465,
        heightmaps: Some(Heightmaps {
            motion_blocking_no_leaves: None,
            motion_blocking: Some(heightmap.clone()),
            ocean_floor: None,
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


fn get_block_state_id(block_name: &str) -> i32 {
    // This should be replaced with a proper block state registry lookup
    match block_name {
        "minecraft:air" => 0,
        "minecraft:stone" => 1,
        "minecraft:dirt" => 3,
        "minecraft:grass_block" => 8,
        _ => 0,
    }
}

fn get_biome_id(biome: &str) -> i32 {
    // This should be replaced with a proper biome registry lookup
    match biome {
        "minecraft:plains" => 1,
        _ => 0,
    }
}
use crate::test_de_data::{create_test_player, Player};
use crate::test_simd_de_data::MinecraftChunk;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nbt_lib::{
    read_tag, Deserialize, NBTDeserialize, NBTDeserializeBytes, NBTSerialize, Serialize,
};
use std::io::{Cursor, Read, Write};

mod test_de_data {
    use super::*;
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[nbt(is_root)]
    #[nbt(rename = "Player")]
    pub struct Player {
        name: String,
        age: i32,
        health: f32,
        mana: f32,
        experience: i64,
        is_admin: bool,
        position: Position,
        inventory: Vec<Item>,
        skills: Vec<Skill>,
        achievements: Achievements,
        stats: Stats,
        settings: Settings,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Position {
        x: f64,
        y: f64,
        z: f64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Item {
        id: String,
        count: i32,
        durability: i16,
        enchantments: Vec<Enchantment>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Enchantment {
        id: String,
        level: i16,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Skill {
        name: String,
        level: i32,
        experience: f32,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Achievements {
        total_unlocked: i32,
        list: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Stats {
        playtime: i64,
        mobs_killed: i32,
        distance_traveled: f64,
        items_crafted: i32,
        blocks_broken: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Settings {
        render_distance: i32,
        difficulty: String,
        sound_volume: f32,
        music_volume: f32,
        language: String,
    }

    pub fn create_test_player() -> Player {
        Player {
            name: "SuperPlayer123".to_string(),
            age: 25,
            health: 95.5,
            mana: 150.0,
            experience: 1_000_000,
            is_admin: false,
            position: Position {
                x: 156.7,
                y: 64.0,
                z: -892.3,
            },
            inventory: vec![
                Item {
                    id: "diamond_sword".to_string(),
                    count: 1,
                    durability: 1500,
                    enchantments: vec![
                        Enchantment {
                            id: "sharpness".to_string(),
                            level: 5,
                        },
                        Enchantment {
                            id: "unbreaking".to_string(),
                            level: 3,
                        },
                    ],
                },
                /*Item {
                    id: "golden_apple".to_string(),
                    count: 64,
                    durability: 0,
                    enchantments: vec![],
                },*/
                Item {
                    id: "netherite_pickaxe".to_string(),
                    count: 1,
                    durability: 2000,
                    enchantments: vec![
                        Enchantment {
                            id: "efficiency".to_string(),
                            level: 4,
                        },
                        Enchantment {
                            id: "fortune".to_string(),
                            level: 3,
                        },
                    ],
                },
            ],
            skills: vec![
                Skill {
                    name: "Mining".to_string(),
                    level: 75,
                    experience: 95000.5,
                },
                Skill {
                    name: "Combat".to_string(),
                    level: 60,
                    experience: 75000.0,
                },
                Skill {
                    name: "Farming".to_string(),
                    level: 45,
                    experience: 50000.25,
                },
            ],
            achievements: Achievements {
                total_unlocked: 150,
                list: vec![
                    "Open Inventory".to_string(),
                    "Kill the Ender Dragon".to_string(),
                    "Reach the End".to_string(),
                    "Defeat the Wither".to_string(),
                    "Enchant an Item".to_string(),
                ],
            },
            stats: Stats {
                playtime: 1_000_000,
                mobs_killed: 10_000,
                distance_traveled: 1_000_000.5,
                items_crafted: 50_000,
                blocks_broken: 500_000,
            },
            settings: Settings {
                render_distance: 16,
                difficulty: "Hard".to_string(),
                sound_volume: 0.8,
                music_volume: 0.6,
                language: "en_US".to_string(),
            },
        }
    }
}
mod test_simd_de_data {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    #[nbt(rename = "Level")]
    #[nbt(is_root)]
    pub struct MinecraftChunk {
        #[nbt(rename = "xPos")]
        pub x_pos: i32,
        #[nbt(rename = "zPos")]
        pub z_pos: i32,
        #[nbt(rename = "LastUpdate")]
        pub last_update: i64,
        #[nbt(rename = "TerrainPopulated")]
        pub terrain_populated: i8,
        #[nbt(rename = "LightPopulated")]
        pub light_populated: i8,
        #[nbt(rename = "InhabitedTime")]
        pub inhabited_time: i64,
        #[nbt(rename = "Biomes")]
        pub biomes: Vec<i8>,
        #[nbt(rename = "HeightMap")]
        pub height_map: Vec<i32>,
        #[nbt(rename = "Sections")]
        pub sections: Vec<ChunkSection>,
        #[nbt(rename = "Entities")]
        pub entities: Vec<Entity>,
        #[nbt(rename = "TileEntities")]
        pub tile_entities: Vec<TileEntity>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ChunkSection {
        #[nbt(rename = "Y")]
        pub y: i8,
        #[nbt(rename = "BlockLight")]
        pub block_light: Vec<i8>,
        #[nbt(rename = "SkyLight")]
        pub sky_light: Vec<i8>,
        #[nbt(rename = "Blocks")]
        pub blocks: Vec<i8>,
        #[nbt(rename = "Data")]
        pub data: Vec<i8>,
        #[nbt(rename = "BlockStates")]
        pub block_states: Vec<i64>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Entity {
        #[nbt(rename = "id")]
        pub id: String,
        #[nbt(rename = "Pos")]
        pub position: Vec<f64>,
        #[nbt(rename = "Motion")]
        pub motion: Vec<f64>,
        #[nbt(rename = "Rotation")]
        pub rotation: Vec<f32>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TileEntity {
        #[nbt(rename = "id")]
        pub id: String,
        #[nbt(rename = "x")]
        pub x: i32,
        #[nbt(rename = "y")]
        pub y: i32,
        #[nbt(rename = "z")]
        pub z: i32,
    }

    impl MinecraftChunk {
        pub fn create_test_instance() -> Self {
            MinecraftChunk {
                x_pos: 0,
                z_pos: 0,
                last_update: 1234567890,
                terrain_populated: 1,
                light_populated: 1,
                inhabited_time: 9876543210,
                biomes: vec![1; 256],      // 16x16 biome data
                height_map: vec![64; 256], // 16x16 height map
                sections: vec![ChunkSection {
                    y: 0,
                    block_light: vec![0; 2048], // 16x16x16 / 2 (4 bits per block)
                    sky_light: vec![15; 2048],  // 16x16x16 / 2 (4 bits per block)
                    blocks: vec![1; 4096],      // 16x16x16 block IDs
                    data: vec![0; 2048],        // 16x16x16 / 2 (4 bits per block)
                    block_states: vec![0; 256], // Compressed block state data
                }],
                entities: vec![Entity {
                    id: "minecraft:pig".to_string(),
                    position: vec![0.5, 65.0, 0.5],
                    motion: vec![0.0, 0.0, 0.0],
                    rotation: vec![0.0, 0.0],
                }],
                tile_entities: vec![TileEntity {
                    id: "minecraft:chest".to_string(),
                    x: 0,
                    y: 64,
                    z: 0,
                }],
            }
        }
    }
}

fn benchmark_serialization(c: &mut Criterion) {
    let world = MinecraftChunk::create_test_instance();

    c.bench_function("serialize", |b| {
        b.iter(|| {
            let mut buffer = Vec::with_capacity(10240);
            black_box(world.serialize(&mut buffer)).unwrap();
        })
    });
}

/*fn benchmark_deserialization(c: &mut Criterion) {
    let player = create_test_player();
    let mut buffer = Vec::new();
    player.serialize(&mut buffer).unwrap();

    std::fs::write(".etc/test_player.nbt", &buffer).unwrap();

    c.bench_function("deserialize", |b| b.iter(|| {
        let cursor = Cursor::new(buffer.clone());
        let nbt_data = black_box(read_tag(&mut cursor.clone())).unwrap();
        let _deserialized_player: Player = black_box(Player::read_from(nbt_data)).unwrap();
    }));
}
*/

fn get_nbt_buffer() -> Vec<u8> {
    /*let mut buffer = std::fs::read(".etc/TheAIguy_.nbt").unwrap();

    // decompress gzip
    if buffer[0] == 0x1F && buffer[1] == 0x8B && buffer[2] == 0x08 {
        println!("Decompressing gzip...");
        let mut decoder = flate2::read::GzDecoder::new(&buffer[..]);
        let mut decoded = Vec::new();
        decoder.read_to_end(&mut decoded).unwrap();
        return decoded;
    }

    buffer*/

    let data = MinecraftChunk::create_test_instance();

    let mut buffer = Vec::with_capacity(2048);
    data.serialize(&mut buffer).unwrap();

    buffer
}

fn benchmark_raw_deserialization(c: &mut Criterion) {
    let buffer = get_nbt_buffer();
    let mut cursor = Cursor::new(buffer);

    c.bench_function("world_chunk_deser_raw", |b| {
        b.iter(|| {
            let nbt = read_tag(&mut cursor).unwrap();

            black_box(nbt);

            cursor.set_position(0);
        })
    });
}
fn benchmark_simdnbt_deserialization(c: &mut Criterion) {
    let buffer = get_nbt_buffer();

    let buffer = buffer.as_slice();

    let mut cursor = Cursor::new(buffer);

    c.bench_function("aiguynbt_deser_simdnbt", |b| {
        b.iter(|| {
            let data = simdnbt::borrow::read(&mut cursor).unwrap().unwrap();

            black_box(data);
        })
    });
}

criterion_group!(benches, benchmark_serialization);
criterion_main!(benches);

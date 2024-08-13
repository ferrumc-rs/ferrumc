use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nbt_lib::{read_tag, Deserialize, NBTDeserialize, NBTSerialize, Serialize};
use r#struct::{create_test_player, Player};
use std::io::Cursor;

mod r#struct {
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

fn benchmark_serialization(c: &mut Criterion) {
    let player = create_test_player();

    c.bench_function("serialize", |b| {
        b.iter(|| {
            let mut buffer = Vec::with_capacity(2048);
            black_box(player.serialize(&mut buffer)).unwrap();
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
fn benchmark_raw_deserialization(c: &mut Criterion) {
    let player = create_test_player();
    let mut buffer = Vec::new();
    player.serialize(&mut buffer).unwrap();

    std::fs::write(".etc/test_player.nbt", &buffer).unwrap();

    c.bench_function("raw_deserialize", |b| {
        b.iter(|| {
            let cursor = black_box(Cursor::new(buffer.clone()));
            let nbt_data = black_box(read_tag(&mut cursor.clone())).unwrap();
            black_box(nbt_data);
        })
    });
}
fn benchmark_simdnbt_deserialization(c: &mut Criterion) {
    let player = create_test_player();
    let mut buffer = Vec::new();
    player.serialize(&mut buffer).unwrap();

    std::fs::write(".etc/test_player.nbt", &buffer).unwrap();

    let data = buffer.clone();
    let data = data.as_slice();
    let mut cursor = Cursor::new(data);

    c.bench_function("simdnbt_deserialize", |b| {
        b.iter(|| {
            let nbt_data = black_box(simdnbt::borrow::read(&mut cursor.clone())).unwrap();
            black_box(nbt_data);
        })
    });
}

criterion_group!(benches, benchmark_simdnbt_deserialization);
criterion_main!(benches);

use crate::tests::nbt_de::test_de_data::Player;
use nbt_lib::{NBTDeserialize, NBTDeserializeBytes, NBTSerialize};
use std::io::Cursor;

pub mod test_de_data {
    use super::*;

    #[derive(NBTSerialize, NBTDeserialize, Debug, Clone)]
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

    #[derive(NBTSerialize, NBTDeserialize, Debug, Clone)]
    struct Position {
        x: f64,
        y: f64,
        z: f64,
    }

    #[derive(NBTSerialize, NBTDeserialize, Debug, Clone)]
    struct Item {
        id: String,
        count: i32,
        durability: i16,
        enchantments: Vec<Enchantment>,
    }

    #[derive(NBTSerialize, NBTDeserialize, Debug, Clone)]
    struct Enchantment {
        id: String,
        level: i16,
    }

    #[derive(NBTSerialize, NBTDeserialize, Debug, Clone)]
    struct Skill {
        name: String,
        level: i32,
        experience: f32,
    }

    #[derive(NBTSerialize, NBTDeserialize, Debug, Clone)]
    struct Achievements {
        total_unlocked: i32,
        list: Vec<String>,
    }

    #[derive(NBTSerialize, NBTDeserialize, Debug, Clone)]
    struct Stats {
        playtime: i64,
        mobs_killed: i32,
        distance_traveled: f64,
        items_crafted: i32,
        blocks_broken: i64,
    }

    #[derive(NBTSerialize, NBTDeserialize, Debug, Clone)]
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

#[test]
fn try_read() {
    let player = test_de_data::create_test_player();
    let mut buffer = Vec::new();
    player.nbt_serialize(&mut buffer).unwrap();

    let mut cursor = std::io::Cursor::new(buffer);
    let nbt_data = nbt_lib::read_tag(&mut cursor).unwrap();
    let deserialized_player = Player::read_from(nbt_data).unwrap();
    println!("{:#?}", deserialized_player);
}
mod alguy_struct {
    use std::collections::HashMap;

    use nbt_lib::{NBTDeserialize, NBTSerialize};

    #[derive(Debug, NBTSerialize, NBTDeserialize)]
    #[nbt(rename = "")] // root tag
    #[nbt(is_root)]
    pub struct PlayerData {
        #[nbt(rename = "Pos")]
        pos: Vec<f64>,
        #[nbt(rename = "SelectedItemSlot")]
        selected_item_slot: i32,
        #[nbt(rename = "HurtByTimestamp")]
        hurt_by_timestamp: i32,
        #[nbt(rename = "foodSaturationLevel")]
        food_saturation_level: f32,
        #[nbt(rename = "EnderItems")]
        ender_items: Vec<Item>,
        #[nbt(rename = "DataVersion")]
        data_version: i32,
        #[nbt(rename = "XpTotal")]
        xp_total: i32,
        abilities: Abilities,
        #[nbt(rename = "Air")]
        air: i16,
        #[nbt(rename = "XpSeed")]
        xp_seed: i32,
        #[nbt(rename = "SleepTimer")]
        sleep_timer: i16,
        #[nbt(rename = "playerGameType")]
        player_game_type: i32,
        warden_spawn_tracker: WardenSpawnTracker,
        #[nbt(rename = "PortalCooldown")]
        portal_cooldown: i32,
        #[nbt(rename = "Health")]
        health: f32,
        #[nbt(rename = "Invulnerable")]
        invulnerable: bool,
        #[nbt(rename = "Rotation")]
        rotation: Vec<f32>,
        #[nbt(rename = "UUID")]
        uuid: Vec<i32>,
        #[nbt(rename = "recipeBook")]
        recipe_book: RecipeBook,
        #[nbt(rename = "foodTickTimer")]
        food_tick_timer: i32,
        #[nbt(rename = "HurtTime")]
        hurt_time: i16,
        #[nbt(rename = "FallFlying")]
        fall_flying: bool,
        #[nbt(rename = "DeathTime")]
        death_time: i16,
        #[nbt(rename = "Brain")]
        brain: Brain,
        #[nbt(rename = "Dimension")]
        dimension: String,
        #[nbt(rename = "AbsorptionAmount")]
        absorption_amount: f32,
        #[nbt(rename = "foodLevel")]
        food_level: i32,
        #[nbt(rename = "Motion")]
        motion: Vec<f64>,
        #[nbt(rename = "XpP")]
        xp_p: f32,
        #[nbt(rename = "Attributes")]
        attributes: Vec<Attribute>,
        #[nbt(rename = "Score")]
        score: i32,
        #[nbt(rename = "BalmData")]
        balm_data: HashMap<String, String>,
        #[nbt(rename = "FallDistance")]
        fall_distance: f32,
        #[nbt(rename = "OnGround")]
        on_ground: bool,
        #[nbt(rename = "XpLevel")]
        xp_level: i32,
        #[nbt(rename = "Fire")]
        fire: i16,
        #[nbt(rename = "foodExhaustionLevel")]
        food_exhaustion_level: f32,
        #[nbt(rename = "seenCredits")]
        seen_credits: bool,
        #[nbt(rename = "Inventory")]
        inventory: Vec<Item>,
    }

    #[derive(Debug, NBTSerialize, NBTDeserialize)]
    struct Item {
        #[nbt(rename = "Slot")]
        slot: i8,
        id: String,
        #[nbt(rename = "Count")]
        count: i8,
        tag: Option<ItemTag>,
    }

    #[derive(Debug, NBTSerialize, NBTDeserialize)]
    struct ItemTag {
        display: Option<Display>,
        #[nbt(rename = "Enchantments")]
        enchantments: Option<Vec<Enchantment>>,
        #[nbt(rename = "RepairCost")]
        repair_cost: Option<i32>,
        #[nbt(rename = "Damage")]
        damage: Option<i32>,
        #[nbt(rename = "BlockEntityTag")]
        block_entity_tag: Option<BlockEntityTag>,
        #[nbt(rename = "Trim")]
        trim: Option<Trim>,
    }

    #[derive(Debug, NBTSerialize, NBTDeserialize)]
    struct Display {
        #[nbt(rename = "Name")]
        name: Option<String>,
    }

    #[derive(Debug, NBTSerialize, NBTDeserialize)]
    struct Enchantment {
        id: String,
        lvl: i16,
    }

    #[derive(Debug, NBTSerialize, NBTDeserialize)]
    struct BlockEntityTag {
        #[nbt(rename = "Items")]
        items: Option<Vec<Item>>,
        #[nbt(rename = "CustomName")]
        custom_name: Option<String>,
        #[nbt(rename = "Patterns")]
        patterns: Option<Vec<Pattern>>,
        #[nbt(rename = "Base")]
        base: Option<i32>,
    }

    #[derive(Debug, NBTSerialize, NBTDeserialize)]
    struct Pattern {
        #[nbt(rename = "Color")]
        color: i32,
        #[nbt(rename = "Pattern")]
        pattern: String,
    }

    #[derive(Debug, NBTSerialize, NBTDeserialize)]
    struct Trim {
        pattern: String,
        material: String,
    }

    #[derive(Debug, NBTSerialize, NBTDeserialize)]
    struct Abilities {
        flying: bool,
        instabuild: bool,
        mayfly: bool,
        #[nbt(rename = "flySpeed")]
        fly_speed: f32,
        #[nbt(rename = "walkSpeed")]
        walk_speed: f32,
        invulnerable: bool,
        #[nbt(rename = "mayBuild")]
        may_build: bool,
    }

    #[derive(Debug, NBTSerialize, NBTDeserialize)]
    struct WardenSpawnTracker {
        warning_level: i32,
        ticks_since_last_warning: i32,
        cooldown_ticks: i32,
    }

    #[derive(Debug, NBTSerialize, NBTDeserialize)]
    struct RecipeBook {
        recipes: Vec<String>,
        #[nbt(rename = "toBeDisplayed")]
        to_be_displayed: Vec<String>,
        #[nbt(rename = "isFilteringCraftable")]
        is_filtering_craftable: bool,
        #[nbt(rename = "isGuiOpen")]
        is_gui_open: bool,
        #[nbt(rename = "isFurnaceFilteringCraftable")]
        is_furnace_filtering_craftable: bool,
        #[nbt(rename = "isFurnaceGuiOpen")]
        is_furnace_gui_open: bool,
        #[nbt(rename = "isBlastingFurnaceFilteringCraftable")]
        is_blasting_furnace_filtering_craftable: bool,
        #[nbt(rename = "isBlastingFurnaceGuiOpen")]
        is_blasting_furnace_gui_open: bool,
        #[nbt(rename = "isSmokerFilteringCraftable")]
        is_smoker_filtering_craftable: bool,
        #[nbt(rename = "isSmokerGuiOpen")]
        is_smoker_gui_open: bool,
    }

    #[derive(Debug, NBTSerialize, NBTDeserialize)]
    struct Brain {
        memories: HashMap<String, String>,
    }

    #[derive(Debug, NBTSerialize, NBTDeserialize)]
    struct Attribute {
        #[nbt(rename = "Name")]
        name: String,
        #[nbt(rename = "Base")]
        base: f64,
    }
}

#[test]
fn showcase_nbt_usage() {
    #[derive(NBTSerialize, NBTDeserialize)]
    #[nbt(is_root)]
    struct Sample {
        name: String,
        hand_item: Item,
    }

    #[derive(NBTSerialize, NBTDeserialize)]
    struct Item {
        id: String,
    }

    // Create an instance of the sample
    let sample = Sample {
        name: "Steve".to_string(),
        hand_item: Item {
            id: "minecraft:diamond_sword".to_string(),
        },
    };

    // Serialize the item into plain bytes:
    let mut buffer = Vec::new(); // => Serialized bytes
    sample.nbt_serialize(&mut buffer).unwrap();

    // Deserialize the item from the bytes:
    Sample::read_from_bytes(&mut Cursor::new(buffer)).unwrap();
}

#[test]
fn test_byte_array() {
    #[derive(NBTSerialize, NBTDeserialize, Debug)]
    #[nbt(is_root)]
    struct ByteArray {
        data: Vec<i8>,
    }

    let byte_array = ByteArray {
        data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    };

    let mut buffer = Vec::new();
    byte_array.nbt_serialize(&mut buffer).unwrap();

    let deserialized_byte_array = ByteArray::read_from_bytes(&mut Cursor::new(buffer)).unwrap();
    println!("{:?}", deserialized_byte_array);
}

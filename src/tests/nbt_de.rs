use std::io::{Read, Write};
use nbt_lib::{Deserialize, NBTDeserializeBytes, };
use crate::tests::nbt_de::alguy_struct::PlayerData;

#[test]
fn try_read() {
    let file_bytes = std::fs::read(".etc/test.nbt").unwrap();

    #[derive(Deserialize, Debug)]
    #[nbt(is_root)]
    #[nbt(rename = "")]
    struct Test{
        field_one: i8,
        field_two: i16,
        optional_field: Option<i32>,
    }

    let mut cursor = std::io::Cursor::new(file_bytes);
    let root = Test::read_from_bytes(&mut cursor).unwrap();

    println!("{:?}", root);
}
mod alguy_struct {
    use std::collections::HashMap;
    use nbt_lib::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
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

    #[derive(Debug, Serialize, Deserialize)]
    struct Item {
        #[nbt(rename = "Slot")]
        slot: i8,
        id: String,
        #[nbt(rename = "Count")]
        count: i8,
        tag: Option<ItemTag>,
    }

    #[derive(Debug, Serialize, Deserialize)]
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

    #[derive(Debug, Serialize, Deserialize)]
    struct Display {
        #[nbt(rename = "Name")]
        name: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Enchantment {
        id: String,
        lvl: i16,
    }

    #[derive(Debug, Serialize, Deserialize)]
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

    #[derive(Debug, Serialize, Deserialize)]
    struct Pattern {
        #[nbt(rename = "Color")]
        color: i32,
        #[nbt(rename = "Pattern")]
        pattern: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Trim {
        pattern: String,
        material: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
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

    #[derive(Debug, Serialize, Deserialize)]
    struct WardenSpawnTracker {
        warning_level: i32,
        ticks_since_last_warning: i32,
        cooldown_ticks: i32,
    }

    #[derive(Debug, Serialize, Deserialize)]
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

    #[derive(Debug, Serialize, Deserialize)]
    struct Brain {
        memories: HashMap<String, String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Attribute {
        #[nbt(rename = "Name")]
        name: String,
        #[nbt(rename = "Base")]
        base: f64,
    }
}
use crate::utils;
use heck::ToShoutySnakeCase;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::Write;
use std::path::Path;

// --- JSON Parsing Structs (Matching items.json) ---

#[derive(Deserialize)]
struct RegistryReport {
    #[serde(rename = "minecraft:item")]
    item_registry: RegistryEntries,
}

#[derive(Deserialize)]
struct RegistryEntries {
    entries: HashMap<String, RegistryEntry>,
}

#[derive(Deserialize)]
struct RegistryEntry {
    protocol_id: u32,
}

#[derive(Deserialize)]
struct ItemDefinition {
    components: ItemComponents,
}

#[derive(Deserialize)]
struct ItemComponents {
    #[serde(rename = "minecraft:max_stack_size")]
    max_stack_size: Option<u8>,
    #[serde(rename = "minecraft:max_damage")]
    max_damage: Option<u16>,
    #[serde(rename = "minecraft:rarity")]
    rarity: Option<String>,
    #[serde(rename = "minecraft:damage_resistant")]
    damage_resistant: Option<DamageResistant>,

    #[serde(rename = "minecraft:food")]
    food: Option<FoodComponent>,
    #[serde(rename = "minecraft:tool")]
    tool: Option<ToolComponent>,
    #[serde(rename = "minecraft:enchantable")]
    enchantable: Option<EnchantableComponent>,
    #[serde(rename = "minecraft:repairable")]
    repairable: Option<RepairableComponent>,

    #[serde(rename = "minecraft:attribute_modifiers")]
    attributes: Option<AttributeModifiers>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum AttributeModifiers {
    Object { modifiers: Vec<AttributeEntry> },
    List(Vec<AttributeEntry>),
}

impl AttributeModifiers {
    // Helper to unify access
    fn into_vec(self) -> Vec<AttributeEntry> {
        match self {
            Self::Object { modifiers } => modifiers,
            Self::List(list) => list,
        }
    }
}

#[derive(Deserialize)]
struct DamageResistant {
    types: String, // Tag like "#minecraft:is_fire"
}

#[derive(Deserialize)]
struct FoodComponent {
    nutrition: u32,
    saturation: f32,
    #[serde(default)]
    can_always_eat: bool,
    // "minecraft:consumable" usually holds eat_seconds, but we'll default to 1.6
    // TODO: eat_seconds 
}

#[derive(Deserialize)]
struct ToolComponent {
    rules: Vec<ToolRuleEntry>,
    #[serde(default = "default_mining_speed")]
    default_mining_speed: f32,
    #[serde(default = "default_damage")]
    damage_per_block: u32,
}
fn default_mining_speed() -> f32 {
    1.0
}
fn default_damage() -> u32 {
    1
}

#[derive(Deserialize)]
#[serde(untagged)]
enum BlockList {
    Tag(String),
    List(Vec<String>),
}

#[derive(Deserialize)]
struct ToolRuleEntry {
    blocks: BlockList,
    speed: Option<f32>,
    correct_for_drops: Option<bool>,
}

#[derive(Deserialize)]
struct EnchantableComponent {
    value: u32,
}

#[derive(Deserialize)]
struct RepairableComponent {
    items: String,
}

#[derive(Deserialize)]
struct AttributeEntry {
    #[serde(rename = "type")]
    kind: String,
    amount: f64,
    operation: String,
    slot: Option<String>, // Sometimes "any" or valid slot group
}

// --- Generator Logic ---

pub fn generate(report_path: &Path, registries_path: &Path, output: &Path) {
    println!("   ... Parsing Items from {:?}", report_path);
    let content = fs::read_to_string(report_path).expect("Failed to read items.json");
    // The file is a Map: "minecraft:apple" -> Definition
    let raw_report: BTreeMap<String, serde_json::Value> =
        serde_json::from_str(&content).expect("Failed to parse items.json structure");

    println!("   ... Parsing Registries from {:?}", registries_path);
    let reg_content = fs::read_to_string(registries_path).expect("Failed to read registries.json");
    let reg_report: RegistryReport =
        serde_json::from_str(&reg_content).expect("Failed to parse registries.json");

    let id_map: HashMap<String, u32> = reg_report
        .item_registry
        .entries
        .into_iter()
        .map(|(k, v)| (k, v.protocol_id))
        .collect();

    let mut file = utils::create_file(output);

    // Imports
    writeln!(file, "use ferrumc_core::items::item_data::*;").unwrap();
    writeln!(file, "").unwrap();

    let mut name_match_arms = String::new();
    let mut id_match_arms = String::new();

    for (name, json_value) in raw_report {
        // Try to parse the specific item definition
        let def: ItemDefinition = match serde_json::from_value(json_value) {
            Ok(d) => d,
            Err(e) => {
                // --- DEBUG PRINT ---
                println!("FAILED to parse item: {}", name);
                println!("Error: {}", e);
                // We panic here so you can see it immediately
                panic!("Schema mismatch on {}", name);
            }
        };

        let clean_name = name.replace("minecraft:", "");
        let const_name = clean_name.to_shouty_snake_case();
        let safe_const_name = if const_name == "TYPE" || const_name == "MATCH" {
            format!("{}_ITEM", const_name)
        } else {
            const_name
        };

        // --- Lookup ID ---
        let protocol_id = *id_map.get(&name).unwrap_or_else(|| {
            println!("Warning: No ID found for {}", name);
            &0
        });

        let comp = def.components;

        // --- Basic Fields ---
        let stack_size = comp.max_stack_size.unwrap_or(64);
        let max_damage = comp.max_damage.unwrap_or(0);

        let rarity = match comp.rarity.as_deref().unwrap_or("common") {
            "uncommon" => "Rarity::Uncommon",
            "rare" => "Rarity::Rare",
            "epic" => "Rarity::Epic",
            _ => "Rarity::Common",
        };

        // Check if fire resistant (tag contains "fire")
        let is_fire_resistant = comp
            .damage_resistant
            .map(|dr| dr.types.contains("fire"))
            .unwrap_or(false);

        // --- Food ---
        let food_str = if let Some(f) = comp.food {
            format!("Some(FoodData {{ nutrition: {}, saturation: {:.1}, can_always_eat: {}, eat_seconds: 1.6 }})",
                f.nutrition, f.saturation, f.can_always_eat)
        } else {
            "None".to_string()
        };
        // --- Tool ---
        let tool_str = if let Some(t) = comp.tool {
            let mut rules_str = String::new();
            for r in t.rules {
                // Handle the Enum
                let blocks_string = match r.blocks {
                    BlockList::Tag(t) => t,
                    BlockList::List(l) => l.join(","),
                };

                rules_str.push_str(&format!(
                    "ToolRule {{ blocks: \"{}\", speed: {:?}, correct_for_drops: {:?} }}, ",
                    blocks_string, r.speed, r.correct_for_drops
                ));
            }
            format!("Some(ToolData {{ default_mining_speed: {:.1}, damage_per_block: {}, rules: &[{}] }})",
                t.default_mining_speed, t.damage_per_block, rules_str)
        } else {
            "None".to_string()
        };

        // --- Enchantable/Repairable ---
        let enchant_str = if let Some(e) = comp.enchantable {
            format!("Some({})", e.value)
        } else {
            "None".to_string()
        };
        let repair_str = if let Some(r) = comp.repairable {
            format!("Some(\"{}\")", r.items)
        } else {
            "None".to_string()
        };

        // --- Attributes Logic ---
        let mut attr_str = String::new();
        if let Some(attrs) = comp.attributes {
            // Use the helper to unwrap the enum
            for a in attrs.into_vec() {
                let kind = match a.kind.replace("minecraft:", "").as_str() {
                    "attack_damage" | "generic.attack_damage" => "AttributeType::AttackDamage",
                    "attack_speed" | "generic.attack_speed" => "AttributeType::AttackSpeed",
                    "max_health" | "generic.max_health" => "AttributeType::MaxHealth",
                    "movement_speed" | "generic.movement_speed" => "AttributeType::MovementSpeed",
                    "knockback_resistance" | "generic.knockback_resistance" => {
                        "AttributeType::KnockbackResistance"
                    }
                    "armor" | "generic.armor" => "AttributeType::Armor",
                    "armor_toughness" | "generic.armor_toughness" => {
                        "AttributeType::ArmorToughness"
                    }
                    "luck" | "generic.luck" => "AttributeType::Luck",
                    _ => "AttributeType::Unknown",
                };

                let op = match a.operation.as_str() {
                    "add_value" => "AttributeOperation::AddValue",
                    "add_multiplied_base" => "AttributeOperation::MultiplyBase",
                    "add_multiplied_total" => "AttributeOperation::MultiplyTotal",
                    _ => "AttributeOperation::AddValue",
                };

                let slot = match a.slot.as_deref().unwrap_or("any") {
                    "mainhand" => "EquipmentSlot::MainHand",
                    "offhand" => "EquipmentSlot::OffHand",
                    "feet" => "EquipmentSlot::Feet",
                    "legs" => "EquipmentSlot::Legs",
                    "chest" => "EquipmentSlot::Chest",
                    "head" => "EquipmentSlot::Head",
                    "body" => "EquipmentSlot::Body",
                    _ => "EquipmentSlot::Any",
                };

                attr_str.push_str(&format!(
                    "AttributeModifier {{ kind: {}, amount: {:.4}, operation: {}, slot: {} }}, ",
                    kind, a.amount, op, slot
                ));
            }
        }

        // --- Write Output ---
        writeln!(
            file,
            "pub const {}: ItemData = ItemData {{",
            safe_const_name
        )
        .unwrap();
        writeln!(file, "    name: \"{}\",", name).unwrap();
        writeln!(file, "    protocol_id: {},", protocol_id).unwrap();
        writeln!(file, "    max_stack_size: {},", stack_size).unwrap();
        writeln!(file, "    max_damage: {},", max_damage).unwrap();
        writeln!(file, "    is_fire_resistant: {},", is_fire_resistant).unwrap();
        writeln!(file, "    rarity: {},", rarity).unwrap();
        writeln!(file, "    food: {},", food_str).unwrap();
        writeln!(file, "    tool: {},", tool_str).unwrap();
        writeln!(file, "    enchantable_value: {},", enchant_str).unwrap();
        writeln!(file, "    repairable_with: {},", repair_str).unwrap();
        writeln!(file, "    attribute_modifiers: &[{}],", attr_str).unwrap();
        writeln!(file, "}};").unwrap();
        writeln!(file, "").unwrap();

        name_match_arms.push_str(&format!(
            "        \"{}\" => Some(&{}),\n",
            clean_name, safe_const_name
        ));

        id_match_arms.push_str(&format!(
            "        {} => Some(&{}),\n",
            protocol_id, safe_const_name
        ));
    }

    // --- Write get_item_by_name ---
    writeln!(
        file,
        "pub fn get_item_by_name(name: &str) -> Option<&'static ItemData> {{"
    )
    .unwrap();
    writeln!(
        file,
        "    let name = name.strip_prefix(\"minecraft:\").unwrap_or(name);"
    )
    .unwrap();
    writeln!(file, "    match name {{").unwrap();
    file.write_all(name_match_arms.as_bytes()).unwrap();
    writeln!(file, "        _ => None,").unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(file, "}}").unwrap();

    // --- Write get_item_by_id ---
    writeln!(file, "").unwrap();
    writeln!(
        file,
        "pub fn get_item_by_id(id: u32) -> Option<&'static ItemData> {{"
    )
    .unwrap();
    writeln!(file, "    match id {{").unwrap();
    file.write_all(id_match_arms.as_bytes()).unwrap();
    writeln!(file, "        _ => None,").unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(file, "}}").unwrap();
}

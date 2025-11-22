use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ItemData {
    pub name: &'static str,
    pub protocol_id: u32,

    // --- Properties ---
    pub max_stack_size: u8,
    pub max_damage: u16,
    pub rarity: Rarity,

    // Replaces old "fire_resistant" bool.
    // If true, it won't burn in lava/fire.
    pub is_fire_resistant: bool,

    // --- Components ---
    pub food: Option<FoodData>,
    pub tool: Option<ToolData>,

    /// How easily this item receives enchantments (e.g. Gold = 25, Stone = 5).
    /// None if not enchantable.
    pub enchantable_value: Option<u32>,

    /// The Item Tag or ID that repairs this item in an anvil.
    /// e.g. "#minecraft:wooden_tool_materials"
    pub repairable_with: Option<&'static str>,

    /// Modifiers applied when holding/wearing the item.
    pub attribute_modifiers: &'static [AttributeModifier],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum Rarity {
    #[default]
    Common,
    Uncommon,
    Rare,
    Epic,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FoodData {
    pub nutrition: u32,
    pub saturation: f32,
    pub can_always_eat: bool,
    pub eat_seconds: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ToolData {
    pub default_mining_speed: f32,
    pub damage_per_block: u32,
    pub rules: &'static [ToolRule],
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ToolRule {
    pub blocks: &'static str, // Tag or Block ID
    pub speed: Option<f32>,
    pub correct_for_drops: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeModifier {
    pub kind: AttributeType,
    pub amount: f64,
    pub operation: AttributeOperation,
    pub slot: EquipmentSlot,
}

// Enums for Attributes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttributeType {
    AttackDamage,
    AttackSpeed,
    MaxHealth,
    MovementSpeed,
    KnockbackResistance,
    Armor,
    ArmorToughness,
    Luck,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttributeOperation {
    AddValue,
    MultiplyBase,
    MultiplyTotal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquipmentSlot {
    MainHand,
    OffHand,
    Feet,
    Legs,
    Chest,
    Head,
    Body,
    Any,
}

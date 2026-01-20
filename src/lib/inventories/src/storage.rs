//! Storage-friendly representations of inventory data for bitcode persistence.
//!
//! This module provides types that can be serialized with bitcode for persistent storage,
//! separate from the network-oriented types used for protocol encoding.
//!
//! # Design
//! - Network types (`Component`, `InventorySlot`) use `NBT<T>`, `LengthPrefixedVec<T>`, etc.
//! - Storage types use primitives and JSON strings that bitcode can handle.
//! - Bidirectional conversion via `From`/`TryFrom` traits.

use crate::components::*;
use crate::item::ItemID;
use crate::slot::InventorySlot;
use bitcode_derive::{Decode, Encode};
use ferrumc_nbt::NBT;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_text::TextComponent;

/// Error type for storage conversion failures.
#[derive(Debug, Clone)]
pub enum StorageConversionError {
    /// Failed to serialize/deserialize JSON for TextComponent
    JsonError(String),
    /// Unknown component type during conversion
    UnknownComponent(i32),
    /// Invalid enum value during conversion
    InvalidEnumValue { type_name: &'static str, value: u8 },
}

impl std::fmt::Display for StorageConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JsonError(msg) => write!(f, "JSON conversion error: {}", msg),
            Self::UnknownComponent(id) => write!(f, "Unknown component ID: {}", id),
            Self::InvalidEnumValue { type_name, value } => {
                write!(f, "Invalid {} value: {}", type_name, value)
            }
        }
    }
}

impl std::error::Error for StorageConversionError {}

// ============================================================================
// Storage Enchantment
// ============================================================================

/// Storage-friendly enchantment data.
#[derive(Debug, Clone, Encode, Decode)]
pub struct StorageEnchant {
    pub id: i32,
    pub level: i32,
}

impl From<&EnchantComponent> for StorageEnchant {
    fn from(e: &EnchantComponent) -> Self {
        Self {
            id: e.id.0,
            level: e.level.0,
        }
    }
}

impl From<StorageEnchant> for EnchantComponent {
    fn from(e: StorageEnchant) -> Self {
        Self {
            id: VarInt(e.id),
            level: VarInt(e.level),
        }
    }
}

// ============================================================================
// Storage Component
// ============================================================================

/// Storage-friendly component representation.
///
/// Uses primitive types and JSON strings instead of network-specific wrappers.
/// Each variant maps 1:1 with `Component` but uses bitcode-compatible types.
#[derive(Debug, Clone, Encode, Decode)]
pub enum StorageComponent {
    // ID 0
    CustomData(Vec<u8>),
    // ID 1
    MaxStackSize(i32),
    // ID 2
    MaxDamage(i32),
    // ID 3
    Damage(i32),
    // ID 4
    Unbreakable,
    // ID 5 - TextComponent as JSON
    CustomName(String),
    // ID 6
    ItemName(String),
    // ID 7
    ItemModel(String),
    // ID 8 - Vec of JSON strings
    Lore(Vec<String>),
    // ID 9
    Rarity(u8),
    // ID 10
    Enchantments(Vec<StorageEnchant>),
    // ID 11-12: BlockPredicates - store as JSON for simplicity
    CanPlaceOn(String),
    CanBreak(String),
    // ID 13
    AttributeModifiers(String), // Complex, store as JSON
    // ID 14
    CustomModelData {
        floats: Vec<f32>,
        flags: Vec<bool>,
        strings: Vec<String>,
        colors: Vec<i32>,
    },
    // ID 15
    TooltipDisplay {
        hide_tooltip: bool,
        hidden_components: Vec<i32>,
    },
    // ID 16
    RepairCost(i32),
    // ID 17
    CreativeSlotLock,
    // ID 18
    EnchantmentGlintOverride(bool),
    // ID 19
    IntangibleProjectile(Vec<u8>),
    // ID 20
    Food {
        nutrition: i32,
        saturation_modifier: f32,
        can_always_eat: bool,
    },
    // ID 21 - Complex, store as JSON
    Consumable(String),
    // ID 22 - Recursive slot stored as raw bytes to avoid cycle
    UseRemainder(Vec<u8>),
    // ID 23
    UseCooldown {
        seconds: f32,
        cooldown_group: Option<String>,
    },
    // ID 24
    DamageResistant(String),
    // ID 25 - Complex
    Tool(String),
    // ID 26
    Weapon {
        damage: i32,
        disable_blocking_for_seconds: f32,
    },
    // ID 27
    Enchantable(i32),
    // ID 28 - Complex
    Equippable(String),
    // ID 29
    Repairable(String), // IDSet as JSON
    // ID 30
    Glider,
    // ID 31
    TooltipStyle(String),
    // ID 32
    DeathProtection(String), // Complex
    // ID 33
    BlocksAttacks(String), // Complex
    // ID 34
    StoredEnchantments(Vec<StorageEnchant>),
    // ID 35
    DyedColor(i32),
    // ID 36
    MapColor(i32),
    // ID 37
    MapId(i32),
    // ID 38
    MapDecorations(Vec<u8>),
    // ID 39
    MapPostProcessing(u8),
    // ID 40 - Contains slots, stored as raw bytes to avoid cycle
    ChargedProjectiles(Vec<u8>),
    // ID 41 - Contains slots, stored as raw bytes to avoid cycle
    BundleContents(Vec<u8>),
    // ID 42 - Complex
    PotionContents(String),
    // ID 43
    PotionDurationScale(f32),
    // ID 44
    SuspiciousStewEffects(String),
    // ID 45
    WritableBookContent(String),
    // ID 46
    WrittenBookContent(String),
    // ID 47
    Trim(String),
    // ID 48
    DebugStickState(Vec<u8>),
    // ID 49
    EntityData {
        entity_type: i32,
        data: Vec<u8>,
    },
    // ID 50
    BucketEntityData(Vec<u8>),
    // ID 51
    BlockEntityData {
        block_entity_type: i32,
        data: Vec<u8>,
    },
    // ID 52
    Instrument(String),
    // ID 53
    ProvidesTrimMaterial(String),
    // ID 54
    OminousBottleAmplifier(i32),
    // ID 55
    JukeboxPlayable(String),
    // ID 56
    ProvidesBannerPatterns(String),
    // ID 57
    Recipes(Vec<u8>),
    // ID 58
    LodestoneTracker(String),
    // ID 59
    FireworkExplosion(String),
    // ID 60
    Fireworks(String),
    // ID 61
    Profile(String),
    // ID 62
    NoteBlockSound(String),
    // ID 63
    BannerPatterns(String),
    // ID 64
    BaseColor(u8),
    // ID 65
    PotDecorations(Vec<i32>),
    // ID 66 - Contains slots, stored as raw bytes to avoid cycle
    Container(Vec<u8>),
    // ID 67
    BlockState(Vec<(String, String)>),
    // ID 68
    Bees(String),
    // ID 69
    Lock(String),
    // ID 70
    ContainerLoot(Vec<u8>),
    // ID 71
    BreakSound(String),
    // ID 72-95: Various entity variants - simple numeric IDs
    VillagerVariant(i32),
    WolfVariant(i32),
    WolfSoundVariant(i32),
    WolfCollar(u8),
    FoxVariant(u8),
    SalmonSize(u8),
    ParrotVariant(u8),
    TropicalFishPattern(u8),
    TropicalFishBaseColor(u8),
    TropicalFishPatternColor(u8),
    MooshroomVariant(u8),
    RabbitVariant(u8),
    PigVariant(i32),
    CowVariant(i32),
    ChickenVariant(String),
    FrogVariant(i32),
    HorseVariant(u8),
    PaintingVariant(String),
    LlamaVariant(u8),
    AxolotlVariant(u8),
    CatVariant(i32),
    CatCollar(u8),
    SheepColor(u8),
    ShulkerColor(u8),
}

// ============================================================================
// Storage Inventory Slot
// ============================================================================

/// Storage-friendly inventory slot representation.
#[derive(Debug, Clone, Default, Encode, Decode)]
pub struct StorageInventorySlot {
    pub count: i32,
    pub item_id: Option<i32>,
    pub components_to_add: Vec<StorageComponent>,
    pub components_to_remove: Vec<i32>,
}

// ============================================================================
// Helper Functions
// ============================================================================

fn text_to_json(tc: &TextComponent) -> String {
    serde_json::to_string(tc).unwrap_or_default()
}

fn json_to_text(json: &str) -> Result<TextComponent, StorageConversionError> {
    serde_json::from_str(json).map_err(|e| StorageConversionError::JsonError(e.to_string()))
}

// ============================================================================
// Safe Enum Conversion Helpers
// ============================================================================

/// Macro to generate safe u8-to-enum conversion functions.
/// These replace unsafe transmute calls with explicit match statements.
macro_rules! impl_safe_enum_convert {
    ($fn_name:ident, $enum_type:ty, [$($variant:ident = $value:expr),+ $(,)?]) => {
        fn $fn_name(value: u8) -> Result<$enum_type, StorageConversionError> {
            match value {
                $($value => Ok(<$enum_type>::$variant),)+
                _ => Err(StorageConversionError::InvalidEnumValue {
                    type_name: stringify!($enum_type),
                    value,
                }),
            }
        }
    };
}

impl_safe_enum_convert!(rarity_from_u8, Rarity, [
    Common = 0, Uncommon = 1, Rare = 2, Epic = 3
]);

impl_safe_enum_convert!(dye_color_from_u8, DyeColor, [
    White = 0, Orange = 1, Magenta = 2, LightBlue = 3, Yellow = 4, Lime = 5,
    Pink = 6, Gray = 7, LightGray = 8, Cyan = 9, Purple = 10, Blue = 11,
    Brown = 12, Green = 13, Red = 14, Black = 15
]);

impl_safe_enum_convert!(map_post_processing_from_u8, MapPostProcessing, [
    Lock = 0, Scale = 1
]);

impl_safe_enum_convert!(fox_variant_from_u8, FoxVariant, [
    Red = 0, Snow = 1
]);

impl_safe_enum_convert!(salmon_size_from_u8, SalmonSize, [
    Small = 0, Medium = 1, Large = 2
]);

impl_safe_enum_convert!(parrot_variant_from_u8, ParrotVariant, [
    RedBlue = 0, Blue = 1, Green = 2, YellowBlue = 3, Gray = 4
]);

impl_safe_enum_convert!(tropical_fish_pattern_from_u8, TropicalFishPattern, [
    Kob = 0, Sunstreak = 1, Snooper = 2, Dasher = 3, Brinely = 4, Spotty = 5,
    Flopper = 6, Stripey = 7, Glitter = 8, Blockfish = 9, Betty = 10, Clayfish = 11
]);

impl_safe_enum_convert!(mooshroom_variant_from_u8, MooshroomVariant, [
    Red = 0, Brown = 1
]);

impl_safe_enum_convert!(rabbit_variant_from_u8, RabbitVariant, [
    Brown = 0, White = 1, Black = 2, BlackAndWhite = 3, Gold = 4, SaltAndPepper = 5, Evil = 6
]);

impl_safe_enum_convert!(horse_variant_from_u8, HorseVariant, [
    White = 0, Creamy = 1, Chestnut = 2, Brown = 3, Black = 4, Gray = 5, DarkBrown = 6
]);

impl_safe_enum_convert!(llama_variant_from_u8, LlamaVariant, [
    Creamy = 0, White = 1, Brown = 2, Gray = 3
]);

impl_safe_enum_convert!(axolotl_variant_from_u8, AxolotlVariant, [
    Lucy = 0, Wild = 1, Gold = 2, Cyan = 3, Blue = 4
]);

// ============================================================================
// Component -> StorageComponent Conversion
// ============================================================================

impl From<&Component> for StorageComponent {
    fn from(component: &Component) -> Self {
        match component {
            Component::CustomData(raw) => StorageComponent::CustomData(raw.0.clone()),
            Component::MaxStackSize(v) => StorageComponent::MaxStackSize(v.0),
            Component::MaxDamage(v) => StorageComponent::MaxDamage(v.0),
            Component::Damage(v) => StorageComponent::Damage(v.0),
            Component::Unbreakable => StorageComponent::Unbreakable,
            Component::CustomName(nbt) => StorageComponent::CustomName(text_to_json(nbt)),
            Component::ItemName(nbt) => StorageComponent::ItemName(text_to_json(nbt)),
            Component::ItemModel(s) => StorageComponent::ItemModel(s.clone()),
            Component::Lore(vec) => {
                StorageComponent::Lore(vec.data.iter().map(|nbt| text_to_json(nbt)).collect())
            }
            Component::Rarity(r) => StorageComponent::Rarity(*r as u8),
            Component::Enchantments(vec) => {
                StorageComponent::Enchantments(vec.data.iter().map(Into::into).collect())
            }
            // Complex types - store placeholder (would need Serialize derives)
            Component::CanPlaceOn(_) => StorageComponent::CanPlaceOn(String::new()),
            Component::CanBreak(_) => StorageComponent::CanBreak(String::new()),
            Component::AttributeModifiers(_) => StorageComponent::AttributeModifiers(String::new()),
            Component::CustomModelData {
                floats,
                flags,
                strings,
                colors,
            } => StorageComponent::CustomModelData {
                floats: floats.data.clone(),
                flags: flags.data.clone(),
                strings: strings.data.clone(),
                colors: colors.data.clone(),
            },
            Component::TooltipDisplay {
                hide_tooltip,
                hidden_components,
            } => StorageComponent::TooltipDisplay {
                hide_tooltip: *hide_tooltip,
                hidden_components: hidden_components.data.iter().map(|v| v.0).collect(),
            },
            Component::RepairCost(v) => StorageComponent::RepairCost(v.0),
            Component::CreativeSlotLock => StorageComponent::CreativeSlotLock,
            Component::EnchantmentGlintOverride(b) => StorageComponent::EnchantmentGlintOverride(*b),
            Component::IntangibleProjectile(raw) => {
                StorageComponent::IntangibleProjectile(raw.0.clone())
            }
            Component::Food {
                nutrition,
                saturation_modifier,
                can_always_eat,
            } => StorageComponent::Food {
                nutrition: nutrition.0,
                saturation_modifier: *saturation_modifier,
                can_always_eat: *can_always_eat,
            },
            Component::Consumable { .. } => StorageComponent::Consumable(String::new()),
            Component::UseRemainder(_) => {
                // Recursive slot - store as empty for now (complex serialization needed)
                StorageComponent::UseRemainder(Vec::new())
            }
            Component::UseCooldown {
                seconds,
                cooldown_group,
            } => StorageComponent::UseCooldown {
                seconds: *seconds,
                cooldown_group: cooldown_group.clone().to_option(),
            },
            Component::DamageResistant(s) => StorageComponent::DamageResistant(s.clone()),
            Component::Tool { .. } => StorageComponent::Tool(String::new()),
            Component::Weapon {
                damage,
                disable_blocking_for_seconds,
            } => StorageComponent::Weapon {
                damage: damage.0,
                disable_blocking_for_seconds: *disable_blocking_for_seconds,
            },
            Component::Enchantable(v) => StorageComponent::Enchantable(v.0),
            Component::Equippable { .. } => StorageComponent::Equippable(String::new()),
            Component::Repairable(_) => StorageComponent::Repairable(String::new()),
            Component::Glider => StorageComponent::Glider,
            Component::TooltipStyle(s) => StorageComponent::TooltipStyle(s.clone()),
            Component::DeathProtection(_) => StorageComponent::DeathProtection(String::new()),
            Component::BlocksAttacks { .. } => StorageComponent::BlocksAttacks(String::new()),
            Component::StoredEnchantments(vec) => {
                StorageComponent::StoredEnchantments(vec.data.iter().map(Into::into).collect())
            }
            Component::DyedColor(c) => StorageComponent::DyedColor(*c),
            Component::MapColor(c) => StorageComponent::MapColor(*c),
            Component::MapId(v) => StorageComponent::MapId(v.0),
            Component::MapDecorations(raw) => StorageComponent::MapDecorations(raw.0.clone()),
            Component::MapPostProcessing(m) => StorageComponent::MapPostProcessing(*m as u8),
            Component::ChargedProjectiles(_) => {
                // Contains recursive slots - store as empty for now
                StorageComponent::ChargedProjectiles(Vec::new())
            }
            Component::BundleContents(_) => {
                // Contains recursive slots - store as empty for now
                StorageComponent::BundleContents(Vec::new())
            }
            Component::PotionContents { .. } => StorageComponent::PotionContents(String::new()),
            Component::PotionDurationScale(f) => StorageComponent::PotionDurationScale(*f),
            Component::SuspiciousStewEffects(_) => StorageComponent::SuspiciousStewEffects(String::new()),
            Component::WritableBookContent(_) => StorageComponent::WritableBookContent(String::new()),
            Component::WrittenBookContent { .. } => StorageComponent::WrittenBookContent(String::new()),
            Component::Trim { .. } => StorageComponent::Trim(String::new()),
            Component::DebugStickState(raw) => StorageComponent::DebugStickState(raw.0.clone()),
            Component::EntityData { entity_type, data } => StorageComponent::EntityData {
                entity_type: entity_type.0,
                data: data.0.clone(),
            },
            Component::BucketEntityData(raw) => StorageComponent::BucketEntityData(raw.0.clone()),
            Component::BlockEntityData {
                block_entity_type,
                data,
            } => StorageComponent::BlockEntityData {
                block_entity_type: block_entity_type.0,
                data: data.0.clone(),
            },
            Component::Instrument(_) => StorageComponent::Instrument(String::new()),
            Component::ProvidesTrimMaterial { .. } => StorageComponent::ProvidesTrimMaterial(String::new()),
            Component::OminousBottleAmplifier(v) => StorageComponent::OminousBottleAmplifier(v.0),
            Component::JukeboxPlayable { .. } => StorageComponent::JukeboxPlayable(String::new()),
            Component::ProvidesBannerPatterns(s) => {
                StorageComponent::ProvidesBannerPatterns(s.clone())
            }
            Component::Recipes(raw) => StorageComponent::Recipes(raw.0.clone()),
            Component::LodestoneTracker { .. } => StorageComponent::LodestoneTracker(String::new()),
            Component::FireworkExplosion(_) => StorageComponent::FireworkExplosion(String::new()),
            Component::Fireworks { .. } => StorageComponent::Fireworks(String::new()),
            Component::Profile(_) => StorageComponent::Profile(String::new()),
            Component::NoteBlockSound(s) => StorageComponent::NoteBlockSound(s.clone()),
            Component::BannerPatterns(_) => StorageComponent::BannerPatterns(String::new()),
            Component::BaseColor(c) => StorageComponent::BaseColor(*c as u8),
            Component::PotDecorations(vec) => {
                StorageComponent::PotDecorations(vec.data.iter().map(|v| v.0).collect())
            }
            Component::Container(_) => {
                // Contains recursive slots - store as empty for now
                StorageComponent::Container(Vec::new())
            }
            Component::BlockState(vec) => StorageComponent::BlockState(
                vec.data
                    .iter()
                    .map(|p| (p.name.clone(), p.value.clone()))
                    .collect(),
            ),
            Component::Bees(_) => StorageComponent::Bees(String::new()),
            Component::Lock(s) => StorageComponent::Lock(s.clone()),
            Component::ContainerLoot(raw) => StorageComponent::ContainerLoot(raw.0.clone()),
            Component::BreakSound(_) => StorageComponent::BreakSound(String::new()),
            Component::VillagerVariant(v) => StorageComponent::VillagerVariant(v.0),
            Component::WolfVariant(v) => StorageComponent::WolfVariant(v.0),
            Component::WolfSoundVariant(v) => StorageComponent::WolfSoundVariant(v.0),
            Component::WolfCollar(c) => StorageComponent::WolfCollar(*c as u8),
            Component::FoxVariant(v) => StorageComponent::FoxVariant(*v as u8),
            Component::SalmonSize(s) => StorageComponent::SalmonSize(*s as u8),
            Component::ParrotVariant(v) => StorageComponent::ParrotVariant(*v as u8),
            Component::TropicalFishPattern(p) => StorageComponent::TropicalFishPattern(*p as u8),
            Component::TropicalFishBaseColor(c) => StorageComponent::TropicalFishBaseColor(*c as u8),
            Component::TropicalFishPatternColor(c) => {
                StorageComponent::TropicalFishPatternColor(*c as u8)
            }
            Component::MooshroomVariant(v) => StorageComponent::MooshroomVariant(*v as u8),
            Component::RabbitVariant(v) => StorageComponent::RabbitVariant(*v as u8),
            Component::PigVariant(v) => StorageComponent::PigVariant(v.0),
            Component::CowVariant(v) => StorageComponent::CowVariant(v.0),
            Component::ChickenVariant(_) => StorageComponent::ChickenVariant(String::new()),
            Component::FrogVariant(v) => StorageComponent::FrogVariant(v.0),
            Component::HorseVariant(v) => StorageComponent::HorseVariant(*v as u8),
            Component::PaintingVariant(_) => StorageComponent::PaintingVariant(String::new()),
            Component::LlamaVariant(v) => StorageComponent::LlamaVariant(*v as u8),
            Component::AxolotlVariant(v) => StorageComponent::AxolotlVariant(*v as u8),
            Component::CatVariant(v) => StorageComponent::CatVariant(v.0),
            Component::CatCollar(c) => StorageComponent::CatCollar(*c as u8),
            Component::SheepColor(c) => StorageComponent::SheepColor(*c as u8),
            Component::ShulkerColor(c) => StorageComponent::ShulkerColor(*c as u8),
        }
    }
}

// ============================================================================
// StorageComponent -> Component Conversion
// ============================================================================

impl TryFrom<StorageComponent> for Component {
    type Error = StorageConversionError;

    fn try_from(storage: StorageComponent) -> Result<Self, Self::Error> {
        Ok(match storage {
            StorageComponent::CustomData(data) => Component::CustomData(RawNbt(data)),
            StorageComponent::MaxStackSize(v) => Component::MaxStackSize(VarInt(v)),
            StorageComponent::MaxDamage(v) => Component::MaxDamage(VarInt(v)),
            StorageComponent::Damage(v) => Component::Damage(VarInt(v)),
            StorageComponent::Unbreakable => Component::Unbreakable,
            StorageComponent::CustomName(json) => {
                Component::CustomName(NBT::new(json_to_text(&json)?))
            }
            StorageComponent::ItemName(json) => {
                Component::ItemName(NBT::new(json_to_text(&json)?))
            }
            StorageComponent::ItemModel(s) => Component::ItemModel(s),
            StorageComponent::Lore(lines) => {
                let data: Result<Vec<_>, _> = lines
                    .into_iter()
                    .map(|json| json_to_text(&json).map(NBT::new))
                    .collect();
                Component::Lore(LengthPrefixedVec::new(data?))
            }
            StorageComponent::Rarity(r) => Component::Rarity(rarity_from_u8(r)?),
            StorageComponent::Enchantments(vec) => {
                Component::Enchantments(LengthPrefixedVec::new(vec.into_iter().map(Into::into).collect()))
            }
            StorageComponent::RepairCost(v) => Component::RepairCost(VarInt(v)),
            StorageComponent::CreativeSlotLock => Component::CreativeSlotLock,
            StorageComponent::EnchantmentGlintOverride(b) => Component::EnchantmentGlintOverride(b),
            StorageComponent::IntangibleProjectile(data) => {
                Component::IntangibleProjectile(RawNbt(data))
            }
            StorageComponent::Food {
                nutrition,
                saturation_modifier,
                can_always_eat,
            } => Component::Food {
                nutrition: VarInt(nutrition),
                saturation_modifier,
                can_always_eat,
            },
            StorageComponent::Weapon {
                damage,
                disable_blocking_for_seconds,
            } => Component::Weapon {
                damage: VarInt(damage),
                disable_blocking_for_seconds,
            },
            StorageComponent::Enchantable(v) => Component::Enchantable(VarInt(v)),
            StorageComponent::Glider => Component::Glider,
            StorageComponent::TooltipStyle(s) => Component::TooltipStyle(s),
            StorageComponent::StoredEnchantments(vec) => {
                Component::StoredEnchantments(LengthPrefixedVec::new(vec.into_iter().map(Into::into).collect()))
            }
            StorageComponent::DyedColor(c) => Component::DyedColor(c),
            StorageComponent::MapColor(c) => Component::MapColor(c),
            StorageComponent::MapId(v) => Component::MapId(VarInt(v)),
            StorageComponent::MapDecorations(data) => Component::MapDecorations(RawNbt(data)),
            StorageComponent::MapPostProcessing(m) => {
                Component::MapPostProcessing(map_post_processing_from_u8(m)?)
            }
            StorageComponent::PotionDurationScale(f) => Component::PotionDurationScale(f),
            StorageComponent::DebugStickState(data) => Component::DebugStickState(RawNbt(data)),
            StorageComponent::EntityData { entity_type, data } => Component::EntityData {
                entity_type: VarInt(entity_type),
                data: RawNbt(data),
            },
            StorageComponent::BucketEntityData(data) => Component::BucketEntityData(RawNbt(data)),
            StorageComponent::BlockEntityData {
                block_entity_type,
                data,
            } => Component::BlockEntityData {
                block_entity_type: VarInt(block_entity_type),
                data: RawNbt(data),
            },
            StorageComponent::OminousBottleAmplifier(v) => {
                Component::OminousBottleAmplifier(VarInt(v))
            }
            StorageComponent::ProvidesBannerPatterns(s) => Component::ProvidesBannerPatterns(s),
            StorageComponent::Recipes(data) => Component::Recipes(RawNbt(data)),
            StorageComponent::NoteBlockSound(s) => Component::NoteBlockSound(s),
            StorageComponent::BaseColor(c) => Component::BaseColor(dye_color_from_u8(c)?),
            StorageComponent::Lock(s) => Component::Lock(s),
            StorageComponent::ContainerLoot(data) => Component::ContainerLoot(RawNbt(data)),
            StorageComponent::VillagerVariant(v) => Component::VillagerVariant(VarInt(v)),
            StorageComponent::WolfVariant(v) => Component::WolfVariant(VarInt(v)),
            StorageComponent::WolfSoundVariant(v) => Component::WolfSoundVariant(VarInt(v)),
            StorageComponent::WolfCollar(c) => Component::WolfCollar(dye_color_from_u8(c)?),
            StorageComponent::FoxVariant(v) => Component::FoxVariant(fox_variant_from_u8(v)?),
            StorageComponent::SalmonSize(s) => Component::SalmonSize(salmon_size_from_u8(s)?),
            StorageComponent::ParrotVariant(v) => {
                Component::ParrotVariant(parrot_variant_from_u8(v)?)
            }
            StorageComponent::TropicalFishPattern(p) => {
                Component::TropicalFishPattern(tropical_fish_pattern_from_u8(p)?)
            }
            StorageComponent::TropicalFishBaseColor(c) => {
                Component::TropicalFishBaseColor(dye_color_from_u8(c)?)
            }
            StorageComponent::TropicalFishPatternColor(c) => {
                Component::TropicalFishPatternColor(dye_color_from_u8(c)?)
            }
            StorageComponent::MooshroomVariant(v) => {
                Component::MooshroomVariant(mooshroom_variant_from_u8(v)?)
            }
            StorageComponent::RabbitVariant(v) => {
                Component::RabbitVariant(rabbit_variant_from_u8(v)?)
            }
            StorageComponent::PigVariant(v) => Component::PigVariant(VarInt(v)),
            StorageComponent::CowVariant(v) => Component::CowVariant(VarInt(v)),
            StorageComponent::FrogVariant(v) => Component::FrogVariant(VarInt(v)),
            StorageComponent::HorseVariant(v) => {
                Component::HorseVariant(horse_variant_from_u8(v)?)
            }
            StorageComponent::LlamaVariant(v) => {
                Component::LlamaVariant(llama_variant_from_u8(v)?)
            }
            StorageComponent::AxolotlVariant(v) => {
                Component::AxolotlVariant(axolotl_variant_from_u8(v)?)
            }
            StorageComponent::CatVariant(v) => Component::CatVariant(VarInt(v)),
            StorageComponent::CatCollar(c) => Component::CatCollar(dye_color_from_u8(c)?),
            StorageComponent::SheepColor(c) => Component::SheepColor(dye_color_from_u8(c)?),
            StorageComponent::ShulkerColor(c) => Component::ShulkerColor(dye_color_from_u8(c)?),
            StorageComponent::DamageResistant(s) => Component::DamageResistant(s),
            // Complex types that need JSON deserialization - for now return reasonable defaults
            // These can be expanded as needed
            _ => {
                // For complex JSON-serialized components, return a placeholder
                // Real implementation would deserialize the JSON
                return Err(StorageConversionError::UnknownComponent(-1));
            }
        })
    }
}

// ============================================================================
// InventorySlot <-> StorageInventorySlot Conversion
// ============================================================================

impl From<&InventorySlot> for StorageInventorySlot {
    fn from(slot: &InventorySlot) -> Self {
        Self {
            count: slot.count.0,
            item_id: slot.item_id.map(|id| id.0 .0),
            components_to_add: slot.components_to_add.iter().map(Into::into).collect(),
            components_to_remove: slot.components_to_remove.iter().map(|v| v.0).collect(),
        }
    }
}

impl TryFrom<StorageInventorySlot> for InventorySlot {
    type Error = StorageConversionError;

    fn try_from(storage: StorageInventorySlot) -> Result<Self, Self::Error> {
        let components: Result<Vec<_>, _> = storage
            .components_to_add
            .into_iter()
            .map(TryInto::try_into)
            .collect();

        Ok(Self {
            count: VarInt(storage.count),
            item_id: storage.item_id.map(ItemID::new),
            components_to_add: components?,
            components_to_remove: storage.components_to_remove.into_iter().map(VarInt).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_slot_roundtrip() {
        let slot = InventorySlot::new(1, 64);
        let storage = StorageInventorySlot::from(&slot);
        let restored = InventorySlot::try_from(storage).unwrap();

        assert_eq!(slot.count.0, restored.count.0);
        assert_eq!(
            slot.item_id.map(|i| i.0 .0),
            restored.item_id.map(|i| i.0 .0)
        );
    }

    #[test]
    fn test_slot_with_max_stack_roundtrip() {
        let slot = InventorySlot::with_components(1, 64, vec![Component::max_stack_size(99)]);
        let storage = StorageInventorySlot::from(&slot);
        let restored = InventorySlot::try_from(storage).unwrap();

        assert_eq!(restored.components_to_add.len(), 1);
        if let Component::MaxStackSize(v) = &restored.components_to_add[0] {
            assert_eq!(v.0, 99);
        } else {
            panic!("Expected MaxStackSize component");
        }
    }

    #[test]
    fn test_slot_with_custom_name_roundtrip() {
        let slot = InventorySlot::with_components(
            1,
            1,
            vec![Component::custom_name("Test Name")],
        );
        let storage = StorageInventorySlot::from(&slot);
        let restored = InventorySlot::try_from(storage).unwrap();

        assert_eq!(restored.components_to_add.len(), 1);
        assert_eq!(restored.components_to_add[0].id().0, 5); // CustomName ID
    }
}

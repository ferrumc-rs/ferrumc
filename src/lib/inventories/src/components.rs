//! Item component types for the Minecraft protocol.
//!
//! This module implements all item components as defined in the
//! [Minecraft protocol documentation](https://minecraft.wiki/w/Java_Edition_protocol/Slot_data).

use crate::slot::InventorySlot;
use ferrumc_macros::{NetDecode, NetEncode};
use ferrumc_nbt::NBT;
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::id_set::IDSet;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_text::TextComponent;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

// ============================================================================
// Raw NBT Data Wrapper
// ============================================================================

/// A wrapper for raw NBT data that is not parsed.
/// Used for component fields that contain arbitrary NBT structures.
#[derive(Debug, Clone)]
pub struct RawNbt(pub Vec<u8>);

impl NetEncode for RawNbt {
    fn encode<W: Write>(&self, writer: &mut W, _opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        writer.write_all(&self.0)?;
        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        _writer: &mut W,
        _opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        todo!("RawNbt async encoding not yet implemented")
    }
}

impl NetDecode for RawNbt {
    fn decode<R: Read>(_reader: &mut R, _opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        todo!("RawNbt decoding not yet implemented - requires NBT length detection")
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!("RawNbt async decoding not yet implemented")
    }
}

// ============================================================================
// Main Component Enum
// ============================================================================

/// All possible item components in the Minecraft protocol.
/// Each variant corresponds to a component ID as defined in the protocol.
#[derive(Debug, Clone, NetEncode)]
pub enum Component {
    // ID 0
    CustomData(RawNbt),
    // ID 1
    MaxStackSize(VarInt),
    // ID 2
    MaxDamage(VarInt),
    // ID 3
    Damage(VarInt),
    // ID 4
    Unbreakable,
    // ID 5
    CustomName(NBT<TextComponent>),
    // ID 6
    ItemName(NBT<TextComponent>),
    // ID 7
    ItemModel(String),
    // ID 8
    Lore(LengthPrefixedVec<NBT<TextComponent>>),
    // ID 9
    Rarity(Rarity),
    // ID 10
    Enchantments(LengthPrefixedVec<EnchantComponent>),
    // ID 11
    CanPlaceOn(BlockPredicates),
    // ID 12
    CanBreak(BlockPredicates),
    // ID 13
    AttributeModifiers(LengthPrefixedVec<AttributeModifierComponent>),
    // ID 14
    CustomModelData {
        floats: LengthPrefixedVec<f32>,
        flags: LengthPrefixedVec<bool>,
        strings: LengthPrefixedVec<String>,
        colors: LengthPrefixedVec<i32>,
    },
    // ID 15
    TooltipDisplay {
        hide_tooltip: bool,
        hidden_components: LengthPrefixedVec<VarInt>,
    },
    // ID 16
    RepairCost(VarInt),
    // ID 17
    CreativeSlotLock,
    // ID 18
    EnchantmentGlintOverride(bool),
    // ID 19
    IntangibleProjectile(RawNbt),
    // ID 20
    Food {
        nutrition: VarInt,
        saturation_modifier: f32,
        can_always_eat: bool,
    },
    // ID 21
    Consumable {
        consume_seconds: f32,
        animation: ConsumableAnimation,
        sound: IdOrSoundEvent,
        has_particles: bool,
        consume_effects: LengthPrefixedVec<ConsumeEffect>,
    },
    // ID 22
    UseRemainder(InventorySlot),
    // ID 23
    UseCooldown {
        seconds: f32,
        cooldown_group: PrefixedOptional<String>,
    },
    // ID 24
    DamageResistant(String),
    // ID 25
    Tool {
        rules: LengthPrefixedVec<ToolRule>,
        default_mining_speed: f32,
        damage_per_block: VarInt,
        can_destroy_blocks_in_creative: bool,
    },
    // ID 26
    Weapon {
        damage: VarInt,
        disable_blocking_for_seconds: f32,
    },
    // ID 27
    Enchantable(VarInt),
    // ID 28
    Equippable {
        slot: EquippableSlot,
        sound: IdOrSoundEvent,
        model: PrefixedOptional<String>,
        camera_overlay: PrefixedOptional<String>,
        allowed_entities: PrefixedOptional<IDSet>,
        dispensable: bool,
        swappable: bool,
        damage_on_hurt: bool,
    },
    // ID 29
    Repairable(IDSet),
    // ID 30
    Glider,
    // ID 31
    TooltipStyle(String),
    // ID 32
    DeathProtection(LengthPrefixedVec<ConsumeEffect>),
    // ID 33
    BlocksAttacks {
        block_delay_seconds: f32,
        disable_cooldown_scale: f32,
        damage_reductions: LengthPrefixedVec<DamageReduction>,
        item_damage: ItemDamage,
        bypassed_by: PrefixedOptional<String>,
        block_sound: PrefixedOptional<IdOrSoundEvent>,
        disable_sound: PrefixedOptional<IdOrSoundEvent>,
    },
    // ID 34
    StoredEnchantments(LengthPrefixedVec<EnchantComponent>),
    // ID 35
    DyedColor(i32),
    // ID 36
    MapColor(i32),
    // ID 37
    MapId(VarInt),
    // ID 38
    MapDecorations(RawNbt),
    // ID 39
    MapPostProcessing(MapPostProcessing),
    // ID 40
    ChargedProjectiles(LengthPrefixedVec<InventorySlot>),
    // ID 41
    BundleContents(LengthPrefixedVec<InventorySlot>),
    // ID 42
    PotionContents {
        potion_id: PrefixedOptional<VarInt>,
        custom_color: PrefixedOptional<i32>,
        custom_effects: LengthPrefixedVec<PotionEffect>,
        custom_name: String,
    },
    // ID 43
    PotionDurationScale(f32),
    // ID 44
    SuspiciousStewEffects(LengthPrefixedVec<SuspiciousStewEffect>),
    // ID 45
    WritableBookContent(LengthPrefixedVec<WritableBookPage>),
    // ID 46
    WrittenBookContent {
        raw_title: String,
        filtered_title: PrefixedOptional<String>,
        author: String,
        generation: VarInt,
        pages: LengthPrefixedVec<WrittenBookPage>,
        resolved: bool,
    },
    // ID 47
    Trim {
        material: IdOrTrimMaterial,
        pattern: IdOrTrimPattern,
    },
    // ID 48
    DebugStickState(RawNbt),
    // ID 49
    EntityData {
        entity_type: VarInt,
        data: RawNbt,
    },
    // ID 50
    BucketEntityData(RawNbt),
    // ID 51
    BlockEntityData {
        block_entity_type: VarInt,
        data: RawNbt,
    },
    // ID 52
    Instrument(IdOrInstrument),
    // ID 53
    ProvidesTrimMaterial {
        material: IdOrIdentifier,
    },
    // ID 54
    OminousBottleAmplifier(VarInt),
    // ID 55
    JukeboxPlayable {
        song: IdOrJukeboxSong,
    },
    // ID 56
    ProvidesBannerPatterns(String),
    // ID 57
    Recipes(RawNbt),
    // ID 58
    LodestoneTracker {
        /// The global position (dimension + block position), present if the lodestone was tracked
        global_position: PrefixedOptional<GlobalPosition>,
        tracked: bool,
    },
    // ID 59
    FireworkExplosion(FireworkExplosion),
    // ID 60
    Fireworks {
        flight_duration: VarInt,
        explosions: LengthPrefixedVec<FireworkExplosion>,
    },
    // ID 61
    Profile(ResolvableProfile),
    // ID 62
    NoteBlockSound(String),
    // ID 63
    BannerPatterns(LengthPrefixedVec<BannerPatternLayer>),
    // ID 64
    BaseColor(DyeColor),
    // ID 65
    PotDecorations(LengthPrefixedVec<VarInt>),
    // ID 66
    Container(LengthPrefixedVec<InventorySlot>),
    // ID 67
    BlockState(LengthPrefixedVec<BlockStateProperty>),
    // ID 68
    Bees(LengthPrefixedVec<BeeData>),
    // ID 69
    Lock(String),
    // ID 70
    ContainerLoot(RawNbt),
    // ID 71
    BreakSound(IdOrSoundEvent),
    // ID 72
    VillagerVariant(VarInt),
    // ID 73
    WolfVariant(VarInt),
    // ID 74
    WolfSoundVariant(VarInt),
    // ID 75
    WolfCollar(DyeColor),
    // ID 76
    FoxVariant(FoxVariant),
    // ID 77
    SalmonSize(SalmonSize),
    // ID 78
    ParrotVariant(ParrotVariant),
    // ID 79
    TropicalFishPattern(TropicalFishPattern),
    // ID 80
    TropicalFishBaseColor(DyeColor),
    // ID 81
    TropicalFishPatternColor(DyeColor),
    // ID 82
    MooshroomVariant(MooshroomVariant),
    // ID 83
    RabbitVariant(RabbitVariant),
    // ID 84
    PigVariant(VarInt),
    // ID 85
    CowVariant(VarInt),
    // ID 86
    ChickenVariant(IdOrIdentifier),
    // ID 87
    FrogVariant(VarInt),
    // ID 88
    HorseVariant(HorseVariant),
    // ID 89
    PaintingVariant(IdOrPaintingVariant),
    // ID 90
    LlamaVariant(LlamaVariant),
    // ID 91
    AxolotlVariant(AxolotlVariant),
    // ID 92
    CatVariant(VarInt),
    // ID 93
    CatCollar(DyeColor),
    // ID 94
    SheepColor(DyeColor),
    // ID 95
    ShulkerColor(DyeColor),
}

impl Component {
    /// Returns the protocol ID for this component type.
    pub fn id(&self) -> VarInt {
        VarInt(match self {
            Component::CustomData(_) => 0,
            Component::MaxStackSize(_) => 1,
            Component::MaxDamage(_) => 2,
            Component::Damage(_) => 3,
            Component::Unbreakable => 4,
            Component::CustomName(_) => 5,
            Component::ItemName(_) => 6,
            Component::ItemModel(_) => 7,
            Component::Lore(_) => 8,
            Component::Rarity(_) => 9,
            Component::Enchantments(_) => 10,
            Component::CanPlaceOn(_) => 11,
            Component::CanBreak(_) => 12,
            Component::AttributeModifiers(_) => 13,
            Component::CustomModelData { .. } => 14,
            Component::TooltipDisplay { .. } => 15,
            Component::RepairCost(_) => 16,
            Component::CreativeSlotLock => 17,
            Component::EnchantmentGlintOverride(_) => 18,
            Component::IntangibleProjectile(_) => 19,
            Component::Food { .. } => 20,
            Component::Consumable { .. } => 21,
            Component::UseRemainder(_) => 22,
            Component::UseCooldown { .. } => 23,
            Component::DamageResistant(_) => 24,
            Component::Tool { .. } => 25,
            Component::Weapon { .. } => 26,
            Component::Enchantable(_) => 27,
            Component::Equippable { .. } => 28,
            Component::Repairable(_) => 29,
            Component::Glider => 30,
            Component::TooltipStyle(_) => 31,
            Component::DeathProtection(_) => 32,
            Component::BlocksAttacks { .. } => 33,
            Component::StoredEnchantments(_) => 34,
            Component::DyedColor(_) => 35,
            Component::MapColor(_) => 36,
            Component::MapId(_) => 37,
            Component::MapDecorations(_) => 38,
            Component::MapPostProcessing(_) => 39,
            Component::ChargedProjectiles(_) => 40,
            Component::BundleContents(_) => 41,
            Component::PotionContents { .. } => 42,
            Component::PotionDurationScale(_) => 43,
            Component::SuspiciousStewEffects(_) => 44,
            Component::WritableBookContent(_) => 45,
            Component::WrittenBookContent { .. } => 46,
            Component::Trim { .. } => 47,
            Component::DebugStickState(_) => 48,
            Component::EntityData { .. } => 49,
            Component::BucketEntityData(_) => 50,
            Component::BlockEntityData { .. } => 51,
            Component::Instrument(_) => 52,
            Component::ProvidesTrimMaterial { .. } => 53,
            Component::OminousBottleAmplifier(_) => 54,
            Component::JukeboxPlayable { .. } => 55,
            Component::ProvidesBannerPatterns(_) => 56,
            Component::Recipes(_) => 57,
            Component::LodestoneTracker { .. } => 58,
            Component::FireworkExplosion(_) => 59,
            Component::Fireworks { .. } => 60,
            Component::Profile(_) => 61,
            Component::NoteBlockSound(_) => 62,
            Component::BannerPatterns(_) => 63,
            Component::BaseColor(_) => 64,
            Component::PotDecorations(_) => 65,
            Component::Container(_) => 66,
            Component::BlockState(_) => 67,
            Component::Bees(_) => 68,
            Component::Lock(_) => 69,
            Component::ContainerLoot(_) => 70,
            Component::BreakSound(_) => 71,
            Component::VillagerVariant(_) => 72,
            Component::WolfVariant(_) => 73,
            Component::WolfSoundVariant(_) => 74,
            Component::WolfCollar(_) => 75,
            Component::FoxVariant(_) => 76,
            Component::SalmonSize(_) => 77,
            Component::ParrotVariant(_) => 78,
            Component::TropicalFishPattern(_) => 79,
            Component::TropicalFishBaseColor(_) => 80,
            Component::TropicalFishPatternColor(_) => 81,
            Component::MooshroomVariant(_) => 82,
            Component::RabbitVariant(_) => 83,
            Component::PigVariant(_) => 84,
            Component::CowVariant(_) => 85,
            Component::ChickenVariant(_) => 86,
            Component::FrogVariant(_) => 87,
            Component::HorseVariant(_) => 88,
            Component::PaintingVariant(_) => 89,
            Component::LlamaVariant(_) => 90,
            Component::AxolotlVariant(_) => 91,
            Component::CatVariant(_) => 92,
            Component::CatCollar(_) => 93,
            Component::SheepColor(_) => 94,
            Component::ShulkerColor(_) => 95,
        })
    }
}

// ============================================================================
// Helper Enums
// ============================================================================

/// Item rarity levels
#[derive(Debug, Clone, Copy, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum Rarity {
    Common = 0,
    Uncommon = 1,
    Rare = 2,
    Epic = 3,
}

impl NetEncode for Rarity {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode_async(writer, opts).await
    }
}

/// Dye colors used throughout Minecraft
#[derive(Debug, Clone, Copy, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum DyeColor {
    White = 0,
    Orange = 1,
    Magenta = 2,
    LightBlue = 3,
    Yellow = 4,
    Lime = 5,
    Pink = 6,
    Gray = 7,
    LightGray = 8,
    Cyan = 9,
    Purple = 10,
    Blue = 11,
    Brown = 12,
    Green = 13,
    Red = 14,
    Black = 15,
}

impl NetEncode for DyeColor {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode_async(writer, opts).await
    }
}

/// Equipment slot types
#[derive(Debug, Clone, Copy, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum EquippableSlot {
    MainHand = 0,
    Feet = 1,
    Legs = 2,
    Chest = 3,
    Head = 4,
    OffHand = 5,
    Body = 6,
}

impl NetEncode for EquippableSlot {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode_async(writer, opts).await
    }
}

/// Animation types for consumable items
#[derive(Debug, Clone, Copy, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum ConsumableAnimation {
    None = 0,
    Eat = 1,
    Drink = 2,
    Block = 3,
    Bow = 4,
    Spear = 5,
    Crossbow = 6,
    Spyglass = 7,
    TootHorn = 8,
    Brush = 9,
}

impl NetEncode for ConsumableAnimation {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode_async(writer, opts).await
    }
}

/// Attribute modifier operations
#[derive(Debug, Clone, Copy, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum AttributeModifierOperation {
    AddNumber = 0,
    AddPercentage = 1,
    MultiplyPercentage = 2,
}

impl NetEncode for AttributeModifierOperation {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode_async(writer, opts).await
    }
}

/// Attribute modifier slot types
#[derive(Debug, Clone, Copy, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum AttributeModifierSlot {
    Any = 0,
    MainHand = 1,
    OffHand = 2,
    Hand = 3,
    Feet = 4,
    Legs = 5,
    Chest = 6,
    Head = 7,
    Armor = 8,
    Body = 9,
}

impl NetEncode for AttributeModifierSlot {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode_async(writer, opts).await
    }
}

/// Map post-processing types
#[derive(Debug, Clone, Copy, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum MapPostProcessing {
    Lock = 0,
    Scale = 1,
}

impl NetEncode for MapPostProcessing {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode_async(writer, opts).await
    }
}

/// Firework explosion shapes
#[derive(Debug, Clone, Copy, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum FireworkExplosionShape {
    SmallBall = 0,
    LargeBall = 1,
    Star = 2,
    Creeper = 3,
    Burst = 4,
}

impl NetEncode for FireworkExplosionShape {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode_async(writer, opts).await
    }
}

/// Fox variants
#[derive(Debug, Clone, Copy, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum FoxVariant {
    Red = 0,
    Snow = 1,
}

impl NetEncode for FoxVariant {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode_async(writer, opts).await
    }
}

/// Salmon sizes
#[derive(Debug, Clone, Copy, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum SalmonSize {
    Small = 0,
    Medium = 1,
    Large = 2,
}

impl NetEncode for SalmonSize {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode_async(writer, opts).await
    }
}

/// Parrot variants
#[derive(Debug, Clone, Copy, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum ParrotVariant {
    RedBlue = 0,
    Blue = 1,
    Green = 2,
    YellowBlue = 3,
    Gray = 4,
}

impl NetEncode for ParrotVariant {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode_async(writer, opts).await
    }
}

/// Tropical fish patterns
#[derive(Debug, Clone, Copy, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum TropicalFishPattern {
    Kob = 0,
    Sunstreak = 1,
    Snooper = 2,
    Dasher = 3,
    Brinely = 4,
    Spotty = 5,
    Flopper = 6,
    Stripey = 7,
    Glitter = 8,
    Blockfish = 9,
    Betty = 10,
    Clayfish = 11,
}

impl NetEncode for TropicalFishPattern {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode_async(writer, opts).await
    }
}

/// Mooshroom variants
#[derive(Debug, Clone, Copy, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum MooshroomVariant {
    Red = 0,
    Brown = 1,
}

impl NetEncode for MooshroomVariant {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode_async(writer, opts).await
    }
}

/// Rabbit variants
#[derive(Debug, Clone, Copy, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum RabbitVariant {
    Brown = 0,
    White = 1,
    Black = 2,
    BlackAndWhite = 3,
    Gold = 4,
    SaltAndPepper = 5,
    Evil = 6,
}

impl NetEncode for RabbitVariant {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode_async(writer, opts).await
    }
}

/// Horse variants
#[derive(Debug, Clone, Copy, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum HorseVariant {
    White = 0,
    Creamy = 1,
    Chestnut = 2,
    Brown = 3,
    Black = 4,
    Gray = 5,
    DarkBrown = 6,
}

impl NetEncode for HorseVariant {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode_async(writer, opts).await
    }
}

/// Llama variants
#[derive(Debug, Clone, Copy, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum LlamaVariant {
    Creamy = 0,
    White = 1,
    Brown = 2,
    Gray = 3,
}

impl NetEncode for LlamaVariant {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode_async(writer, opts).await
    }
}

/// Axolotl variants
#[derive(Debug, Clone, Copy, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum AxolotlVariant {
    Lucy = 0,
    Wild = 1,
    Gold = 2,
    Cyan = 3,
    Blue = 4,
}

impl NetEncode for AxolotlVariant {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt(*self as i32).encode_async(writer, opts).await
    }
}

// ============================================================================
// Helper Structs
// ============================================================================

/// Enchantment data
#[derive(Debug, Clone, NetEncode, NetDecode)]
pub struct EnchantComponent {
    pub id: VarInt,
    pub level: VarInt,
}

/// Attribute modifier data
#[derive(Debug, Clone, NetEncode, NetDecode)]
pub struct AttributeModifierComponent {
    pub attribute_id: VarInt,
    pub modifier_id: String,
    pub value: f64,
    pub operation: AttributeModifierOperation,
    pub slot: AttributeModifierSlot,
}

/// Tool rule for mining behavior
#[derive(Debug, Clone, NetEncode, NetDecode)]
pub struct ToolRule {
    pub blocks: IDSet,
    pub speed: PrefixedOptional<f32>,
    pub correct_drop: PrefixedOptional<bool>,
}

/// Sound event with optional fixed range
#[derive(Debug, Clone, NetEncode, NetDecode)]
pub struct SoundEvent {
    pub sound_name: String,
    pub fixed_range: PrefixedOptional<f32>,
}

/// Either a registry ID or an inline sound event.
/// Encoded as VarInt where 0 means inline, otherwise (id + 1).
#[derive(Debug, Clone, NetEncode)]
pub enum IdOrSoundEvent {
    Id(VarInt),
    Inline(SoundEvent),
}

/// Block predicates for can_place_on and can_break
#[derive(Debug, Clone, NetEncode, NetDecode)]
pub struct BlockPredicates {
    pub predicates: LengthPrefixedVec<BlockPredicate>,
}

/// A single block predicate
#[derive(Debug, Clone, NetEncode)]
pub struct BlockPredicate {
    pub blocks: PrefixedOptional<IDSet>,
    pub properties: PrefixedOptional<LengthPrefixedVec<BlockPropertyMatcher>>,
    pub nbt: PrefixedOptional<RawNbt>,
    /// Exact data component matchers (component type ID + component value)
    pub data_components: LengthPrefixedVec<DataComponentMatcher>,
    /// Partial data component predicates (max 64)
    pub partial_predicates: LengthPrefixedVec<PartialDataComponentPredicate>,
}

/// Exact data component matcher for block predicates
#[derive(Debug, Clone, NetEncode, NetDecode)]
pub struct DataComponentMatcher {
    /// The component type ID
    pub component_type: VarInt,
    /// The component value as raw NBT (full component data)
    pub value: RawNbt,
}

/// Partial data component predicate for block predicates
#[derive(Debug, Clone, NetEncode, NetDecode)]
pub struct PartialDataComponentPredicate {
    /// The component type ID
    pub component_type: VarInt,
    /// Partial NBT data to match against
    pub predicate: RawNbt,
}

/// Block property matcher
#[derive(Debug, Clone, NetEncode, NetDecode)]
pub struct BlockPropertyMatcher {
    pub name: String,
    pub is_exact: bool,
    pub exact_value: PrefixedOptional<String>,
    pub min_value: PrefixedOptional<String>,
    pub max_value: PrefixedOptional<String>,
}

/// Damage reduction entry for shields
#[derive(Debug, Clone, NetEncode, NetDecode)]
pub struct DamageReduction {
    pub horizontal_blocking_angle: f32,
    pub type_predicate: PrefixedOptional<IDSet>,
    pub base: f32,
    pub factor: f32,
}

/// Item damage configuration
#[derive(Debug, Clone, NetEncode, NetDecode)]
pub struct ItemDamage {
    pub threshold: f32,
    pub base: f32,
    pub factor: f32,
}

/// Potion effect data
#[derive(Debug, Clone)]
pub struct PotionEffect {
    pub effect_id: VarInt,
    pub amplifier: VarInt,
    pub duration: VarInt,
    pub ambient: bool,
    pub show_particles: bool,
    pub show_icon: bool,
    pub hidden_effect: Option<Box<PotionEffect>>,
}

impl NetEncode for PotionEffect {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.effect_id.encode(writer, opts)?;
        self.amplifier.encode(writer, opts)?;
        self.duration.encode(writer, opts)?;
        self.ambient.encode(writer, opts)?;
        self.show_particles.encode(writer, opts)?;
        self.show_icon.encode(writer, opts)?;
        match &self.hidden_effect {
            Some(effect) => {
                true.encode(writer, opts)?;
                effect.encode(writer, opts)?;
            }
            None => {
                false.encode(writer, opts)?;
            }
        }
        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        _writer: &mut W,
        _opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        todo!()
    }
}

impl NetDecode for PotionEffect {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let effect_id = VarInt::decode(reader, opts)?;
        let amplifier = VarInt::decode(reader, opts)?;
        let duration = VarInt::decode(reader, opts)?;
        let ambient = bool::decode(reader, opts)?;
        let show_particles = bool::decode(reader, opts)?;
        let show_icon = bool::decode(reader, opts)?;
        let has_hidden = bool::decode(reader, opts)?;
        let hidden_effect = if has_hidden {
            Some(Box::new(PotionEffect::decode(reader, opts)?))
        } else {
            None
        };
        Ok(PotionEffect {
            effect_id,
            amplifier,
            duration,
            ambient,
            show_particles,
            show_icon,
            hidden_effect,
        })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!()
    }
}

/// Suspicious stew effect
#[derive(Debug, Clone, NetEncode, NetDecode)]
pub struct SuspiciousStewEffect {
    pub effect_id: VarInt,
    pub duration: VarInt,
}

/// Writable book page
#[derive(Debug, Clone, NetEncode, NetDecode)]
pub struct WritableBookPage {
    pub raw_content: String,
    pub filtered_content: PrefixedOptional<String>,
}

/// Written book page with text components
#[derive(Debug, Clone, NetEncode)]
pub struct WrittenBookPage {
    pub raw_content: NBT<TextComponent>,
    pub filtered_content: PrefixedOptional<NBT<TextComponent>>,
}

/// Firework explosion data
#[derive(Debug, Clone, NetEncode, NetDecode)]
pub struct FireworkExplosion {
    pub shape: FireworkExplosionShape,
    pub colors: LengthPrefixedVec<i32>,
    pub fade_colors: LengthPrefixedVec<i32>,
    pub has_trail: bool,
    pub has_twinkle: bool,
}

/// Resolvable game profile
#[derive(Debug, Clone, NetEncode, NetDecode)]
pub struct ResolvableProfile {
    pub name: PrefixedOptional<String>,
    pub uuid: PrefixedOptional<u128>,
    pub properties: LengthPrefixedVec<ProfileProperty>,
}

/// Profile property
#[derive(Debug, Clone, NetEncode, NetDecode)]
pub struct ProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: PrefixedOptional<String>,
}

/// Banner pattern layer
#[derive(Debug, Clone, NetEncode)]
pub struct BannerPatternLayer {
    pub pattern: IdOrBannerPattern,
    pub color: DyeColor,
}

/// Either a registry ID or inline banner pattern.
/// Encoded as VarInt where 0 means inline, otherwise (id + 1).
#[derive(Debug, Clone, NetEncode)]
pub enum IdOrBannerPattern {
    Id(VarInt),
    Inline {
        asset_id: String,
        translation_key: String,
    },
}

/// Block state property
#[derive(Debug, Clone, NetEncode, NetDecode)]
pub struct BlockStateProperty {
    pub name: String,
    pub value: String,
}

/// Global position for lodestone tracking (dimension + block position)
#[derive(Debug, Clone, NetEncode, NetDecode)]
pub struct GlobalPosition {
    pub dimension: String,
    pub position: NetworkPosition,
}

/// Bee data for beehives
#[derive(Debug, Clone, NetEncode)]
pub struct BeeData {
    /// Entity type registry ID
    pub entity_type: VarInt,
    /// NBT data for the bee entity
    pub entity_data: RawNbt,
    pub ticks_in_hive: VarInt,
    pub min_ticks_in_hive: VarInt,
}

/// Either a registry ID or an identifier string.
/// Mode byte: 0 = identifier, 1 = registry ID.
#[derive(Debug, Clone, NetEncode)]
pub enum IdOrIdentifier {
    Identifier(String),
    Id(VarInt),
}

/// Either a registry ID or inline trim material.
/// Encoded as VarInt where 0 means inline, otherwise (id + 1).
#[derive(Debug, Clone, NetEncode)]
pub enum IdOrTrimMaterial {
    Id(VarInt),
    Inline(TrimMaterial),
}

/// Inline trim material data
#[derive(Debug, Clone, NetEncode)]
pub struct TrimMaterial {
    pub asset_name: String,
    pub ingredient: VarInt,
    pub item_model_index: f32,
    pub override_armor_materials: LengthPrefixedVec<ArmorMaterialOverride>,
    pub description: NBT<TextComponent>,
}

/// Armor material override
#[derive(Debug, Clone, NetEncode, NetDecode)]
pub struct ArmorMaterialOverride {
    pub armor_material: String,
    pub override_asset_name: String,
}

/// Either a registry ID or inline trim pattern.
/// Encoded as VarInt where 0 means inline, otherwise (id + 1).
#[derive(Debug, Clone, NetEncode)]
pub enum IdOrTrimPattern {
    Id(VarInt),
    Inline(TrimPattern),
}

/// Inline trim pattern data
#[derive(Debug, Clone, NetEncode)]
pub struct TrimPattern {
    pub asset_id: String,
    pub template_item: VarInt,
    pub description: NBT<TextComponent>,
    pub decal: bool,
}

/// Either a registry ID or inline instrument.
/// Encoded as VarInt where 0 means inline, otherwise (id + 1).
#[derive(Debug, Clone, NetEncode)]
pub enum IdOrInstrument {
    Id(VarInt),
    Inline(Instrument),
}

/// Inline instrument data
#[derive(Debug, Clone, NetEncode)]
pub struct Instrument {
    pub sound_event: IdOrSoundEvent,
    pub use_duration: f32,
    pub range: f32,
    pub description: NBT<TextComponent>,
}

/// Either a registry ID or inline jukebox song.
/// Encoded as VarInt where 0 means inline, otherwise (id + 1).
#[derive(Debug, Clone, NetEncode)]
pub enum IdOrJukeboxSong {
    Id(VarInt),
    Inline(JukeboxSong),
}

/// Inline jukebox song data
#[derive(Debug, Clone, NetEncode)]
pub struct JukeboxSong {
    pub sound_event: IdOrSoundEvent,
    pub description: NBT<TextComponent>,
    pub length_in_seconds: f32,
    pub comparator_output: VarInt,
}

/// Either a registry ID or inline painting variant.
/// Encoded as VarInt where 0 means inline, otherwise (id + 1).
#[derive(Debug, Clone, NetEncode)]
pub enum IdOrPaintingVariant {
    Id(VarInt),
    Inline(PaintingVariant),
}

/// Inline painting variant data
#[derive(Debug, Clone, NetEncode)]
pub struct PaintingVariant {
    pub width: VarInt,
    pub height: VarInt,
    pub asset_id: String,
    pub title: PrefixedOptional<NBT<TextComponent>>,
    pub author: PrefixedOptional<NBT<TextComponent>>,
}

// ============================================================================
// Consume Effects
// ============================================================================

/// Effects that can occur when consuming an item
#[derive(Debug, Clone, NetEncode)]
pub enum ConsumeEffect {
    // Type 0
    ApplyEffects {
        effects: LengthPrefixedVec<ConsumeEffectEntry>,
        probability: f32,
    },
    // Type 1
    RemoveEffects(IDSet),
    // Type 2
    ClearAllEffects,
    // Type 3
    TeleportRandomly(f32),
    // Type 4
    PlaySound(IdOrSoundEvent),
}

/// A single effect entry for consume effects
#[derive(Debug, Clone, NetEncode, NetDecode)]
pub struct ConsumeEffectEntry {
    pub effect: PotionEffect,
}

// ============================================================================
// NetDecode Implementation for Component
// ============================================================================

impl NetDecode for Component {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let component_id = VarInt::decode(reader, opts)?;
        match component_id.0 {
            // ID 0: CustomData
            0 => Ok(Component::CustomData(RawNbt::decode(reader, opts)?)),
            // ID 1: MaxStackSize
            1 => Ok(Component::MaxStackSize(VarInt::decode(reader, opts)?)),
            // ID 2: MaxDamage
            2 => Ok(Component::MaxDamage(VarInt::decode(reader, opts)?)),
            // ID 3: Damage
            3 => Ok(Component::Damage(VarInt::decode(reader, opts)?)),
            // ID 4: Unbreakable
            4 => Ok(Component::Unbreakable),
            // ID 5: CustomName (requires NBT<TextComponent> decode - not yet supported)
            5 => todo!("CustomName decoding requires FromNbt for TextComponent"),
            // ID 6: ItemName (requires NBT<TextComponent> decode - not yet supported)
            6 => todo!("ItemName decoding requires FromNbt for TextComponent"),
            // ID 7: ItemModel
            7 => Ok(Component::ItemModel(String::decode(reader, opts)?)),
            // ID 8: Lore (requires NBT<TextComponent> decode - not yet supported)
            8 => todo!("Lore decoding requires FromNbt for TextComponent"),
            // ID 9: Rarity
            9 => Ok(Component::Rarity(Rarity::decode(reader, opts)?)),
            // ID 10: Enchantments
            10 => Ok(Component::Enchantments(LengthPrefixedVec::decode(reader, opts)?)),
            // ID 11: CanPlaceOn
            11 => Ok(Component::CanPlaceOn(BlockPredicates::decode(reader, opts)?)),
            // ID 12: CanBreak
            12 => Ok(Component::CanBreak(BlockPredicates::decode(reader, opts)?)),
            // ID 13: AttributeModifiers
            13 => Ok(Component::AttributeModifiers(LengthPrefixedVec::decode(reader, opts)?)),
            // ID 14: CustomModelData
            14 => Ok(Component::CustomModelData {
                floats: LengthPrefixedVec::decode(reader, opts)?,
                flags: LengthPrefixedVec::decode(reader, opts)?,
                strings: LengthPrefixedVec::decode(reader, opts)?,
                colors: LengthPrefixedVec::decode(reader, opts)?,
            }),
            // ID 15: TooltipDisplay
            15 => Ok(Component::TooltipDisplay {
                hide_tooltip: bool::decode(reader, opts)?,
                hidden_components: LengthPrefixedVec::decode(reader, opts)?,
            }),
            // ID 16: RepairCost
            16 => Ok(Component::RepairCost(VarInt::decode(reader, opts)?)),
            // ID 17: CreativeSlotLock
            17 => Ok(Component::CreativeSlotLock),
            // ID 18: EnchantmentGlintOverride
            18 => Ok(Component::EnchantmentGlintOverride(bool::decode(reader, opts)?)),
            // ID 19: IntangibleProjectile
            19 => Ok(Component::IntangibleProjectile(RawNbt::decode(reader, opts)?)),
            // ID 20: Food
            20 => Ok(Component::Food {
                nutrition: VarInt::decode(reader, opts)?,
                saturation_modifier: f32::decode(reader, opts)?,
                can_always_eat: bool::decode(reader, opts)?,
            }),
            // ID 21: Consumable
            21 => Ok(Component::Consumable {
                consume_seconds: f32::decode(reader, opts)?,
                animation: ConsumableAnimation::decode(reader, opts)?,
                sound: IdOrSoundEvent::decode(reader, opts)?,
                has_particles: bool::decode(reader, opts)?,
                consume_effects: LengthPrefixedVec::decode(reader, opts)?,
            }),
            // ID 22: UseRemainder
            22 => Ok(Component::UseRemainder(InventorySlot::decode(reader, opts)?)),
            // ID 23: UseCooldown
            23 => Ok(Component::UseCooldown {
                seconds: f32::decode(reader, opts)?,
                cooldown_group: PrefixedOptional::decode(reader, opts)?,
            }),
            // ID 24: DamageResistant
            24 => Ok(Component::DamageResistant(String::decode(reader, opts)?)),
            // ID 25: Tool
            25 => Ok(Component::Tool {
                rules: LengthPrefixedVec::decode(reader, opts)?,
                default_mining_speed: f32::decode(reader, opts)?,
                damage_per_block: VarInt::decode(reader, opts)?,
                can_destroy_blocks_in_creative: bool::decode(reader, opts)?,
            }),
            // ID 26: Weapon
            26 => Ok(Component::Weapon {
                damage: VarInt::decode(reader, opts)?,
                disable_blocking_for_seconds: f32::decode(reader, opts)?,
            }),
            // ID 27: Enchantable
            27 => Ok(Component::Enchantable(VarInt::decode(reader, opts)?)),
            // ID 28: Equippable
            28 => Ok(Component::Equippable {
                slot: EquippableSlot::decode(reader, opts)?,
                sound: IdOrSoundEvent::decode(reader, opts)?,
                model: PrefixedOptional::decode(reader, opts)?,
                camera_overlay: PrefixedOptional::decode(reader, opts)?,
                allowed_entities: PrefixedOptional::decode(reader, opts)?,
                dispensable: bool::decode(reader, opts)?,
                swappable: bool::decode(reader, opts)?,
                damage_on_hurt: bool::decode(reader, opts)?,
            }),
            // ID 29: Repairable
            29 => Ok(Component::Repairable(IDSet::decode(reader, opts)?)),
            // ID 30: Glider
            30 => Ok(Component::Glider),
            // ID 31: TooltipStyle
            31 => Ok(Component::TooltipStyle(String::decode(reader, opts)?)),
            // ID 32: DeathProtection
            32 => Ok(Component::DeathProtection(LengthPrefixedVec::decode(reader, opts)?)),
            // ID 33: BlocksAttacks
            33 => Ok(Component::BlocksAttacks {
                block_delay_seconds: f32::decode(reader, opts)?,
                disable_cooldown_scale: f32::decode(reader, opts)?,
                damage_reductions: LengthPrefixedVec::decode(reader, opts)?,
                item_damage: ItemDamage::decode(reader, opts)?,
                bypassed_by: PrefixedOptional::decode(reader, opts)?,
                block_sound: PrefixedOptional::decode(reader, opts)?,
                disable_sound: PrefixedOptional::decode(reader, opts)?,
            }),
            // ID 34: StoredEnchantments
            34 => Ok(Component::StoredEnchantments(LengthPrefixedVec::decode(reader, opts)?)),
            // ID 35: DyedColor
            35 => Ok(Component::DyedColor(i32::decode(reader, opts)?)),
            // ID 36: MapColor
            36 => Ok(Component::MapColor(i32::decode(reader, opts)?)),
            // ID 37: MapId
            37 => Ok(Component::MapId(VarInt::decode(reader, opts)?)),
            // ID 38: MapDecorations
            38 => Ok(Component::MapDecorations(RawNbt::decode(reader, opts)?)),
            // ID 39: MapPostProcessing
            39 => Ok(Component::MapPostProcessing(MapPostProcessing::decode(reader, opts)?)),
            // ID 40: ChargedProjectiles
            40 => Ok(Component::ChargedProjectiles(LengthPrefixedVec::decode(reader, opts)?)),
            // ID 41: BundleContents
            41 => Ok(Component::BundleContents(LengthPrefixedVec::decode(reader, opts)?)),
            // ID 42: PotionContents
            42 => Ok(Component::PotionContents {
                potion_id: PrefixedOptional::decode(reader, opts)?,
                custom_color: PrefixedOptional::decode(reader, opts)?,
                custom_effects: LengthPrefixedVec::decode(reader, opts)?,
                custom_name: String::decode(reader, opts)?,
            }),
            // ID 43: PotionDurationScale
            43 => Ok(Component::PotionDurationScale(f32::decode(reader, opts)?)),
            // ID 44: SuspiciousStewEffects
            44 => Ok(Component::SuspiciousStewEffects(LengthPrefixedVec::decode(reader, opts)?)),
            // ID 45: WritableBookContent
            45 => Ok(Component::WritableBookContent(LengthPrefixedVec::decode(reader, opts)?)),
            // ID 46: WrittenBookContent (requires NBT<TextComponent> decode - not yet supported)
            46 => todo!("WrittenBookContent decoding requires FromNbt for TextComponent"),
            // ID 47: Trim
            47 => Ok(Component::Trim {
                material: IdOrTrimMaterial::decode(reader, opts)?,
                pattern: IdOrTrimPattern::decode(reader, opts)?,
            }),
            // ID 48: DebugStickState
            48 => Ok(Component::DebugStickState(RawNbt::decode(reader, opts)?)),
            // ID 49: EntityData
            49 => Ok(Component::EntityData {
                entity_type: VarInt::decode(reader, opts)?,
                data: RawNbt::decode(reader, opts)?,
            }),
            // ID 50: BucketEntityData
            50 => Ok(Component::BucketEntityData(RawNbt::decode(reader, opts)?)),
            // ID 51: BlockEntityData
            51 => Ok(Component::BlockEntityData {
                block_entity_type: VarInt::decode(reader, opts)?,
                data: RawNbt::decode(reader, opts)?,
            }),
            // ID 52: Instrument
            52 => Ok(Component::Instrument(IdOrInstrument::decode(reader, opts)?)),
            // ID 53: ProvidesTrimMaterial
            53 => Ok(Component::ProvidesTrimMaterial {
                material: IdOrIdentifier::decode(reader, opts)?,
            }),
            // ID 54: OminousBottleAmplifier
            54 => Ok(Component::OminousBottleAmplifier(VarInt::decode(reader, opts)?)),
            // ID 55: JukeboxPlayable
            55 => Ok(Component::JukeboxPlayable {
                song: IdOrJukeboxSong::decode(reader, opts)?,
            }),
            // ID 56: ProvidesBannerPatterns
            56 => Ok(Component::ProvidesBannerPatterns(String::decode(reader, opts)?)),
            // ID 57: Recipes
            57 => Ok(Component::Recipes(RawNbt::decode(reader, opts)?)),
            // ID 58: LodestoneTracker
            58 => Ok(Component::LodestoneTracker {
                global_position: PrefixedOptional::decode(reader, opts)?,
                tracked: bool::decode(reader, opts)?,
            }),
            // ID 59: FireworkExplosion
            59 => Ok(Component::FireworkExplosion(FireworkExplosion::decode(reader, opts)?)),
            // ID 60: Fireworks
            60 => Ok(Component::Fireworks {
                flight_duration: VarInt::decode(reader, opts)?,
                explosions: LengthPrefixedVec::decode(reader, opts)?,
            }),
            // ID 61: Profile
            61 => Ok(Component::Profile(ResolvableProfile::decode(reader, opts)?)),
            // ID 62: NoteBlockSound
            62 => Ok(Component::NoteBlockSound(String::decode(reader, opts)?)),
            // ID 63: BannerPatterns
            63 => Ok(Component::BannerPatterns(LengthPrefixedVec::decode(reader, opts)?)),
            // ID 64: BaseColor
            64 => Ok(Component::BaseColor(DyeColor::decode(reader, opts)?)),
            // ID 65: PotDecorations
            65 => Ok(Component::PotDecorations(LengthPrefixedVec::decode(reader, opts)?)),
            // ID 66: Container
            66 => Ok(Component::Container(LengthPrefixedVec::decode(reader, opts)?)),
            // ID 67: BlockState
            67 => Ok(Component::BlockState(LengthPrefixedVec::decode(reader, opts)?)),
            // ID 68: Bees
            68 => Ok(Component::Bees(LengthPrefixedVec::decode(reader, opts)?)),
            // ID 69: Lock
            69 => Ok(Component::Lock(String::decode(reader, opts)?)),
            // ID 70: ContainerLoot
            70 => Ok(Component::ContainerLoot(RawNbt::decode(reader, opts)?)),
            // ID 71: BreakSound
            71 => Ok(Component::BreakSound(IdOrSoundEvent::decode(reader, opts)?)),
            // ID 72: VillagerVariant
            72 => Ok(Component::VillagerVariant(VarInt::decode(reader, opts)?)),
            // ID 73: WolfVariant
            73 => Ok(Component::WolfVariant(VarInt::decode(reader, opts)?)),
            // ID 74: WolfSoundVariant
            74 => Ok(Component::WolfSoundVariant(VarInt::decode(reader, opts)?)),
            // ID 75: WolfCollar
            75 => Ok(Component::WolfCollar(DyeColor::decode(reader, opts)?)),
            // ID 76: FoxVariant
            76 => Ok(Component::FoxVariant(FoxVariant::decode(reader, opts)?)),
            // ID 77: SalmonSize
            77 => Ok(Component::SalmonSize(SalmonSize::decode(reader, opts)?)),
            // ID 78: ParrotVariant
            78 => Ok(Component::ParrotVariant(ParrotVariant::decode(reader, opts)?)),
            // ID 79: TropicalFishPattern
            79 => Ok(Component::TropicalFishPattern(TropicalFishPattern::decode(reader, opts)?)),
            // ID 80: TropicalFishBaseColor
            80 => Ok(Component::TropicalFishBaseColor(DyeColor::decode(reader, opts)?)),
            // ID 81: TropicalFishPatternColor
            81 => Ok(Component::TropicalFishPatternColor(DyeColor::decode(reader, opts)?)),
            // ID 82: MooshroomVariant
            82 => Ok(Component::MooshroomVariant(MooshroomVariant::decode(reader, opts)?)),
            // ID 83: RabbitVariant
            83 => Ok(Component::RabbitVariant(RabbitVariant::decode(reader, opts)?)),
            // ID 84: PigVariant
            84 => Ok(Component::PigVariant(VarInt::decode(reader, opts)?)),
            // ID 85: CowVariant
            85 => Ok(Component::CowVariant(VarInt::decode(reader, opts)?)),
            // ID 86: ChickenVariant
            86 => Ok(Component::ChickenVariant(IdOrIdentifier::decode(reader, opts)?)),
            // ID 87: FrogVariant
            87 => Ok(Component::FrogVariant(VarInt::decode(reader, opts)?)),
            // ID 88: HorseVariant
            88 => Ok(Component::HorseVariant(HorseVariant::decode(reader, opts)?)),
            // ID 89: PaintingVariant
            89 => Ok(Component::PaintingVariant(IdOrPaintingVariant::decode(reader, opts)?)),
            // ID 90: LlamaVariant
            90 => Ok(Component::LlamaVariant(LlamaVariant::decode(reader, opts)?)),
            // ID 91: AxolotlVariant
            91 => Ok(Component::AxolotlVariant(AxolotlVariant::decode(reader, opts)?)),
            // ID 92: CatVariant
            92 => Ok(Component::CatVariant(VarInt::decode(reader, opts)?)),
            // ID 93: CatCollar
            93 => Ok(Component::CatCollar(DyeColor::decode(reader, opts)?)),
            // ID 94: SheepColor
            94 => Ok(Component::SheepColor(DyeColor::decode(reader, opts)?)),
            // ID 95: ShulkerColor
            95 => Ok(Component::ShulkerColor(DyeColor::decode(reader, opts)?)),
            // Unknown component ID
            _ => Err(NetDecodeError::InvalidEnumVariant),
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!("Async component decoding not yet implemented")
    }
}

// ============================================================================
// NetDecode implementations for enum variants with non-trivial encoding
// ============================================================================

impl NetDecode for IdOrSoundEvent {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let id = VarInt::decode(reader, opts)?;
        if id.0 == 0 {
            Ok(IdOrSoundEvent::Inline(SoundEvent::decode(reader, opts)?))
        } else {
            Ok(IdOrSoundEvent::Id(VarInt(id.0 - 1)))
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!()
    }
}

impl NetDecode for IdOrBannerPattern {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let id = VarInt::decode(reader, opts)?;
        if id.0 == 0 {
            let asset_id = String::decode(reader, opts)?;
            let translation_key = String::decode(reader, opts)?;
            Ok(IdOrBannerPattern::Inline {
                asset_id,
                translation_key,
            })
        } else {
            Ok(IdOrBannerPattern::Id(VarInt(id.0 - 1)))
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!()
    }
}

impl NetDecode for BannerPatternLayer {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let pattern = IdOrBannerPattern::decode(reader, opts)?;
        let color = DyeColor::decode(reader, opts)?;
        Ok(BannerPatternLayer { pattern, color })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!()
    }
}

impl NetDecode for IdOrIdentifier {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let mode = u8::decode(reader, opts)?;
        match mode {
            0 => Ok(IdOrIdentifier::Identifier(String::decode(reader, opts)?)),
            1 => Ok(IdOrIdentifier::Id(VarInt::decode(reader, opts)?)),
            _ => Err(NetDecodeError::InvalidEnumVariant),
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!()
    }
}

impl NetDecode for IdOrTrimMaterial {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let id = VarInt::decode(reader, opts)?;
        if id.0 == 0 {
            Ok(IdOrTrimMaterial::Inline(TrimMaterial::decode(reader, opts)?))
        } else {
            Ok(IdOrTrimMaterial::Id(VarInt(id.0 - 1)))
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!()
    }
}

impl NetDecode for IdOrTrimPattern {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let id = VarInt::decode(reader, opts)?;
        if id.0 == 0 {
            Ok(IdOrTrimPattern::Inline(TrimPattern::decode(reader, opts)?))
        } else {
            Ok(IdOrTrimPattern::Id(VarInt(id.0 - 1)))
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!()
    }
}

impl NetDecode for IdOrInstrument {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let id = VarInt::decode(reader, opts)?;
        if id.0 == 0 {
            Ok(IdOrInstrument::Inline(Instrument::decode(reader, opts)?))
        } else {
            Ok(IdOrInstrument::Id(VarInt(id.0 - 1)))
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!()
    }
}

impl NetDecode for IdOrJukeboxSong {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let id = VarInt::decode(reader, opts)?;
        if id.0 == 0 {
            Ok(IdOrJukeboxSong::Inline(JukeboxSong::decode(reader, opts)?))
        } else {
            Ok(IdOrJukeboxSong::Id(VarInt(id.0 - 1)))
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!()
    }
}

impl NetDecode for IdOrPaintingVariant {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let id = VarInt::decode(reader, opts)?;
        if id.0 == 0 {
            Ok(IdOrPaintingVariant::Inline(PaintingVariant::decode(reader, opts)?))
        } else {
            Ok(IdOrPaintingVariant::Id(VarInt(id.0 - 1)))
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!()
    }
}

impl NetDecode for ConsumeEffect {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let type_id = VarInt::decode(reader, opts)?;
        match type_id.0 {
            0 => {
                let effects = LengthPrefixedVec::decode(reader, opts)?;
                let probability = f32::decode(reader, opts)?;
                Ok(ConsumeEffect::ApplyEffects {
                    effects,
                    probability,
                })
            }
            1 => Ok(ConsumeEffect::RemoveEffects(IDSet::decode(reader, opts)?)),
            2 => Ok(ConsumeEffect::ClearAllEffects),
            3 => Ok(ConsumeEffect::TeleportRandomly(f32::decode(reader, opts)?)),
            4 => Ok(ConsumeEffect::PlaySound(IdOrSoundEvent::decode(
                reader, opts,
            )?)),
            _ => Err(NetDecodeError::InvalidEnumVariant),
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!()
    }
}

impl NetDecode for WrittenBookPage {
    fn decode<R: Read>(_reader: &mut R, _opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        // Requires NBT<TextComponent> decode which needs FromNbt for TextComponent
        todo!("WrittenBookPage decoding requires FromNbt for TextComponent")
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!()
    }
}

impl NetDecode for BeeData {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        Ok(BeeData {
            entity_type: VarInt::decode(reader, opts)?,
            entity_data: RawNbt::decode(reader, opts)?,
            ticks_in_hive: VarInt::decode(reader, opts)?,
            min_ticks_in_hive: VarInt::decode(reader, opts)?,
        })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!()
    }
}

impl NetDecode for BlockPredicate {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        Ok(BlockPredicate {
            blocks: PrefixedOptional::decode(reader, opts)?,
            properties: PrefixedOptional::decode(reader, opts)?,
            nbt: PrefixedOptional::decode(reader, opts)?,
            data_components: LengthPrefixedVec::decode(reader, opts)?,
            partial_predicates: LengthPrefixedVec::decode(reader, opts)?,
        })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!()
    }
}

impl NetDecode for TrimMaterial {
    fn decode<R: Read>(_reader: &mut R, _opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        // Requires NBT<TextComponent> decode which needs FromNbt for TextComponent
        todo!("TrimMaterial decoding requires FromNbt for TextComponent")
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!()
    }
}

impl NetDecode for TrimPattern {
    fn decode<R: Read>(_reader: &mut R, _opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        // Requires NBT<TextComponent> decode which needs FromNbt for TextComponent
        todo!("TrimPattern decoding requires FromNbt for TextComponent")
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!()
    }
}

impl NetDecode for Instrument {
    fn decode<R: Read>(_reader: &mut R, _opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        // Requires NBT<TextComponent> decode which needs FromNbt for TextComponent
        todo!("Instrument decoding requires FromNbt for TextComponent")
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!()
    }
}

impl NetDecode for JukeboxSong {
    fn decode<R: Read>(_reader: &mut R, _opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        // Requires NBT<TextComponent> decode which needs FromNbt for TextComponent
        todo!("JukeboxSong decoding requires FromNbt for TextComponent")
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!()
    }
}

impl NetDecode for PaintingVariant {
    fn decode<R: Read>(_reader: &mut R, _opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        // Requires NBT<TextComponent> decode which needs FromNbt for TextComponent
        todo!("PaintingVariant decoding requires FromNbt for TextComponent")
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!()
    }
}

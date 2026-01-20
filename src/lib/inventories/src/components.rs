use crate::slot::InventorySlot;
use ferrumc_macros::{NetDecode, NetEncode};
use ferrumc_nbt::NBT;
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::net_types::id_set::IDSet;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_text::TextComponent;
use tokio::io::AsyncRead;


// https://minecraft.wiki/w/Java_Edition_protocol/Slot_data?oldid=3156505
#[derive(NetEncode)]
pub enum Component {
    CustomData, // Unimplemented
    MaxStackSize(VarInt),
    MaxDamage(VarInt),
    Damage(VarInt),
    Unbreakable,
    CustomName(NBT<TextComponent>),
    ItemName(NBT<TextComponent>),
    ItemModel(String),
    Lore(LengthPrefixedVec<NBT<TextComponent>>),
    Rarity(Rarity),
    Enchantments(LengthPrefixedVec<EnchantComponent>),
    CanPlaceOn, // Unimplemented
    CanBreak,   // Unimplemented
    AttributeModifiers(LengthPrefixedVec<AttributeModifierComponent>),
    CustomModelData, // Unimplemented
    BannerPatterns,
    BaseColor,
    Bees {
        entity_data: String,
    },
    BlockEntityData,
    BlockState,
    BlocksAttacks {
        block_delay_seconds: float,
        disable_cooldown_scale: float,
        damage_reductions: list,
        item_damage: VarInt,
    },
    BundleComponents {
        id: String,
        count: VarInt,
        components: LengthPrefixedVec<VarInt>,
    },
    
    TooltipDisplay {
        hide_tooltip: bool,
        hidden_components: LengthPrefixedVec<VarInt>,
    },
    RepairCost(VarInt),
    CreativeSlotLock,
    EnchantmentGlintOverride(bool),
    IntangibleProjectile, // Unimplemented
    Food {
        nutrition: VarInt,
        saturation_modifier: f32,
        can_always_eat: bool,
    },
    Consumable {
        consume_time: VarInt,
        animation: ConsumableAnimation,
        id: String,
        has_particles: bool,
        consume_effects: LengthPrefixedVec<ConsumeEffect>,
    },
    UseRemainder(InventorySlot),
    UseCooldown {
        seconds: f32,
        group: PrefixedOptional<String>,
    },
    DamageResistant(String),
    Tool {
        rule: LengthPrefixedVec<ToolRule>,
    },
    Weapon {
        damage: VarInt,
        disable_block: f32,
    },
    Enchantable(VarInt),
    Equippable {
        slot: EquippableSlot,
        sound: SoundEvent,
        model: PrefixedOptional<String>,
        camera_overlay: PrefixedOptional<String>,
        allowed_entities: PrefixedOptional<IDSet>,
        dispensable: bool,
        swapper: bool,
        damage_on_hurt: bool,
    },
}
#[derive(NetEncode, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum EquippableSlot {
    MainHand,
    Feet,
    Legs,
    Chest,
    Head,
    OffHand,
    Body,
}
#[derive(NetEncode, NetDecode)]
pub struct ToolRule {
    pub blocks: IDSet,
    pub speed: PrefixedOptional<f32>,
    pub correct_drop: PrefixedOptional<bool>,
}
#[derive(NetEncode)]
pub enum ConsumeEffect {
    ApplyEffects {
        effects: LengthPrefixedVec<VarInt>,
        probability: f32,
    },
    RemoveEffects(IDSet),
    ClearEffects,
    TeleportRandomly(f32),
    PlaySound(SoundEvent),
}
#[derive(NetEncode, NetDecode)]
pub struct SoundEvent {
    pub sound_name: String,
    fixed_range: PrefixedOptional<f32>,
}

#[derive(NetEncode, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum ConsumableAnimation {
    None,
    Eat,
    Drink,
    Bloc,
    Bow,
    Spear,
    Crossbow,
    Spyglass,
    TootHorn,
    Brush,
}

#[derive(NetEncode, NetDecode)]
pub struct EnchantComponent {
    pub id: VarInt,
    pub level: VarInt,
}

#[derive(NetEncode, NetDecode)]
pub struct AttributeModifierComponent {
    pub attribute_id: VarInt,
    pub identifier: String,
    pub value: f64,
    pub operation: AttributeModifierOperation,
    pub slot: AttributeModifierSlot,
}

#[derive(NetEncode, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum AttributeModifierOperation {
    AddNumber = 0,
    AddPercentage = 1,
    MultiplyPercentage = 2,
}

#[derive(NetEncode, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum AttributeModifierSlot {
    Any,
    MainHand,
    OffHand,
    Hand,
    Feet,
    Legs,
    Chest,
    Head,
    Armor,
    Body,
}

#[derive(NetEncode, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}

impl NetDecode for Component {
    fn decode<R: std::io::Read>(
        mut reader: &mut R,
        opts: &ferrumc_net_codec::decode::NetDecodeOpts,
    ) -> Result<Self, ferrumc_net_codec::decode::errors::NetDecodeError> {
        let variant = VarInt::read(reader)?;
        match variant.0 {
            0 => unimplemented!(),
            1 => {
                let size = VarInt::read(&mut reader)?;
                Ok(Component::MaxStackSize(size))
            }
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!()
    }
}

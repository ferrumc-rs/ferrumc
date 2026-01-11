use crate::structured_components::components::*;
use crate::structured_components::errors::StructuredComponentError;
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use log::debug;
use std::io::{Read, Write};
use tokio::io::AsyncReadExt;
use tokio::io::{AsyncRead, AsyncWrite};
#[doc = r" NOTE:"]
#[doc = r" Structured components use an asymmetric protocol:"]
#[doc = r" - client -> server: id + length + data"]
#[doc = r" - server -> client: id + data"]
#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub enum StructuredComponent {
    #[default]
    Invalid,
    MaxStackSize(MaxStackSize),
    MaxDamage(MaxDamage),
    Damage(Damage),
    Unbreakable,
    CustomName(TextComponentWrapper),
    ItemName(TextComponentWrapper),
    Lore(Lore),
    Rarity(Rarity),
    Enchantments(EnchantmentsCollection),
    CustomModelData(CustomModelData),
    TooltipDisplay(TooltipDisplay),
    RepairCost(RepairCost),
    CreativeSlotLock,
    EnchantmentGlintOverride(EnchantmentGlintOverride),
    Food(Food),
    Consumable(Consumable),
    UseCooldown(UseCooldown),
    DamageResistant(DamageResistant),
    Tool(Tool),
    Weapon(Weapon),
    Enchantable(Enchantable),
    Equippable(Equippable),
    Repairable(Repairable),
    Glider,
    TooltipStyle(TooltipStyle),
    DeathProtection(DeathProtection),
    BlocksAttacks(BlocksAttacks),
    StoredEnchantments(EnchantmentsCollection),
    DyedColor(DyedColor),
    MapColor(MapColor),
    MapId(MapId),
    MapPostProcessing(MapPostProcessing),
    PotionContents(PotionContents),
    SuspiciousStewEffects(SuspiciousStewEffects),
    WritableBookContent(WritableBookContent),
    WrittenBookContent(WrittenBookContent),
    OminousBottleAmplifier(OminousBottleAmplifier),
    Fireworks(Fireworks),
}
impl StructuredComponent {
    pub fn to_id(&self) -> Result<VarInt, StructuredComponentError> {
        match self {
            StructuredComponent::Invalid => Err(StructuredComponentError::InvalidEnum),
            StructuredComponent::MaxStackSize(_) => Ok(VarInt::from(1i32)),
            StructuredComponent::MaxDamage(_) => Ok(VarInt::from(2i32)),
            StructuredComponent::Damage(_) => Ok(VarInt::from(3i32)),
            StructuredComponent::Unbreakable => Ok(VarInt::from(4i32)),
            StructuredComponent::CustomName(_) => Ok(VarInt::from(5i32)),
            StructuredComponent::ItemName(_) => Ok(VarInt::from(6i32)),
            StructuredComponent::Lore(_) => Ok(VarInt::from(8i32)),
            StructuredComponent::Rarity(_) => Ok(VarInt::from(9i32)),
            StructuredComponent::Enchantments(_) => Ok(VarInt::from(10i32)),
            StructuredComponent::CustomModelData(_) => Ok(VarInt::from(14i32)),
            StructuredComponent::TooltipDisplay(_) => Ok(VarInt::from(15i32)),
            StructuredComponent::RepairCost(_) => Ok(VarInt::from(16i32)),
            StructuredComponent::CreativeSlotLock => Ok(VarInt::from(17i32)),
            StructuredComponent::EnchantmentGlintOverride(_) => Ok(VarInt::from(18i32)),
            StructuredComponent::Food(_) => Ok(VarInt::from(20i32)),
            StructuredComponent::Consumable(_) => Ok(VarInt::from(21i32)),
            StructuredComponent::UseCooldown(_) => Ok(VarInt::from(23i32)),
            StructuredComponent::DamageResistant(_) => Ok(VarInt::from(24i32)),
            StructuredComponent::Tool(_) => Ok(VarInt::from(25i32)),
            StructuredComponent::Weapon(_) => Ok(VarInt::from(26i32)),
            StructuredComponent::Enchantable(_) => Ok(VarInt::from(27i32)),
            StructuredComponent::Equippable(_) => Ok(VarInt::from(28i32)),
            StructuredComponent::Repairable(_) => Ok(VarInt::from(29i32)),
            StructuredComponent::Glider => Ok(VarInt::from(30i32)),
            StructuredComponent::TooltipStyle(_) => Ok(VarInt::from(31i32)),
            StructuredComponent::DeathProtection(_) => Ok(VarInt::from(32i32)),
            StructuredComponent::BlocksAttacks(_) => Ok(VarInt::from(33i32)),
            StructuredComponent::StoredEnchantments(_) => Ok(VarInt::from(34i32)),
            StructuredComponent::DyedColor(_) => Ok(VarInt::from(35i32)),
            StructuredComponent::MapColor(_) => Ok(VarInt::from(36i32)),
            StructuredComponent::MapId(_) => Ok(VarInt::from(37i32)),
            StructuredComponent::MapPostProcessing(_) => Ok(VarInt::from(39i32)),
            StructuredComponent::PotionContents(_) => Ok(VarInt::from(42i32)),
            StructuredComponent::SuspiciousStewEffects(_) => Ok(VarInt::from(44i32)),
            StructuredComponent::WritableBookContent(_) => Ok(VarInt::from(45i32)),
            StructuredComponent::WrittenBookContent(_) => Ok(VarInt::from(46i32)),
            StructuredComponent::OminousBottleAmplifier(_) => Ok(VarInt::from(54i32)),
            StructuredComponent::Fireworks(_) => Ok(VarInt::from(60i32)),
        }
    }
}
impl NetEncode for StructuredComponent {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        if let StructuredComponent::Invalid = self {
            return Err(StructuredComponentError::InvalidEnum.into());
        }
        let id = self.to_id()?;
        id.encode(writer, opts)?;
        match self {
            StructuredComponent::Invalid => unreachable!(),
            StructuredComponent::MaxStackSize(inner) => inner.encode(writer, opts),
            StructuredComponent::MaxDamage(inner) => inner.encode(writer, opts),
            StructuredComponent::Damage(inner) => inner.encode(writer, opts),
            StructuredComponent::Unbreakable => Ok(()),
            StructuredComponent::CustomName(inner) => inner.encode(writer, opts),
            StructuredComponent::ItemName(inner) => inner.encode(writer, opts),
            StructuredComponent::Lore(inner) => inner.encode(writer, opts),
            StructuredComponent::Rarity(inner) => inner.encode(writer, opts),
            StructuredComponent::Enchantments(inner) => inner.encode(writer, opts),
            StructuredComponent::CustomModelData(inner) => inner.encode(writer, opts),
            StructuredComponent::TooltipDisplay(inner) => inner.encode(writer, opts),
            StructuredComponent::RepairCost(inner) => inner.encode(writer, opts),
            StructuredComponent::CreativeSlotLock => Ok(()),
            StructuredComponent::EnchantmentGlintOverride(inner) => inner.encode(writer, opts),
            StructuredComponent::Food(inner) => inner.encode(writer, opts),
            StructuredComponent::Consumable(inner) => inner.encode(writer, opts),
            StructuredComponent::UseCooldown(inner) => inner.encode(writer, opts),
            StructuredComponent::DamageResistant(inner) => inner.encode(writer, opts),
            StructuredComponent::Tool(inner) => inner.encode(writer, opts),
            StructuredComponent::Weapon(inner) => inner.encode(writer, opts),
            StructuredComponent::Enchantable(inner) => inner.encode(writer, opts),
            StructuredComponent::Equippable(inner) => inner.encode(writer, opts),
            StructuredComponent::Repairable(inner) => inner.encode(writer, opts),
            StructuredComponent::Glider => Ok(()),
            StructuredComponent::TooltipStyle(inner) => inner.encode(writer, opts),
            StructuredComponent::DeathProtection(inner) => inner.encode(writer, opts),
            StructuredComponent::BlocksAttacks(inner) => inner.encode(writer, opts),
            StructuredComponent::StoredEnchantments(inner) => inner.encode(writer, opts),
            StructuredComponent::DyedColor(inner) => inner.encode(writer, opts),
            StructuredComponent::MapColor(inner) => inner.encode(writer, opts),
            StructuredComponent::MapId(inner) => inner.encode(writer, opts),
            StructuredComponent::MapPostProcessing(inner) => inner.encode(writer, opts),
            StructuredComponent::PotionContents(inner) => inner.encode(writer, opts),
            StructuredComponent::SuspiciousStewEffects(inner) => inner.encode(writer, opts),
            StructuredComponent::WritableBookContent(inner) => inner.encode(writer, opts),
            StructuredComponent::WrittenBookContent(inner) => inner.encode(writer, opts),
            StructuredComponent::OminousBottleAmplifier(inner) => inner.encode(writer, opts),
            StructuredComponent::Fireworks(inner) => inner.encode(writer, opts),
        }
    }
    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        if let StructuredComponent::Invalid = self {
            return Err(StructuredComponentError::InvalidEnum.into());
        }
        let id = self.to_id()?;
        id.encode_async(writer, opts).await?;
        match self {
            StructuredComponent::Invalid => unreachable!(),
            StructuredComponent::MaxStackSize(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::MaxDamage(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::Damage(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::Unbreakable => Ok(()),
            StructuredComponent::CustomName(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::ItemName(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::Lore(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::Rarity(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::Enchantments(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::CustomModelData(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::TooltipDisplay(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::RepairCost(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::CreativeSlotLock => Ok(()),
            StructuredComponent::EnchantmentGlintOverride(inner) => {
                inner.encode_async(writer, opts).await
            }
            StructuredComponent::Food(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::Consumable(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::UseCooldown(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::DamageResistant(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::Tool(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::Weapon(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::Enchantable(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::Equippable(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::Repairable(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::Glider => Ok(()),
            StructuredComponent::TooltipStyle(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::DeathProtection(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::BlocksAttacks(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::StoredEnchantments(inner) => {
                inner.encode_async(writer, opts).await
            }
            StructuredComponent::DyedColor(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::MapColor(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::MapId(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::MapPostProcessing(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::PotionContents(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::SuspiciousStewEffects(inner) => {
                inner.encode_async(writer, opts).await
            }
            StructuredComponent::WritableBookContent(inner) => {
                inner.encode_async(writer, opts).await
            }
            StructuredComponent::WrittenBookContent(inner) => {
                inner.encode_async(writer, opts).await
            }
            StructuredComponent::OminousBottleAmplifier(inner) => {
                inner.encode_async(writer, opts).await
            }
            StructuredComponent::Fireworks(inner) => inner.encode_async(writer, opts).await,
        }
    }
}
impl NetDecode for StructuredComponent {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let id = VarInt::decode(reader, opts)?;
        let length = VarInt::decode(reader, opts)?;
        debug! { "Decoding structuredComponent with id {} and length {}" , id , length }
        let mut buffer = vec![0u8; length.0 as usize];
        reader
            .read_exact(&mut buffer)
            .map_err(|e| NetDecodeError::from(e))?;
        let mut limited_reader = std::io::Cursor::new(buffer);
        let result: Result<StructuredComponent, NetDecodeError> = match id.0 {
            1i32 => Ok(StructuredComponent::MaxStackSize(MaxStackSize::decode(
                &mut limited_reader,
                opts,
            )?)),
            2i32 => Ok(StructuredComponent::MaxDamage(MaxDamage::decode(
                &mut limited_reader,
                opts,
            )?)),
            3i32 => Ok(StructuredComponent::Damage(Damage::decode(
                &mut limited_reader,
                opts,
            )?)),
            4i32 => Ok(StructuredComponent::Unbreakable),
            5i32 => Ok(StructuredComponent::CustomName(
                TextComponentWrapper::decode(&mut limited_reader, opts)?,
            )),
            6i32 => Ok(StructuredComponent::ItemName(TextComponentWrapper::decode(
                &mut limited_reader,
                opts,
            )?)),
            8i32 => Ok(StructuredComponent::Lore(Lore::decode(
                &mut limited_reader,
                opts,
            )?)),
            9i32 => Ok(StructuredComponent::Rarity(Rarity::decode(
                &mut limited_reader,
                opts,
            )?)),
            10i32 => Ok(StructuredComponent::Enchantments(
                EnchantmentsCollection::decode(&mut limited_reader, opts)?,
            )),
            14i32 => Ok(StructuredComponent::CustomModelData(
                CustomModelData::decode(&mut limited_reader, opts)?,
            )),
            15i32 => Ok(StructuredComponent::TooltipDisplay(TooltipDisplay::decode(
                &mut limited_reader,
                opts,
            )?)),
            16i32 => Ok(StructuredComponent::RepairCost(RepairCost::decode(
                &mut limited_reader,
                opts,
            )?)),
            17i32 => Ok(StructuredComponent::CreativeSlotLock),
            18i32 => Ok(StructuredComponent::EnchantmentGlintOverride(
                EnchantmentGlintOverride::decode(&mut limited_reader, opts)?,
            )),
            20i32 => Ok(StructuredComponent::Food(Food::decode(
                &mut limited_reader,
                opts,
            )?)),
            21i32 => Ok(StructuredComponent::Consumable(Consumable::decode(
                &mut limited_reader,
                opts,
            )?)),
            23i32 => Ok(StructuredComponent::UseCooldown(UseCooldown::decode(
                &mut limited_reader,
                opts,
            )?)),
            24i32 => Ok(StructuredComponent::DamageResistant(
                DamageResistant::decode(&mut limited_reader, opts)?,
            )),
            25i32 => Ok(StructuredComponent::Tool(Tool::decode(
                &mut limited_reader,
                opts,
            )?)),
            26i32 => Ok(StructuredComponent::Weapon(Weapon::decode(
                &mut limited_reader,
                opts,
            )?)),
            27i32 => Ok(StructuredComponent::Enchantable(Enchantable::decode(
                &mut limited_reader,
                opts,
            )?)),
            28i32 => Ok(StructuredComponent::Equippable(Equippable::decode(
                &mut limited_reader,
                opts,
            )?)),
            29i32 => Ok(StructuredComponent::Repairable(Repairable::decode(
                &mut limited_reader,
                opts,
            )?)),
            30i32 => Ok(StructuredComponent::Glider),
            31i32 => Ok(StructuredComponent::TooltipStyle(TooltipStyle::decode(
                &mut limited_reader,
                opts,
            )?)),
            32i32 => Ok(StructuredComponent::DeathProtection(
                DeathProtection::decode(&mut limited_reader, opts)?,
            )),
            33i32 => Ok(StructuredComponent::BlocksAttacks(BlocksAttacks::decode(
                &mut limited_reader,
                opts,
            )?)),
            34i32 => Ok(StructuredComponent::StoredEnchantments(
                EnchantmentsCollection::decode(&mut limited_reader, opts)?,
            )),
            35i32 => Ok(StructuredComponent::DyedColor(DyedColor::decode(
                &mut limited_reader,
                opts,
            )?)),
            36i32 => Ok(StructuredComponent::MapColor(MapColor::decode(
                &mut limited_reader,
                opts,
            )?)),
            37i32 => Ok(StructuredComponent::MapId(MapId::decode(
                &mut limited_reader,
                opts,
            )?)),
            39i32 => Ok(StructuredComponent::MapPostProcessing(
                MapPostProcessing::decode(&mut limited_reader, opts)?,
            )),
            42i32 => Ok(StructuredComponent::PotionContents(PotionContents::decode(
                &mut limited_reader,
                opts,
            )?)),
            44i32 => Ok(StructuredComponent::SuspiciousStewEffects(
                SuspiciousStewEffects::decode(&mut limited_reader, opts)?,
            )),
            45i32 => Ok(StructuredComponent::WritableBookContent(
                WritableBookContent::decode(&mut limited_reader, opts)?,
            )),
            46i32 => Ok(StructuredComponent::WrittenBookContent(
                WrittenBookContent::decode(&mut limited_reader, opts)?,
            )),
            54i32 => Ok(StructuredComponent::OminousBottleAmplifier(
                OminousBottleAmplifier::decode(&mut limited_reader, opts)?,
            )),
            60i32 => Ok(StructuredComponent::Fireworks(Fireworks::decode(
                &mut limited_reader,
                opts,
            )?)),
            _ => {
                return Err(NetDecodeError::from(
                    StructuredComponentError::NotSupported(id),
                ));
            }
        };
        if limited_reader.position() < length.0 as u64 {
            return Err(NetDecodeError::ExternalError(
                "Decoding didn't read all expected data".into(),
            ));
        }
        result
    }
    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let id = VarInt::decode_async(reader, opts).await?;
        let length = VarInt::decode_async(reader, opts).await?;
        debug!(
            "Decoding structuredComponent with id {} and length {}",
            id, length
        );
        let mut buffer = vec![0u8; length.0 as usize];
        tokio::io::AsyncReadExt::read_exact(reader, &mut buffer)
            .await
            .map_err(|e| NetDecodeError::from(e))?;
        let mut limited_reader = std::io::Cursor::new(buffer);
        let result: Result<StructuredComponent, NetDecodeError> = match id.0 {
            1i32 => Ok(StructuredComponent::MaxStackSize(MaxStackSize::decode(
                &mut limited_reader,
                opts,
            )?)),
            2i32 => Ok(StructuredComponent::MaxDamage(MaxDamage::decode(
                &mut limited_reader,
                opts,
            )?)),
            3i32 => Ok(StructuredComponent::Damage(Damage::decode(
                &mut limited_reader,
                opts,
            )?)),
            4i32 => Ok(StructuredComponent::Unbreakable),
            5i32 => Ok(StructuredComponent::CustomName(
                TextComponentWrapper::decode(&mut limited_reader, opts)?,
            )),
            6i32 => Ok(StructuredComponent::ItemName(TextComponentWrapper::decode(
                &mut limited_reader,
                opts,
            )?)),
            8i32 => Ok(StructuredComponent::Lore(Lore::decode(
                &mut limited_reader,
                opts,
            )?)),
            9i32 => Ok(StructuredComponent::Rarity(Rarity::decode(
                &mut limited_reader,
                opts,
            )?)),
            10i32 => Ok(StructuredComponent::Enchantments(
                EnchantmentsCollection::decode(&mut limited_reader, opts)?,
            )),
            14i32 => Ok(StructuredComponent::CustomModelData(
                CustomModelData::decode(&mut limited_reader, opts)?,
            )),
            15i32 => Ok(StructuredComponent::TooltipDisplay(TooltipDisplay::decode(
                &mut limited_reader,
                opts,
            )?)),
            16i32 => Ok(StructuredComponent::RepairCost(RepairCost::decode(
                &mut limited_reader,
                opts,
            )?)),
            17i32 => Ok(StructuredComponent::CreativeSlotLock),
            18i32 => Ok(StructuredComponent::EnchantmentGlintOverride(
                EnchantmentGlintOverride::decode(&mut limited_reader, opts)?,
            )),
            20i32 => Ok(StructuredComponent::Food(Food::decode(
                &mut limited_reader,
                opts,
            )?)),
            21i32 => Ok(StructuredComponent::Consumable(Consumable::decode(
                &mut limited_reader,
                opts,
            )?)),
            23i32 => Ok(StructuredComponent::UseCooldown(UseCooldown::decode(
                &mut limited_reader,
                opts,
            )?)),
            24i32 => Ok(StructuredComponent::DamageResistant(
                DamageResistant::decode(&mut limited_reader, opts)?,
            )),
            25i32 => Ok(StructuredComponent::Tool(Tool::decode(
                &mut limited_reader,
                opts,
            )?)),
            26i32 => Ok(StructuredComponent::Weapon(Weapon::decode(
                &mut limited_reader,
                opts,
            )?)),
            27i32 => Ok(StructuredComponent::Enchantable(Enchantable::decode(
                &mut limited_reader,
                opts,
            )?)),
            28i32 => Ok(StructuredComponent::Equippable(Equippable::decode(
                &mut limited_reader,
                opts,
            )?)),
            29i32 => Ok(StructuredComponent::Repairable(Repairable::decode(
                &mut limited_reader,
                opts,
            )?)),
            30i32 => Ok(StructuredComponent::Glider),
            31i32 => Ok(StructuredComponent::TooltipStyle(TooltipStyle::decode(
                &mut limited_reader,
                opts,
            )?)),
            32i32 => Ok(StructuredComponent::DeathProtection(
                DeathProtection::decode(&mut limited_reader, opts)?,
            )),
            33i32 => Ok(StructuredComponent::BlocksAttacks(BlocksAttacks::decode(
                &mut limited_reader,
                opts,
            )?)),
            34i32 => Ok(StructuredComponent::StoredEnchantments(
                EnchantmentsCollection::decode(&mut limited_reader, opts)?,
            )),
            35i32 => Ok(StructuredComponent::DyedColor(DyedColor::decode(
                &mut limited_reader,
                opts,
            )?)),
            36i32 => Ok(StructuredComponent::MapColor(MapColor::decode(
                &mut limited_reader,
                opts,
            )?)),
            37i32 => Ok(StructuredComponent::MapId(MapId::decode(
                &mut limited_reader,
                opts,
            )?)),
            39i32 => Ok(StructuredComponent::MapPostProcessing(
                MapPostProcessing::decode(&mut limited_reader, opts)?,
            )),
            42i32 => Ok(StructuredComponent::PotionContents(PotionContents::decode(
                &mut limited_reader,
                opts,
            )?)),
            44i32 => Ok(StructuredComponent::SuspiciousStewEffects(
                SuspiciousStewEffects::decode(&mut limited_reader, opts)?,
            )),
            45i32 => Ok(StructuredComponent::WritableBookContent(
                WritableBookContent::decode(&mut limited_reader, opts)?,
            )),
            46i32 => Ok(StructuredComponent::WrittenBookContent(
                WrittenBookContent::decode(&mut limited_reader, opts)?,
            )),
            54i32 => Ok(StructuredComponent::OminousBottleAmplifier(
                OminousBottleAmplifier::decode(&mut limited_reader, opts)?,
            )),
            60i32 => Ok(StructuredComponent::Fireworks(Fireworks::decode(
                &mut limited_reader,
                opts,
            )?)),
            _ => {
                return Err(NetDecodeError::from(
                    StructuredComponentError::NotSupported(id),
                ));
            }
        };
        if limited_reader.position() < length.0 as u64 {
            return Err(NetDecodeError::ExternalError(
                "Decoding didn't read all expected data".into(),
            ));
        }
        result
    }
}

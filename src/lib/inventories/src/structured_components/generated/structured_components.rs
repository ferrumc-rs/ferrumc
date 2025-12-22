use crate::structured_components::errors::{
    InvalidStructuredComponentEnumError, NotSupportedStructuredComponentError,
};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use log::debug;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};
#[doc = r" NOTE:"]
#[doc = r" Structured components use an asymmetric protocol:"]
#[doc = r" - client -> server: id + length + data"]
#[doc = r" - server -> client: id + data"]
#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub enum StructuredComponent {
    #[default]
    Invalid,
    MaxStackSize(crate::structured_components::components::MaxStackSize),
    MaxDamage(crate::structured_components::components::MaxDamage),
    Damage(crate::structured_components::components::Damage),
    Unbreakable,
    CustomName(crate::structured_components::components::TextComponentWrapper),
    ItemName(crate::structured_components::components::TextComponentWrapper),
    Lore(crate::structured_components::components::Lore),
    Rarity(crate::structured_components::components::Rarity),
    Enchantments(crate::structured_components::components::EnchantmentsCollection),
    Food(crate::structured_components::components::Food),
    RepairCost(crate::structured_components::components::RepairCost),
    EnchantmentGlintOverride(crate::structured_components::components::EnchantmentGlintOverride),
    Enchantable(crate::structured_components::components::Enchantable),
    StoredEnchantments(crate::structured_components::components::EnchantmentsCollection),
    PotionContents(crate::structured_components::components::PotionContents),
    SuspiciousStewEffects(crate::structured_components::components::SuspiciousStewEffects),
    WritableBookContent(crate::structured_components::components::WritableBookContent),
    WrittenBookContent(crate::structured_components::components::WrittenBookContent),
    OminousBottleAmplifier(crate::structured_components::components::OminousBottleAmplifier),
    Fireworks(crate::structured_components::components::Fireworks),
}
impl StructuredComponent {
    pub fn to_id(&self) -> Result<VarInt, InvalidStructuredComponentEnumError> {
        match self {
            StructuredComponent::Invalid => Err(InvalidStructuredComponentEnumError()),
            StructuredComponent::MaxStackSize(_) => Ok(VarInt::from(1i32)),
            StructuredComponent::MaxDamage(_) => Ok(VarInt::from(2i32)),
            StructuredComponent::Damage(_) => Ok(VarInt::from(3i32)),
            StructuredComponent::Unbreakable => Ok(VarInt::from(4i32)),
            StructuredComponent::CustomName(_) => Ok(VarInt::from(5i32)),
            StructuredComponent::ItemName(_) => Ok(VarInt::from(6i32)),
            StructuredComponent::Lore(_) => Ok(VarInt::from(8i32)),
            StructuredComponent::Rarity(_) => Ok(VarInt::from(9i32)),
            StructuredComponent::Enchantments(_) => Ok(VarInt::from(10i32)),
            StructuredComponent::Food(_) => Ok(VarInt::from(20i32)),
            StructuredComponent::RepairCost(_) => Ok(VarInt::from(16i32)),
            StructuredComponent::EnchantmentGlintOverride(_) => Ok(VarInt::from(18i32)),
            StructuredComponent::Enchantable(_) => Ok(VarInt::from(27i32)),
            StructuredComponent::StoredEnchantments(_) => Ok(VarInt::from(34i32)),
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
            return Err(InvalidStructuredComponentEnumError().into());
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
            StructuredComponent::Food(inner) => inner.encode(writer, opts),
            StructuredComponent::RepairCost(inner) => inner.encode(writer, opts),
            StructuredComponent::EnchantmentGlintOverride(inner) => inner.encode(writer, opts),
            StructuredComponent::Enchantable(inner) => inner.encode(writer, opts),
            StructuredComponent::StoredEnchantments(inner) => inner.encode(writer, opts),
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
            return Err(InvalidStructuredComponentEnumError().into());
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
            StructuredComponent::Food(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::RepairCost(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::EnchantmentGlintOverride(inner) => {
                inner.encode_async(writer, opts).await
            }
            StructuredComponent::Enchantable(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::StoredEnchantments(inner) => {
                inner.encode_async(writer, opts).await
            }
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
        debug! { "Decoding structuredComponent with id {}" , id }
        match id.0 {
            1i32 => Ok(StructuredComponent::MaxStackSize(
                crate::structured_components::components::MaxStackSize::decode(reader, opts)?,
            )),
            2i32 => Ok(StructuredComponent::MaxDamage(
                crate::structured_components::components::MaxDamage::decode(reader, opts)?,
            )),
            3i32 => Ok(StructuredComponent::Damage(
                crate::structured_components::components::Damage::decode(reader, opts)?,
            )),
            4i32 => Ok(StructuredComponent::Unbreakable),
            5i32 => Ok(StructuredComponent::CustomName(
                crate::structured_components::components::TextComponentWrapper::decode(
                    reader, opts,
                )?,
            )),
            6i32 => Ok(StructuredComponent::ItemName(
                crate::structured_components::components::TextComponentWrapper::decode(
                    reader, opts,
                )?,
            )),
            8i32 => Ok(StructuredComponent::Lore(
                crate::structured_components::components::Lore::decode(reader, opts)?,
            )),
            9i32 => Ok(StructuredComponent::Rarity(
                crate::structured_components::components::Rarity::decode(reader, opts)?,
            )),
            10i32 => Ok(StructuredComponent::Enchantments(
                crate::structured_components::components::EnchantmentsCollection::decode(
                    reader, opts,
                )?,
            )),
            20i32 => Ok(StructuredComponent::Food(
                crate::structured_components::components::Food::decode(reader, opts)?,
            )),
            16i32 => Ok(StructuredComponent::RepairCost(
                crate::structured_components::components::RepairCost::decode(reader, opts)?,
            )),
            18i32 => Ok(StructuredComponent::EnchantmentGlintOverride(
                crate::structured_components::components::EnchantmentGlintOverride::decode(
                    reader, opts,
                )?,
            )),
            27i32 => Ok(StructuredComponent::Enchantable(
                crate::structured_components::components::Enchantable::decode(reader, opts)?,
            )),
            34i32 => Ok(StructuredComponent::StoredEnchantments(
                crate::structured_components::components::EnchantmentsCollection::decode(
                    reader, opts,
                )?,
            )),
            42i32 => Ok(StructuredComponent::PotionContents(
                crate::structured_components::components::PotionContents::decode(reader, opts)?,
            )),
            44i32 => Ok(StructuredComponent::SuspiciousStewEffects(
                crate::structured_components::components::SuspiciousStewEffects::decode(
                    reader, opts,
                )?,
            )),
            45i32 => Ok(StructuredComponent::WritableBookContent(
                crate::structured_components::components::WritableBookContent::decode(
                    reader, opts,
                )?,
            )),
            46i32 => Ok(StructuredComponent::WrittenBookContent(
                crate::structured_components::components::WrittenBookContent::decode(reader, opts)?,
            )),
            54i32 => Ok(StructuredComponent::OminousBottleAmplifier(
                crate::structured_components::components::OminousBottleAmplifier::decode(
                    reader, opts,
                )?,
            )),
            60i32 => Ok(StructuredComponent::Fireworks(
                crate::structured_components::components::Fireworks::decode(reader, opts)?,
            )),
            _ => Err(NotSupportedStructuredComponentError(id).into()),
        }
    }
    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let id = VarInt::decode_async(reader, opts).await?;
        debug! { "Decoding structuredComponent with id {}" , id }
        match id.0 {
            1i32 => Ok(StructuredComponent::MaxStackSize(
                crate::structured_components::components::MaxStackSize::decode_async(reader, opts)
                    .await?,
            )),
            2i32 => Ok(StructuredComponent::MaxDamage(
                crate::structured_components::components::MaxDamage::decode_async(reader, opts)
                    .await?,
            )),
            3i32 => Ok(StructuredComponent::Damage(
                crate::structured_components::components::Damage::decode_async(reader, opts)
                    .await?,
            )),
            4i32 => Ok(StructuredComponent::Unbreakable),
            5i32 => Ok(StructuredComponent::CustomName(
                crate::structured_components::components::TextComponentWrapper::decode_async(
                    reader, opts,
                )
                .await?,
            )),
            6i32 => Ok(StructuredComponent::ItemName(
                crate::structured_components::components::TextComponentWrapper::decode_async(
                    reader, opts,
                )
                .await?,
            )),
            8i32 => Ok(StructuredComponent::Lore(
                crate::structured_components::components::Lore::decode_async(reader, opts).await?,
            )),
            9i32 => Ok(StructuredComponent::Rarity(
                crate::structured_components::components::Rarity::decode_async(reader, opts)
                    .await?,
            )),
            10i32 => Ok(StructuredComponent::Enchantments(
                crate::structured_components::components::EnchantmentsCollection::decode_async(
                    reader, opts,
                )
                .await?,
            )),
            20i32 => Ok(StructuredComponent::Food(
                crate::structured_components::components::Food::decode_async(reader, opts).await?,
            )),
            16i32 => Ok(StructuredComponent::RepairCost(
                crate::structured_components::components::RepairCost::decode_async(reader, opts)
                    .await?,
            )),
            18i32 => Ok(StructuredComponent::EnchantmentGlintOverride(
                crate::structured_components::components::EnchantmentGlintOverride::decode_async(
                    reader, opts,
                )
                .await?,
            )),
            27i32 => Ok(StructuredComponent::Enchantable(
                crate::structured_components::components::Enchantable::decode_async(reader, opts)
                    .await?,
            )),
            34i32 => Ok(StructuredComponent::StoredEnchantments(
                crate::structured_components::components::EnchantmentsCollection::decode_async(
                    reader, opts,
                )
                .await?,
            )),
            42i32 => Ok(StructuredComponent::PotionContents(
                crate::structured_components::components::PotionContents::decode_async(
                    reader, opts,
                )
                .await?,
            )),
            44i32 => Ok(StructuredComponent::SuspiciousStewEffects(
                crate::structured_components::components::SuspiciousStewEffects::decode_async(
                    reader, opts,
                )
                .await?,
            )),
            45i32 => Ok(StructuredComponent::WritableBookContent(
                crate::structured_components::components::WritableBookContent::decode_async(
                    reader, opts,
                )
                .await?,
            )),
            46i32 => Ok(StructuredComponent::WrittenBookContent(
                crate::structured_components::components::WrittenBookContent::decode_async(
                    reader, opts,
                )
                .await?,
            )),
            54i32 => Ok(StructuredComponent::OminousBottleAmplifier(
                crate::structured_components::components::OminousBottleAmplifier::decode_async(
                    reader, opts,
                )
                .await?,
            )),
            60i32 => Ok(StructuredComponent::Fireworks(
                crate::structured_components::components::Fireworks::decode_async(reader, opts)
                    .await?,
            )),
            _ => Err(NotSupportedStructuredComponentError(id).into()),
        }
    }
}

use crate::netcode::components::enchantments::enchantable::Enchantable;
use crate::netcode::components::enchantments::enchantment_glint_override::EnchantmentGlintOverride;
use crate::netcode::components::enchantments::enchantments_collection::EnchantmentsCollection;
use crate::netcode::components::potion_contents::PotionContents;
use crate::netcode::errors::{
    InvalidStructuredComponentEnumError,
    NotSupportedStructuredComponentError,
};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};
use crate::netcode::components::damage::{Damage, MaxDamage};
use crate::netcode::components::fireworks::Fireworks;
use crate::netcode::components::ominous_bottle_amplifier::OminousBottleAmplifier;
use crate::netcode::components::suspicious_stew_effects::SuspiciousStewEffects;

#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub enum StructuredComponent {
    #[default]
    Invalid,
    MaxDamage(MaxDamage),
    Damage(Damage),
    Unbreakable(),
    Enchantments(EnchantmentsCollection),
    EnchantmentGlintOverride(EnchantmentGlintOverride),
    Enchantable(Enchantable),
    StoredEnchantments(EnchantmentsCollection),
    PotionContents(PotionContents),
    SuspiciousStewEffects(SuspiciousStewEffects),
    OminousBottleAmplifier(OminousBottleAmplifier),
    Fireworks(Fireworks),
}

impl StructuredComponent {
    fn to_id(&self) -> Result<VarInt, InvalidStructuredComponentEnumError> {
        match self {
            StructuredComponent::MaxDamage(_) => Ok(VarInt::from(2)),
            StructuredComponent::Damage(_) => Ok(VarInt::from(3)),
            StructuredComponent::Unbreakable() => Ok(VarInt::from(4)),
            StructuredComponent::Enchantments(_) => Ok(VarInt::from(10)),
            StructuredComponent::EnchantmentGlintOverride(_) => Ok(VarInt::from(18)),
            StructuredComponent::Enchantable(_) => Ok(VarInt::from(27)),
            StructuredComponent::StoredEnchantments(_) => Ok(VarInt::from(34)),
            StructuredComponent::PotionContents(_) => Ok(VarInt::from(42)),
            StructuredComponent::SuspiciousStewEffects(_) => Ok(VarInt::from(44)),
            StructuredComponent::OminousBottleAmplifier(_) => Ok(VarInt::from(54)),
            StructuredComponent::Fireworks(_) => Ok(VarInt::from(60)),
            StructuredComponent::Invalid => Err(InvalidStructuredComponentEnumError()),
        }
    }

    fn read<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let id = VarInt::decode(reader, opts)?;

        let _probably_data_length = VarInt::decode(reader, opts)?; // I don't know why there is this byte. But it is there!

        let component = match id.0 {
            2 => StructuredComponent::MaxDamage(MaxDamage::decode(reader, opts)?),
            3 => StructuredComponent::Damage(Damage::decode(reader, opts)?),
            4 => StructuredComponent::Unbreakable(),
            10 => StructuredComponent::Enchantments(EnchantmentsCollection::decode(reader, opts)?),
            18 => StructuredComponent::EnchantmentGlintOverride(EnchantmentGlintOverride::decode(reader, opts)?),
            27 => StructuredComponent::Enchantable(Enchantable::decode(reader, opts)?),
            34 => StructuredComponent::StoredEnchantments(EnchantmentsCollection::decode(reader, opts)?),
            42 => StructuredComponent::PotionContents(PotionContents::decode(reader, opts)?),
            44 => StructuredComponent::SuspiciousStewEffects(SuspiciousStewEffects::decode(reader, opts)?),
            54 => StructuredComponent::OminousBottleAmplifier(OminousBottleAmplifier::decode(reader, opts)?),
            60 => StructuredComponent::Fireworks(Fireworks::decode(reader, opts)?),
            _ => Err(NotSupportedStructuredComponentError(id))?,
        };

        Ok(component)
    }

    async fn read_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let id = VarInt::decode_async(reader, opts).await?;

        let _probably_data_length = VarInt::decode_async(reader, opts).await?; // I don't know why there is this byte. But it is there!

        let component = match id.0 {
            2 => StructuredComponent::MaxDamage(MaxDamage::decode_async(reader, opts).await?),
            3 => StructuredComponent::Damage(Damage::decode_async(reader, opts).await?),
            4 => StructuredComponent::Unbreakable(),
            10 => StructuredComponent::Enchantments(EnchantmentsCollection::decode_async(reader, opts).await?),
            18 => StructuredComponent::EnchantmentGlintOverride(EnchantmentGlintOverride::decode_async(reader, opts).await?),
            27 => StructuredComponent::Enchantable(Enchantable::decode_async(reader, opts).await?),
            34 => StructuredComponent::StoredEnchantments(EnchantmentsCollection::decode_async(reader, opts).await?),
            42 => StructuredComponent::PotionContents(PotionContents::decode_async(reader, opts).await?),
            44 => StructuredComponent::SuspiciousStewEffects(SuspiciousStewEffects::decode_async(reader, opts).await?),
            54 => StructuredComponent::OminousBottleAmplifier(OminousBottleAmplifier::decode_async(reader, opts).await?),
            60 => StructuredComponent::Fireworks(Fireworks::decode_async(reader, opts).await?),
            _ => Err(NotSupportedStructuredComponentError(id))?,
        };

        Ok(component)
    }
}

impl NetEncode for StructuredComponent {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        let id = self.to_id()?;

        id.encode(writer, opts)?;

        match self {
            StructuredComponent::MaxDamage(max_damage) => max_damage.encode(writer, opts),
            StructuredComponent::Damage(damage) => damage.encode(writer, opts),
            StructuredComponent::Unbreakable() => Ok(()),
            StructuredComponent::Enchantments(enchantments) => enchantments.encode(writer, opts),
            StructuredComponent::EnchantmentGlintOverride(glint_override) => glint_override.encode(writer, opts),
            StructuredComponent::Enchantable(enchantable) => enchantable.encode(writer, opts),
            StructuredComponent::StoredEnchantments(stored_enchantments) => stored_enchantments.encode(writer, opts),
            StructuredComponent::PotionContents(potion_contents) => potion_contents.encode(writer, opts),
            StructuredComponent::SuspiciousStewEffects(suspicious_stew_effects) => suspicious_stew_effects.encode(writer, opts),
            StructuredComponent::OminousBottleAmplifier(ominous_bottle_amplifier) => ominous_bottle_amplifier.encode(writer, opts),
            StructuredComponent::Fireworks(fireworks) => fireworks.encode(writer, opts),
            StructuredComponent::Invalid => Err(InvalidStructuredComponentEnumError())?,
        }
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        let id = self.to_id()?;

        id.encode_async(writer, opts).await?;

        match self {
            StructuredComponent::MaxDamage(max_damage) => max_damage.encode_async(writer, opts).await,
            StructuredComponent::Damage(damage) => damage.encode_async(writer, opts).await,
            StructuredComponent::Unbreakable() => Ok(()),
            StructuredComponent::Enchantments(enchantments) => enchantments.encode_async(writer, opts).await,
            StructuredComponent::EnchantmentGlintOverride(glint_override) => glint_override.encode_async(writer, opts).await,
            StructuredComponent::Enchantable(enchantable) => enchantable.encode_async(writer, opts).await,
            StructuredComponent::StoredEnchantments(stored_enchantments) => stored_enchantments.encode_async(writer, opts).await,
            StructuredComponent::PotionContents(potion_contents) => potion_contents.encode_async(writer, opts).await,
            StructuredComponent::SuspiciousStewEffects(suspicious_stew_effects) => suspicious_stew_effects.encode_async(writer, opts).await,
            StructuredComponent::OminousBottleAmplifier(ominous_bottle_amplifier) => ominous_bottle_amplifier.encode_async(writer, opts).await,
            StructuredComponent::Fireworks(fireworks) => fireworks.encode_async(writer, opts).await,
            StructuredComponent::Invalid => Err(InvalidStructuredComponentEnumError())?,
        }
    }
}

impl NetDecode for StructuredComponent {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        StructuredComponent::read(reader, opts)
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        StructuredComponent::read_async(reader, opts).await
    }
}

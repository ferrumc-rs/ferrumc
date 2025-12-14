use crate::netcode::errors::{
    InvalidStructuredComponentEnumError, NotSupportedStructuredComponentError,
};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};
#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub enum StructuredComponent {
    # [default] Invalid , MaxDamage (crate :: netcode :: components :: damage :: MaxDamage) , Damage (crate :: netcode :: components :: damage :: Damage) , Unbreakable , Enchantments (crate :: netcode :: components :: enchantments :: enchantments_collection :: EnchantmentsCollection) , EnchantmentGlintOverride (crate :: netcode :: components :: enchantments :: enchantment_glint_override :: EnchantmentGlintOverride) , Enchantable (crate :: netcode :: components :: enchantments :: enchantable :: Enchantable) , StoredEnchantments (crate :: netcode :: components :: enchantments :: enchantments_collection :: EnchantmentsCollection) , PotionContents (crate :: netcode :: components :: potion_contents :: PotionContents) , SuspiciousStewEffects (crate :: netcode :: components :: suspicious_stew_effects :: SuspiciousStewEffects) , OminousBottleAmplifier (crate :: netcode :: components :: ominous_bottle_amplifier :: OminousBottleAmplifier) , Fireworks (crate :: netcode :: components :: fireworks :: Fireworks) , }
impl StructuredComponent {
    pub fn to_id(&self) -> Result<VarInt, InvalidStructuredComponentEnumError> {
        match self {
            StructuredComponent::Invalid => Err(InvalidStructuredComponentEnumError()),
            StructuredComponent::MaxDamage(_) => Ok(VarInt::from(2i32)),
            StructuredComponent::Damage(_) => Ok(VarInt::from(3i32)),
            StructuredComponent::Unbreakable => Ok(VarInt::from(4i32)),
            StructuredComponent::Enchantments(_) => Ok(VarInt::from(10i32)),
            StructuredComponent::EnchantmentGlintOverride(_) => Ok(VarInt::from(18i32)),
            StructuredComponent::Enchantable(_) => Ok(VarInt::from(27i32)),
            StructuredComponent::StoredEnchantments(_) => Ok(VarInt::from(34i32)),
            StructuredComponent::PotionContents(_) => Ok(VarInt::from(42i32)),
            StructuredComponent::SuspiciousStewEffects(_) => Ok(VarInt::from(44i32)),
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
        let id = self.to_id().map_err(|e| NetEncodeError::from(e))?;
        id.encode(writer, opts)?;
        match self {
            StructuredComponent::Invalid => unreachable!(),
            StructuredComponent::MaxDamage(inner) => inner.encode(writer, opts),
            StructuredComponent::Damage(inner) => inner.encode(writer, opts),
            StructuredComponent::Unbreakable => Ok(()),
            StructuredComponent::Enchantments(inner) => inner.encode(writer, opts),
            StructuredComponent::EnchantmentGlintOverride(inner) => inner.encode(writer, opts),
            StructuredComponent::Enchantable(inner) => inner.encode(writer, opts),
            StructuredComponent::StoredEnchantments(inner) => inner.encode(writer, opts),
            StructuredComponent::PotionContents(inner) => inner.encode(writer, opts),
            StructuredComponent::SuspiciousStewEffects(inner) => inner.encode(writer, opts),
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
        let id = self.to_id().map_err(|e| NetEncodeError::from(e))?;
        id.encode_async(writer, opts).await?;
        match self {
            StructuredComponent::Invalid => unreachable!(),
            StructuredComponent::MaxDamage(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::Damage(inner) => inner.encode_async(writer, opts).await,
            StructuredComponent::Unbreakable => Ok(()),
            StructuredComponent::Enchantments(inner) => inner.encode_async(writer, opts).await,
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
        let _probably_data_length = VarInt::decode(reader, opts)?;
        match id . 0 { 2i32 => { Ok (StructuredComponent :: MaxDamage (crate :: netcode :: components :: damage :: MaxDamage :: decode (reader , opts) ?)) } , 3i32 => { Ok (StructuredComponent :: Damage (crate :: netcode :: components :: damage :: Damage :: decode (reader , opts) ?)) } , 4i32 => { Ok (StructuredComponent :: Unbreakable) } , 10i32 => { Ok (StructuredComponent :: Enchantments (crate :: netcode :: components :: enchantments :: enchantments_collection :: EnchantmentsCollection :: decode (reader , opts) ?)) } , 18i32 => { Ok (StructuredComponent :: EnchantmentGlintOverride (crate :: netcode :: components :: enchantments :: enchantment_glint_override :: EnchantmentGlintOverride :: decode (reader , opts) ?)) } , 27i32 => { Ok (StructuredComponent :: Enchantable (crate :: netcode :: components :: enchantments :: enchantable :: Enchantable :: decode (reader , opts) ?)) } , 34i32 => { Ok (StructuredComponent :: StoredEnchantments (crate :: netcode :: components :: enchantments :: enchantments_collection :: EnchantmentsCollection :: decode (reader , opts) ?)) } , 42i32 => { Ok (StructuredComponent :: PotionContents (crate :: netcode :: components :: potion_contents :: PotionContents :: decode (reader , opts) ?)) } , 44i32 => { Ok (StructuredComponent :: SuspiciousStewEffects (crate :: netcode :: components :: suspicious_stew_effects :: SuspiciousStewEffects :: decode (reader , opts) ?)) } , 54i32 => { Ok (StructuredComponent :: OminousBottleAmplifier (crate :: netcode :: components :: ominous_bottle_amplifier :: OminousBottleAmplifier :: decode (reader , opts) ?)) } , 60i32 => { Ok (StructuredComponent :: Fireworks (crate :: netcode :: components :: fireworks :: Fireworks :: decode (reader , opts) ?)) } , _ => { Err (NotSupportedStructuredComponentError (id) . into ()) } }
    }
    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let id = VarInt::decode_async(reader, opts).await?;
        let _probably_data_length = VarInt::decode_async(reader, opts).await?;
        match id . 0 { 2i32 => { Ok (StructuredComponent :: MaxDamage (crate :: netcode :: components :: damage :: MaxDamage :: decode_async (reader , opts) . await ?)) } , 3i32 => { Ok (StructuredComponent :: Damage (crate :: netcode :: components :: damage :: Damage :: decode_async (reader , opts) . await ?)) } , 4i32 => { Ok (StructuredComponent :: Unbreakable) } , 10i32 => { Ok (StructuredComponent :: Enchantments (crate :: netcode :: components :: enchantments :: enchantments_collection :: EnchantmentsCollection :: decode_async (reader , opts) . await ?)) } , 18i32 => { Ok (StructuredComponent :: EnchantmentGlintOverride (crate :: netcode :: components :: enchantments :: enchantment_glint_override :: EnchantmentGlintOverride :: decode_async (reader , opts) . await ?)) } , 27i32 => { Ok (StructuredComponent :: Enchantable (crate :: netcode :: components :: enchantments :: enchantable :: Enchantable :: decode_async (reader , opts) . await ?)) } , 34i32 => { Ok (StructuredComponent :: StoredEnchantments (crate :: netcode :: components :: enchantments :: enchantments_collection :: EnchantmentsCollection :: decode_async (reader , opts) . await ?)) } , 42i32 => { Ok (StructuredComponent :: PotionContents (crate :: netcode :: components :: potion_contents :: PotionContents :: decode_async (reader , opts) . await ?)) } , 44i32 => { Ok (StructuredComponent :: SuspiciousStewEffects (crate :: netcode :: components :: suspicious_stew_effects :: SuspiciousStewEffects :: decode_async (reader , opts) . await ?)) } , 54i32 => { Ok (StructuredComponent :: OminousBottleAmplifier (crate :: netcode :: components :: ominous_bottle_amplifier :: OminousBottleAmplifier :: decode_async (reader , opts) . await ?)) } , 60i32 => { Ok (StructuredComponent :: Fireworks (crate :: netcode :: components :: fireworks :: Fireworks :: decode_async (reader , opts) . await ?)) } , _ => { Err (NotSupportedStructuredComponentError (id) . into ()) } }
    }
}

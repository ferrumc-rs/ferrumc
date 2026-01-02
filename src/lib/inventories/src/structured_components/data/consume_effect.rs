use crate::structured_components::data::consume_effect::ConsumeEffectData::{
    ApplyEffects, ClearAllEffects, Invalid, PlaySound, RemoveEffects, TeleportRandomly,
};
use crate::structured_components::data::potion_effect::PotionEffect;
use crate::structured_components::data::{HashableF32, IdSet, SoundEvent};
use ferrumc_macros::{NetDecode, NetEncode};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct ConsumeEffect {
    pub type_id: VarInt,
    pub data: ConsumeEffectData,
}

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct TeleportData {
    pub diameter: HashableF32,
}

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct Effects {
    pub effects: LengthPrefixedVec<PotionEffect>,
    pub probability: HashableF32,
}

#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub enum ConsumeEffectData {
    #[default]
    Invalid,
    ApplyEffects(Effects),
    RemoveEffects(IdSet),
    ClearAllEffects,
    TeleportRandomly(TeleportData),
    PlaySound(SoundEvent),
}

impl NetDecode for ConsumeEffectData {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let type_id = VarInt::decode(reader, opts)?.0;

        match type_id {
            0 => Ok(ApplyEffects(Effects::decode(reader, opts)?)),
            1 => Ok(RemoveEffects(IdSet::decode(reader, opts)?)),
            2 => Ok(ClearAllEffects),
            3 => Ok(TeleportRandomly(TeleportData::decode(reader, opts)?)),
            4 => Ok(PlaySound(SoundEvent::decode(reader, opts)?)),
            _ => Err(NetDecodeError::ExternalError(
                format!("Unknown ConsumeEffectData. Found {}", type_id).into(),
            )),
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let type_id = VarInt::decode_async(reader, opts).await?.0;

        match type_id {
            0 => Ok(ApplyEffects(Effects::decode_async(reader, opts).await?)),
            1 => Ok(RemoveEffects(IdSet::decode_async(reader, opts).await?)),
            2 => Ok(ClearAllEffects),
            3 => Ok(TeleportRandomly(
                TeleportData::decode_async(reader, opts).await?,
            )),
            4 => Ok(PlaySound(SoundEvent::decode_async(reader, opts).await?)),
            _ => Err(NetDecodeError::ExternalError(
                format!("Unknown ConsumeEffectData. Found {}", type_id).into(),
            )),
        }
    }
}

impl NetEncode for ConsumeEffectData {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        match self {
            Invalid => Err(NetEncodeError::ExternalError(
                "ConsumeEffectData must be set. Encountered none.".into(),
            ))?,
            ApplyEffects(inner) => {
                VarInt::new(0).encode(writer, opts)?;
                inner.encode(writer, opts)
            }
            RemoveEffects(inner) => {
                VarInt::new(1).encode(writer, opts)?;
                inner.encode(writer, opts)
            }
            ClearAllEffects => VarInt::new(2).encode(writer, opts),
            TeleportRandomly(inner) => {
                VarInt::new(3).encode(writer, opts)?;
                inner.encode(writer, opts)
            }
            PlaySound(inner) => {
                VarInt::new(4).encode(writer, opts)?;
                inner.encode(writer, opts)
            }
        }
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        match self {
            Invalid => Err(NetEncodeError::ExternalError(
                "ConsumeEffectData must be set. Encountered none.".into(),
            ))?,
            ApplyEffects(inner) => {
                VarInt::new(0).encode_async(writer, opts).await?;
                inner.encode_async(writer, opts).await
            }
            RemoveEffects(inner) => {
                VarInt::new(1).encode_async(writer, opts).await?;
                inner.encode_async(writer, opts).await
            }
            ClearAllEffects => VarInt::new(2).encode_async(writer, opts).await,
            TeleportRandomly(inner) => {
                VarInt::new(3).encode_async(writer, opts).await?;
                inner.encode_async(writer, opts).await
            }
            PlaySound(inner) => {
                VarInt::new(4).encode_async(writer, opts).await?;
                inner.encode_async(writer, opts).await
            }
        }
    }
}

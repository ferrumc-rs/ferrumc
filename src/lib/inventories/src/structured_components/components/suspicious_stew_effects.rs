use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub struct SuspiciousStewEffects {
    pub effects: LengthPrefixedVec<StewPotionEffect>,
}

#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub struct StewPotionEffect {
    pub effect_id: VarInt,
    pub duration: VarInt,
}

impl NetDecode for StewPotionEffect {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let effect_id = VarInt::decode(reader, opts)?;
        let duration = VarInt::decode(reader, opts)?;

        Ok(StewPotionEffect {
            effect_id,
            duration,
        })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let effect_id = VarInt::decode_async(reader, opts).await?;
        let duration = VarInt::decode_async(reader, opts).await?;

        Ok(StewPotionEffect {
            effect_id,
            duration,
        })
    }
}

impl NetEncode for StewPotionEffect {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.effect_id.encode(writer, opts)?;
        self.duration.encode(writer, opts)?;

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        self.effect_id.encode_async(writer, opts).await?;
        self.duration.encode_async(writer, opts).await?;

        Ok(())
    }
}

impl NetDecode for SuspiciousStewEffects {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let _data_length = VarInt::decode(reader, opts)?;
        let effects = LengthPrefixedVec::decode(reader, opts)?;

        Ok(SuspiciousStewEffects { effects })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let _data_length = VarInt::decode_async(reader, opts).await?;
        let effects = LengthPrefixedVec::decode_async(reader, opts).await?;

        Ok(SuspiciousStewEffects { effects })
    }
}

impl NetEncode for SuspiciousStewEffects {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.effects.encode(writer, opts)?;

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        self.effects.encode_async(writer, opts).await?;

        Ok(())
    }
}

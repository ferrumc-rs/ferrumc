use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

///minecraft:fireworks
#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub struct Fireworks {
    pub flight_duration: VarInt,
    pub explosions: LengthPrefixedVec<FireworkExplosion>,
}

#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub struct FireworkExplosion {
    pub shape: VarInt,
    pub colors: LengthPrefixedVec<i32>,
    pub fade_colors: LengthPrefixedVec<i32>,
    pub has_trail: bool,
    pub has_twinkle: bool,
}

impl NetDecode for FireworkExplosion {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let _data_length = VarInt::decode(reader, opts)?;
        let shape = VarInt::decode(reader, opts)?;
        let colors = LengthPrefixedVec::decode(reader, opts)?;
        let fade_colors = LengthPrefixedVec::decode(reader, opts)?;
        let has_trail = bool::decode(reader, opts)?;
        let has_twinkle = bool::decode(reader, opts)?;

        Ok(FireworkExplosion {
            shape,
            colors,
            fade_colors,
            has_trail,
            has_twinkle,
        })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let _data_length = VarInt::decode_async(reader, opts).await?;
        let shape = VarInt::decode_async(reader, opts).await?;
        let colors = LengthPrefixedVec::decode_async(reader, opts).await?;
        let fade_colors = LengthPrefixedVec::decode_async(reader, opts).await?;
        let has_trail = bool::decode_async(reader, opts).await?;
        let has_twinkle = bool::decode_async(reader, opts).await?;

        Ok(FireworkExplosion {
            shape,
            colors,
            fade_colors,
            has_trail,
            has_twinkle,
        })
    }
}

impl NetEncode for FireworkExplosion {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.shape.encode(writer, opts)?;
        self.colors.encode(writer, opts)?;
        self.fade_colors.encode(writer, opts)?;
        self.has_trail.encode(writer, opts)?;
        self.has_twinkle.encode(writer, opts)?;

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        self.shape.encode_async(writer, opts).await?;
        self.colors.encode_async(writer, opts).await?;
        self.fade_colors.encode_async(writer, opts).await?;
        self.has_trail.encode_async(writer, opts).await?;
        self.has_twinkle.encode_async(writer, opts).await?;

        Ok(())
    }
}

impl NetDecode for Fireworks {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let flight_duration = VarInt::decode(reader, opts)?;
        let explosions = LengthPrefixedVec::decode(reader, opts)?;

        Ok(Fireworks {
            flight_duration,
            explosions,
        })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let flight_duration = VarInt::decode_async(reader, opts).await?;
        let explosions = LengthPrefixedVec::decode_async(reader, opts).await?;

        Ok(Fireworks {
            flight_duration,
            explosions,
        })
    }
}

impl NetEncode for Fireworks {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.flight_duration.encode(writer, opts)?;
        self.explosions.encode(writer, opts)?;

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        self.flight_duration.encode_async(writer, opts).await?;
        self.explosions.encode_async(writer, opts).await?;

        Ok(())
    }
}

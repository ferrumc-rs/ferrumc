use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

/// minecraft:max_damage
/// The maximum damage the item can take before breaking.
#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub struct MaxDamage {
    pub max_damage: VarInt,
}

/// minecraft:damage
/// The current damage (wear) of the item.
#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub struct Damage {
    pub damage: VarInt,
}

impl NetDecode for MaxDamage {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let _data_length = VarInt::decode(reader, opts)?;
        let max_damage = VarInt::decode(reader, opts)?;
        Ok(MaxDamage { max_damage })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let _data_length = VarInt::decode_async(reader, opts).await?;
        let max_damage = VarInt::decode_async(reader, opts).await?;
        Ok(MaxDamage { max_damage })
    }
}

impl NetEncode for MaxDamage {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.max_damage.encode(writer, opts)?;
        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        self.max_damage.encode_async(writer, opts).await?;
        Ok(())
    }
}

impl NetDecode for Damage {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let _data_length = VarInt::decode(reader, opts)?;
        let damage = VarInt::decode(reader, opts)?;
        Ok(Damage { damage })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let _data_length = VarInt::decode_async(reader, opts).await?;
        let damage = VarInt::decode_async(reader, opts).await?;
        Ok(Damage { damage })
    }
}

impl NetEncode for Damage {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.damage.encode(writer, opts)?;
        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        self.damage.encode_async(writer, opts).await?;
        Ok(())
    }
}

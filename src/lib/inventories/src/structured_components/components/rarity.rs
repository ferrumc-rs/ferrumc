use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub struct Rarity {
    pub rarity: VarInt,
}

impl NetEncode for Rarity {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.rarity.encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.rarity.encode_async(writer, opts).await
    }
}

impl NetDecode for Rarity {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let _data_length = VarInt::decode(reader, opts)?;
        let rarity = VarInt::decode(reader, opts)?;
        Ok(Rarity { rarity })
    }

    async fn decode_async<R: AsyncRead + Unpin>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let _data_length = VarInt::decode_async(reader, opts).await?;
        let rarity = VarInt::decode_async(reader, opts).await?;
        Ok(Rarity { rarity })
    }
}
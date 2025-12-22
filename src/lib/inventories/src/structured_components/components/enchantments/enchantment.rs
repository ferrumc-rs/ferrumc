use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub struct Enchantment {
    pub type_id: VarInt,
    pub level: VarInt,
}

impl NetDecode for Enchantment {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let _data_length = VarInt::decode(reader, opts)?;
        let type_id = VarInt::decode(reader, opts)?;
        let level = VarInt::decode(reader, opts)?;

        Ok(Self { type_id, level })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let _data_length = VarInt::decode_async(reader, opts).await?;
        let type_id = VarInt::decode_async(reader, opts).await?;
        let level = VarInt::decode_async(reader, opts).await?;

        Ok(Self { type_id, level })
    }
}

impl NetEncode for Enchantment {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.type_id.encode(writer, opts)?;
        self.level.encode(writer, opts)?;

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        self.type_id.encode_async(writer, opts).await?;
        self.level.encode_async(writer, opts).await?;

        Ok(())
    }
}

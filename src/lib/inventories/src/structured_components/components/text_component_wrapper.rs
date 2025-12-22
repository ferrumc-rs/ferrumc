use crate::structured_components::data::TextComponent;
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub struct TextComponentWrapper {
    pub name: TextComponent,
}

impl NetEncode for TextComponentWrapper {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.name.encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        Ok(self.name.encode_async(writer, opts).await?)
    }
}

impl NetDecode for TextComponentWrapper {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let _data_length = VarInt::decode(reader, opts)?;
        let name = TextComponent::decode(reader, opts)?;

        Ok(TextComponentWrapper { name })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let name = TextComponent::decode_async(reader, opts).await?;

        Ok(TextComponentWrapper { name })
    }
}

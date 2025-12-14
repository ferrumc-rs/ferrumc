use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use crate::netcode::components::enchantments::enchantment::Enchantment;

#[derive(Debug, Clone, Hash, Default, PartialEq)]
///minecraft:enchantments
///minecraft:stored_enchantments
pub struct EnchantmentsCollection {
    pub data : LengthPrefixedVec<Enchantment>,
}

impl NetDecode for EnchantmentsCollection {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let data : LengthPrefixedVec<Enchantment> = LengthPrefixedVec::decode(reader, opts)?;

        Ok(Self { data })
    }

    async fn decode_async<R: AsyncRead + Unpin>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let data : LengthPrefixedVec<Enchantment> = LengthPrefixedVec::decode_async(reader, opts).await?;

        Ok(Self { data })
    }
}

impl NetEncode for EnchantmentsCollection {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.data.encode(writer, opts)?;

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.data.encode_async(writer, opts).await?;

        Ok(())
    }
}
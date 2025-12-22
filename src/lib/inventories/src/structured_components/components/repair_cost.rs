use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub struct RepairCost {
    pub cost: VarInt,
}

impl NetEncode for RepairCost {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.cost.encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.cost.encode_async(writer, opts).await
    }
}

impl NetDecode for RepairCost {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let _data_length = VarInt::decode(reader, opts)?;
        
        let cost = VarInt::decode(reader, opts)?;
        Ok(RepairCost { cost })
    }

    async fn decode_async<R: AsyncRead + Unpin>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let _data_length = VarInt::decode_async(reader, opts).await?;

        let cost = VarInt::decode_async(reader, opts).await?;
        Ok(RepairCost { cost })
    }
}
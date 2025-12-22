use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

/// minecraft:ominous_bottle_amplifier
/// Amplifier for the effect of an ominous bottle.
#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub struct OminousBottleAmplifier {
    pub amplifier: VarInt,
}

impl NetDecode for OminousBottleAmplifier {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let _data_length = VarInt::decode(reader, opts)?;
        let amplifier = VarInt::decode(reader, opts)?;

        Ok(OminousBottleAmplifier { amplifier })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let _data_length = VarInt::decode_async(reader, opts).await?;
        let amplifier = VarInt::decode_async(reader, opts).await?;

        Ok(OminousBottleAmplifier { amplifier })
    }
}

impl NetEncode for OminousBottleAmplifier {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.amplifier.encode(writer, opts)?;

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        self.amplifier.encode_async(writer, opts).await?;

        Ok(())
    }
}

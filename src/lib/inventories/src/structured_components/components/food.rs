use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Food {
    pub nutrition: VarInt,
    pub saturation: f32,
    pub can_always_eat: bool,
}

impl Hash for Food {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.nutrition.hash(state);

        let s_bits = if self.saturation == 0.0 {
            0.0f32.to_bits()
        } else {
            self.saturation.to_bits()
        };
        s_bits.hash(state);

        self.can_always_eat.hash(state);
    }
}

impl NetDecode for Food {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let _data_length = VarInt::decode(reader, opts)?;
        let nutrition = VarInt::decode(reader, opts)?;
        let saturation = f32::decode(reader, opts)?;
        let can_always_eat = bool::decode(reader, opts)?;

        Ok(Self {
            nutrition,
            saturation,
            can_always_eat,
        })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let _data_length = VarInt::decode_async(reader, opts).await?;
        let nutrition = VarInt::decode_async(reader, opts).await?;
        let saturation = f32::decode_async(reader, opts).await?;
        let can_always_eat = bool::decode_async(reader, opts).await?;

        Ok(Self {
            nutrition,
            saturation,
            can_always_eat,
        })
    }
}

impl NetEncode for Food {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.nutrition.encode(writer, opts)?;
        self.saturation.encode(writer, opts)?;
        self.can_always_eat.encode(writer, opts)?;

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        self.nutrition.encode_async(writer, opts).await?;
        self.saturation.encode_async(writer, opts).await?;
        self.can_always_eat.encode_async(writer, opts).await?;

        Ok(())
    }
}

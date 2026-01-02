use crate::structured_components::data::Identifier;
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub enum IdSet {
    #[default]
    Invalid,
    Identifier(Identifier),
    Ids(Vec<VarInt>),
}

impl NetDecode for IdSet {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let discriminator = VarInt::decode(reader, opts)?;

        if discriminator == 0 {
            let identifier = Identifier::decode(reader, opts)?;
            Ok(IdSet::Identifier(identifier))
        } else {
            let count = (discriminator.0 - 1) as usize;
            let mut ids = Vec::with_capacity(count);

            for _ in 0..count {
                ids.push(VarInt::decode(reader, opts)?);
            }

            Ok(IdSet::Ids(ids))
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let discriminator = VarInt::decode_async(reader, opts).await?;

        if discriminator == 0 {
            let identifier = Identifier::decode_async(reader, opts).await?;
            Ok(IdSet::Identifier(identifier))
        } else {
            let count = (discriminator.0 - 1) as usize;
            let mut ids = Vec::with_capacity(count);

            for _ in 0..count {
                ids.push(VarInt::decode_async(reader, opts).await?);
            }

            Ok(IdSet::Ids(ids))
        }
    }
}

impl NetEncode for IdSet {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        match self {
            IdSet::Invalid => Err(NetEncodeError::ExternalError("Invalid IdSet type.".into()))?,
            IdSet::Identifier(id) => {
                VarInt(0).encode(writer, opts)?;
                id.encode(writer, opts)?;
            }
            IdSet::Ids(ids) => {
                VarInt((ids.len() + 1) as i32).encode(writer, opts)?;

                for id in ids {
                    id.encode(writer, opts)?;
                }
            }
        }

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        match self {
            IdSet::Invalid => Err(NetEncodeError::ExternalError("Invalid IdSet type.".into()))?,
            IdSet::Identifier(id) => {
                VarInt(0).encode_async(writer, opts).await?;
                id.encode_async(writer, opts).await?;
            }
            IdSet::Ids(ids) => {
                VarInt((ids.len() + 1) as i32).encode_async(writer, opts).await?;

                for id in ids {
                    id.encode_async(writer, opts).await?;
                }
            }
        }

        Ok(())
    }
}

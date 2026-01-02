use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

///Note that if variant is Id, it contains `registry_id + 1`
#[derive(Debug, Default, Clone, Hash, PartialEq)]
pub enum IdOr<T> {
    #[default]
    Invalid,
    Id(VarInt),
    Data(T),
}

impl<T> NetDecode for IdOr<T>
where
    T: NetDecode,
{
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let id = VarInt::decode(reader, opts)?;

        if id == 0 {
            let data = T::decode(reader, opts)?;
            Ok(IdOr::Data(data))
        } else {
            Ok(IdOr::Id(id))
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let id = VarInt::decode_async(reader, opts).await?;

        if id == 0 {
            let data = T::decode_async(reader, opts).await?;
            Ok(IdOr::Data(data))
        } else {
            Ok(IdOr::Id(id))
        }
    }
}

impl<T> NetEncode for IdOr<T>
where
    T: NetEncode,
{
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        match self {
            IdOr::Id(id) => {
                id.encode(writer, opts)?;
            }
            IdOr::Data(data) => {
                VarInt(0).encode(writer, opts)?;
                data.encode(writer, opts)?;
            }
            IdOr::Invalid => {
                Err(NetEncodeError::ExternalError("Invalid IdOr enum encountered on encode.".into()))?;
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
            IdOr::Id(id) => {
                id.encode_async(writer, opts).await?;
            }
            IdOr::Data(data) => {
                VarInt(0).encode_async(writer, opts).await?;
                data.encode_async(writer, opts).await?;
            }
            IdOr::Invalid => {
                Err(NetEncodeError::ExternalError("Invalid IdOr enum encountered on encode.".into()))?;
            }
        }

        Ok(())
    }
}

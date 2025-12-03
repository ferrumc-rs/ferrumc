use crate::codec::decode::{NetDecode, NetDecodeOpts, errors::NetDecodeError};
use crate::codec::encode::{NetEncode, NetEncodeOpts, errors::NetEncodeError};
use ferrumc_core::resources::data_pack::DataPackEntry;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

// --- Encoding ---
impl NetEncode for DataPackEntry {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.namespace.encode(writer, opts)?;
        self.id.encode(writer, opts)?;
        self.version.encode(writer, opts)?;
        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        self.namespace.encode_async(writer, opts).await?;
        self.id.encode_async(writer, opts).await?;
        self.version.encode_async(writer, opts).await?;
        Ok(())
    }
}

// --- Decoding ---
impl NetDecode for DataPackEntry {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        Ok(DataPackEntry {
            namespace: String::decode(reader, opts)?,
            id: String::decode(reader, opts)?,
            version: String::decode(reader, opts)?,
        })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        Ok(DataPackEntry {
            namespace: String::decode_async(reader, opts).await?,
            id: String::decode_async(reader, opts).await?,
            version: String::decode_async(reader, opts).await?,
        })
    }
}

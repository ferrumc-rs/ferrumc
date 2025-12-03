use crate::codec::encode::errors::NetEncodeError;
use crate::codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_nbt::de::borrow::NbtTape;
use std::io::Write;
use tokio::io::AsyncWrite;

// Restore the NetEncode implementation here
impl<'a> NetEncode for NbtTape<'a> {
    fn encode<W: Write>(
        &self,
        writer: &mut W,
        _opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        // Call the pure IO method you just added to NbtTape
        self.write_nbt(writer)
            .map_err(|e| NetEncodeError::External(e.to_string()))
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        _opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        use tokio::io::AsyncWriteExt;

        writer
            .write_all(self.as_bytes())
            .await
            .map_err(NetEncodeError::Io)
    }
}

/// A wrapper for pre-serialized NBT bytes.
/// Encodes as raw bytes without length prefixing (because NBT is self-terminating).
pub struct RawNbt(pub Vec<u8>);

impl NetEncode for RawNbt {
    fn encode<W: Write>(
        &self,
        writer: &mut W,
        _opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        writer.write_all(&self.0).map_err(NetEncodeError::Io)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        _opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        use tokio::io::AsyncWriteExt;
        writer.write_all(&self.0).await.map_err(NetEncodeError::Io)
    }
}

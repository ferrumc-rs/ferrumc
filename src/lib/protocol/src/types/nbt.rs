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

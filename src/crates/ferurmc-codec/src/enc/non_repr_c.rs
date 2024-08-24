use tokio::io::{AsyncWrite, AsyncWriteExt};
use crate::enc::Encode;
use crate::network_types::varint::VarInt;

impl Encode for String {
    async fn encode<W>(&self, writer: &mut W) -> crate::error::Result<()>
    where
        W: AsyncWrite + Unpin
    {
        let len = VarInt::new(self.len() as i32);
        len.encode(writer).await?;
        writer
            .write_all(self.as_bytes())
            .await?;

        Ok(())
    }
}
use std::io;
use tokio::io::{AsyncWrite, AsyncWriteExt};
pub enum Tag {
    Byte(i8)
}

pub trait NBTSerialize {
    async fn nbt_serialize<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> io::Result<()>;
}

impl NBTSerialize for bool {
    async fn nbt_serialize<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> io::Result<()> {
        // TAG_Byte
        writer.write_u8(1).await?;

        // Name length (0 for now, we're not using names in this simple version)
        writer.write_u16(0).await?;

        // Value
        writer.write_i8(*self as i8).await?;

        Ok(())
    }
}

pub async fn serialize_to_nbt<T: NBTSerialize, W: AsyncWrite + Unpin + Send>(value: &T, writer: &mut W) -> io::Result<()> {
    value.nbt_serialize(writer).await
}

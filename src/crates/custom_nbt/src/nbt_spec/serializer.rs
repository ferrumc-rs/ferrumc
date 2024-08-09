use std::io;
use tokio::io::AsyncWrite;
use crate::nbt_spec::tags::NamedTag;

pub trait NBTSerialize {
    async fn nbt_serialize<W: AsyncWrite + Unpin + Send>(&self, name: &str, writer: &mut W) -> io::Result<()>;
}

pub async fn serialize_to_nbt<T: NBTSerialize, W: AsyncWrite + Unpin + Send>(
    named_tag: &NamedTag<T>,
    writer: &mut W,
) -> io::Result<()> {
    named_tag.value.nbt_serialize(&named_tag.name, writer).await
}


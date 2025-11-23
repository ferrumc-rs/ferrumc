use std::io::Write;
use tokio::io::AsyncWrite;

mod r#impl;

pub trait NBTSerializable {
    fn serialize<W: Write>(&self, buf: &mut W, options: &NBTSerializeOptions<'_>);
    #[allow(async_fn_in_trait)]
    async fn serialize_async<W: AsyncWrite + Unpin>(&self, buf: &mut W, options: &NBTSerializeOptions<'_>);
    fn id() -> u8;
}

/// Options for serializing NBT data.
/// To simplify root serialization.
#[derive(PartialEq, Debug)]
pub enum NBTSerializeOptions<'a> {
    None,
    WithHeader(&'a str),
    Network,
    Flatten,
}

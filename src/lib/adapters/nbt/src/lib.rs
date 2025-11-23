#![allow(unsafe_code)]
pub mod de;
pub mod errors;
pub mod ser;

pub type Result<T> = std::result::Result<T, NBTError>;

use std::io::Write;
use tokio::io::AsyncWrite;
pub use de::borrow::{NbtTape, NbtTapeElement};
pub use de::converter::FromNbt;
pub use errors::NBTError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
pub use ser::{NBTSerializable, NBTSerializeOptions};

pub struct NBT<T> {
    inner: T,
}

impl<T> NBT<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: NBTSerializable> NetEncode for NBT<T> {
    fn encode<W: Write>(&self, writer: &mut W, _opts: &NetEncodeOpts) -> std::result::Result<(), NetEncodeError> {
        Ok(self.inner.serialize(writer, &NBTSerializeOptions::Network))
    }

    async fn encode_async<W: AsyncWrite + Unpin>(&self, writer: &mut W, _opts: &NetEncodeOpts) -> std::result::Result<(), NetEncodeError> {
        Ok(self.inner.serialize_async(writer, &NBTSerializeOptions::Network).await)
    }
}
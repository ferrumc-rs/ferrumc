#![allow(unsafe_code)]
use tokio::io::AsyncReadExt;
pub mod de;
pub mod errors;
pub mod ser;

pub type Result<T> = std::result::Result<T, NBTError>;

pub use de::borrow::{NbtTape, NbtTapeElement};
pub use de::converter::FromNbt;
pub use errors::NBTError;
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
pub use ser::{NBTSerializable, NBTSerializeOptions};
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

pub use tokio;

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

impl<T: for<'a> FromNbt<'a>> NetDecode for NBT<T> {
    fn decode<R: Read>(reader: &mut R, _opts: &NetDecodeOpts) -> std::result::Result<Self, NetDecodeError> {
        let bytes = reader.bytes().into_iter().map(|b| b.unwrap()).collect::<Vec<u8>>();
        let tape = NbtTape::new(&bytes);
        Ok(NBT { inner: T::from_nbt(&tape, tape.get("").unwrap()).map_err(|_| NetDecodeError::ExternalError("NBT Parse Error".into()))? })
    }

    async fn decode_async<R: AsyncRead + Unpin>(reader: &mut R, _opts: &NetDecodeOpts) -> std::result::Result<Self, NetDecodeError> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let tape = NbtTape::new(&bytes);
        Ok(NBT { inner: T::from_nbt(&tape, tape.get("").unwrap()).map_err(|_| NetDecodeError::ExternalError("NBT Parse error".into()))? })
    }
}
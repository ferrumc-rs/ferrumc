use crate::{FromNbt, NBTSerializable, NBTSerializeOptions, NbtTape};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use std::fmt::Debug;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite};

pub struct NBT<T> {
    inner: T,
}

impl<T> NBT<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: NBTSerializable> NetEncode for NBT<T> {
    fn encode<W: Write>(
        &self,
        writer: &mut W,
        _opts: &NetEncodeOpts,
    ) -> std::result::Result<(), NetEncodeError> {
        self.inner.serialize(writer, &NBTSerializeOptions::Network);
        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        _opts: &NetEncodeOpts,
    ) -> std::result::Result<(), NetEncodeError> {
        self.inner
            .serialize_async(writer, &NBTSerializeOptions::Network)
            .await;
        Ok(())
    }
}

impl<T: for<'a> FromNbt<'a>> NetDecode for NBT<T> {
    fn decode<R: Read>(
        reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> std::result::Result<Self, NetDecodeError> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;
        let tape = NbtTape::new(&bytes);
        Ok(NBT {
            inner: T::from_nbt(&tape, tape.get("").unwrap())
                .map_err(|_| NetDecodeError::ExternalError("NBT Parse Error".into()))?,
        })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> std::result::Result<Self, NetDecodeError> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let tape = NbtTape::new(&bytes);
        Ok(NBT {
            inner: T::from_nbt(&tape, tape.get("").unwrap())
                .map_err(|_| NetDecodeError::ExternalError("NBT Parse error".into()))?,
        })
    }
}

impl<T> From<T> for NBT<T> {
    fn from(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Default> Default for NBT<T> {
    fn default() -> Self {
        Self {
            inner: T::default(),
        }
    }
}

impl<T: Clone> Clone for NBT<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: PartialEq> PartialEq for NBT<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T: Debug> Debug for NBT<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> Deref for NBT<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for NBT<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

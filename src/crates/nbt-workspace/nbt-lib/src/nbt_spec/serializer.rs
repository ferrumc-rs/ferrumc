use std::io::Write;
use std::marker::PhantomData;

use ferrumc_codec::enc::Encode;
use impls::NBTFieldType;
use tokio::io::AsyncWrite;

use crate::NBTResult;

pub mod impls;
pub mod nbt_tag_to_writer;
pub mod tag_types;

pub trait NBTSerialize: NBTFieldType {
    fn serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()>;
}

pub trait NBTAnonymousType {
    fn tag_type() -> u8;
}

pub struct NBTSerializeToEncodeWrapper<'a, T: NBTSerialize>(pub &'a T, PhantomData<T>);

impl<'a, T: NBTSerialize> NBTSerializeToEncodeWrapper<'a, T> {
    pub fn new(value: &'a T) -> Self {
        NBTSerializeToEncodeWrapper(value, PhantomData)
    }
}

/// Just a marker trait to identify NBTCompound.
/// This is used to implement network Serialize for : NBTSerialize + NBTCompoundMarker
pub trait NBTCompoundMarker {
    fn wrapped<T>(t: &T) -> NBTSerializeToEncodeWrapper<T>
    where
        T: NBTSerialize;
}

impl<'a, T: NBTSerialize> Encode for NBTSerializeToEncodeWrapper<'a, T> {
    async fn encode<W>(&self, writer: &mut W) -> ferrumc_codec::Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        // Header (TAG_COMPOUND, empty name) is written by the caller. Usually the derive macro.

        let mut sync_bytes = Vec::new();
        self.0
            .serialize(&mut sync_bytes)
            .map_err(ferrumc_codec::error::CodecError::from_external_error)?;
        {
            use tokio::io::AsyncWriteExt;
            writer.write_all(&sync_bytes).await?;
        }

        // End tag is also written by the caller.

        Ok(())
    }
}

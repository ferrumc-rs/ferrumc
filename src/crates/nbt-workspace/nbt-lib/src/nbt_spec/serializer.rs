use crate::NBTResult;
use ferrumc_codec::enc::Encode;
use impls::NBTFieldType;
use std::io::Write;
use thiserror::__private::AsDynError;
use tokio::io::AsyncWrite;

pub mod impls;
pub mod tag_types;
pub mod nbt_tag_to_writer;


pub trait NBTSerialize: NBTFieldType {
    fn serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()>;
}

pub trait NBTAnonymousType {
    fn tag_type() -> u8;
}


pub struct NBTSerializeToEncodeWrapper<T: NBTSerialize>(pub T);

/// Just a marker trait to identify NBTCompound.
/// This is used to implement network Serialize for : NBTSerialize + NBTCompoundMarker
pub trait NBTCompoundMarker {
    fn wrapped<T>(t: T) -> NBTSerializeToEncodeWrapper<T>
    where
        T: NBTSerialize;
}

impl<T> Encode for NBTSerializeToEncodeWrapper<T>
where
    T: NBTSerialize,
{
    async fn encode<W>(&self, writer: &mut W) -> ferrumc_codec::Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        let mut sync_bytes = Vec::new();
        self.0.serialize(&mut sync_bytes)
            .map_err(ferrumc_codec::error::CodecError::from_external_error)?;
        {
            use tokio::io::AsyncWriteExt;
            writer.write_all(&sync_bytes).await?;
        }
        Ok(())
    }
}
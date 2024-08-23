use crate::utils::error::Error;
use crate::utils::impls::type_impls::Encode;
use nbt_lib::NBTSerialize;
use tokio::io::AsyncWrite;
use nbt_lib::nbt_spec::serializer::NBTCompoundMarker;

pub mod bitset;
pub mod position;
pub mod varint;
pub mod varlong;
pub mod velocity;


impl<S> Encode for S
where
    S: NBTSerialize + NBTCompoundMarker,
{
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + Unpin,
    {
        let mut pseudo_cursor = Vec::new();

        self.serialize(&mut pseudo_cursor)?;

        {
            use tokio::io::AsyncWriteExt;
            bytes.write_all(&pseudo_cursor).await?;
        }
        Ok(())
    }
}
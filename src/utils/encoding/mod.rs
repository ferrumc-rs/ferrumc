use std::io::{Cursor, Write};
use ferrumc_codec::enc::Encode;
use nbt_lib::nbt_spec::serializer::NBTCompoundMarker;
use nbt_lib::NBTSerialize;
use tokio::io::AsyncWrite;
use crate::utils::error::Error;

pub mod bitset;
pub mod position;
pub mod velocity;

/*impl<S: NBTSerialize> Encode for &S {
    async fn encode<T>(self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + Unpin
    {
        let mut sync_bytes = Cursor::new(Vec::new());
        self.serialize(&mut sync_bytes)?;
        {
            use tokio::io::AsyncWriteExt;
            bytes.write_all(&sync_bytes.into_inner()).await?;
        }
        Ok(())
    }
}
*/
/*pub struct Enc<S, M>(pub S, std::marker::PhantomData<M>);

impl<S: NBTSerialize> Encode for Enc<S, dyn NBTSerialize> {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + Unpin
    {
        let mut sync_bytes = Cursor::new(Vec::new());
        self.0.serialize(&mut sync_bytes)?;
        {
            use tokio::io::AsyncWriteExt;
            bytes.write_all(&sync_bytes.into_inner()).await?;
        }
        Ok(())
    }
}

impl<S: Encode> Encode for Enc<S, dyn Encode> {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + Unpin
    {
        self.0.encode(bytes).await
    }
}*/
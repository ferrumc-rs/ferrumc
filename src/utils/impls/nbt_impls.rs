
/*pub trait SpecEncode {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + Unpin;
}

impl<S: NBTSerialize> SpecEncode for S {
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

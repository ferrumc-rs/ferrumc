
macro_rules! impl_primitives {
    ($($ty:ty),*) => {
        $(
        impl Encode for $ty
        {
            async fn encode<W>(&self, writer: &mut W) -> crate::error::Result<()> where W: AsyncWrite + Unpin {
                let bytes = self.to_be_bytes();
                writer.write_all(&bytes).await?;

                Ok(())
            }
        }
        )*
    };
}
impl_primitives!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);

impl Encode for bool {
    async fn encode<W>(&self, writer: &mut W) -> crate::error::Result<()>
    where
        W: AsyncWrite + Unpin
    {
        let byte = if *self { 1 } else { 0 } as u8.to_be_bytes();
        writer.write_all(&byte).await?;
        Ok(())
    }
}
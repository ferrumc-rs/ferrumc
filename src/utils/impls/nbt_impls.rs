use tokio::io::{AsyncSeek, AsyncWrite};
use nbt_lib::NBTTag;
use crate::utils::error::Error;
use crate::utils::impls::type_impls::Encode;


impl Encode for NBTTag {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin
    {
        let mut buffer = Vec::new();
        self.ser(&mut buffer)?;
        Ok(())
    }
}
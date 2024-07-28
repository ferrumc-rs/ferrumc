use tokio::io::{AsyncSeek, AsyncWrite};
use crate::utils::encoding::varint::VarInt;
use crate::utils::error::Error;
use crate::utils::type_impls::Encode;

pub struct BitSet {
    len: VarInt,
    data: Vec<u64>,
}

impl Encode for BitSet {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin
    {
        self.len.encode(bytes).await?;
        self.data.encode(bytes).await?;
        Ok(())
    }
}
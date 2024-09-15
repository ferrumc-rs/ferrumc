use tokio::io::AsyncWrite;

use crate::{network_types::varint::VarInt, prelude::*};

mod non_primitives;
mod primitives;

pub trait NetEncode {
    #[allow(async_fn_in_trait)]
    async fn net_encode_no_size<W>(&self, writer: &mut W) -> Result<()>
    where
        W: AsyncWrite + Unpin;

    #[allow(async_fn_in_trait)]
    async fn net_encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        let mut data_buffer = Vec::new();

        self.net_encode_no_size(&mut data_buffer).await;

        let length = VarInt::new(data_buffer.len() as i32);

        length.net_encode(writer).await?;

        Ok(())
    }

    #[allow(async_fn_in_trait)]
    async fn net_compressed_encode<W>(&self, writer: &mut W, threshold: i32) -> Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        let mut data_buffer = Vec::new();

        self.net_encode_no_size(&mut data_buffer).await;

        let length = VarInt::new(data_buffer.len() as i32);

        length.net_encode(writer).await?;

        if data_buffer.len() as i32 >= threshold {
            let compressed_data = compress::compress(data_buffer.as_slice());

            let compressed_length = VarInt::new(compressed_data.len() as i32);

            compressed_length.net_encode(writer).await?;

            writer.net_encode(&compressed_data).await?;
        } else {
            let uncompressed_length = VarInt::new(0);

            uncompressed_length.net_encode(writer).await?;

            writer.write_all(&data_buffer).await?;
        }

        Ok(())
    }

    #[allow(async_fn_in_trait)]
    async fn net_auto_encode<W>(
        &self,
        writer: &mut W,
        compressed: bool,
        threshold: i32,
    ) -> Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        if compressed {
            self.net_compressed_encode(writer, threshold).await
        } else {
            self.net_encode_no_size(writer).await
        }
    }
}

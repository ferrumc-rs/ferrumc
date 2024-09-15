use std::io::Write;

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
        use tokio::io::AsyncWriteExt;

        let mut data_buffer = Vec::new();

        self.net_encode_no_size(&mut data_buffer).await?;

        let length = VarInt::new(data_buffer.len() as i32);

        length.net_encode_no_size(writer).await?;

        writer.write_all(&data_buffer).await?;

        Ok(())
    }

    #[allow(async_fn_in_trait)]
    async fn net_compressed_encode<W>(&self, writer: &mut W, threshold: i32) -> Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        use tokio::io::AsyncWriteExt;

        let mut data_buffer = Vec::new();

        self.net_encode_no_size(&mut data_buffer).await?;

        let data_length = VarInt::new(data_buffer.len() as i32);

        // length.net_encode(writer).await?;

        if data_buffer.len() as i32 >= threshold {
            // Compress the data

            let mut e = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
            e.write_all(&data_buffer)?;
            let compressed_buffer = e.finish()?;

            // Write:
            // Packet Length - Length of (Data Length) + Compressed length of (Packet ID + Data)
            // Data Length - Length of uncompressed (Packet ID + Data) or 0
            // Data contents - Packet ID + Data (compressed)

            // Length of data length + compressed length
            let packet_length = compressed_buffer.len() + data_length.get_len() as usize;

            VarInt::from(packet_length as i32) // Length of (Data Length) + Compressed length of (Packet ID + Data)
                .net_encode_no_size(writer)
                .await?;

            data_length.net_encode_no_size(writer).await?; // Length of uncompressed data

            writer.write_all(&compressed_buffer).await?;
        } else {
            let data_length = VarInt::new(0);

            let packet_length = data_buffer.len() + data_length.get_len() as usize;

            VarInt::from(packet_length as i32) // Length of (Data Length) + Compressed length of (Packet ID + Data)
                .net_encode_no_size(writer)
                .await?;

            data_length.net_encode_no_size(writer).await?; // Length of uncompressed data (0)

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

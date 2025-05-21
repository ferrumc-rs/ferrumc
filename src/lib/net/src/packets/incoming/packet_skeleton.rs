use crate::errors::NetError;
use ferrumc_config::statics::get_global_config;
use ferrumc_net_codec::{decode::errors::NetDecodeError, net_types::var_int::VarInt};
use std::io::Cursor;
use std::{fmt::Debug, io::Read};
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;
use tracing::{debug, trace};

pub struct PacketSkeleton {
    pub length: usize,
    pub id: u8,
    pub data: Cursor<Vec<u8>>,
}

impl Debug for PacketSkeleton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PacketSkeleton")
            .field("length", &self.length)
            .field("id", &self.id)
            .finish()
    }
}

impl PacketSkeleton {
    pub async fn new<R: AsyncRead + Unpin>(
        reader: &mut R,
        compressed: bool,
    ) -> Result<Self, NetError> {
        let pak = match compressed {
            true => Self::read_compressed(reader).await,
            false => Self::read_uncompressed(reader).await,
        };
        match pak {
            Ok(p) => {
                if option_env!("FERRUMC_LOG_PACKETS").is_some() {
                    trace!("Received packet: {:?}", p);
                }
                Ok(p)
            }
            Err(e) => {
                if !matches!(e, NetError::ConnectionDropped) {
                    // Don't log connection dropped errors
                    // They are expected when the client disconnects
                    trace!("Error reading packet: {:?}", e);
                } else {
                    // Log connection dropped errors
                    debug!("Connection dropped: {:?}", e);
                }
                Err(e)
            }
        }
    }

    // #[inline(always)]
    async fn read_uncompressed<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self, NetError> {
        let length = VarInt::read_async(reader).await?.0 as usize;
        let mut buf = {
            let mut buf = vec![0; length];
            reader.read_exact(&mut buf).await?;

            Cursor::new(buf)
        };

        let id = VarInt::read_async(&mut buf).await?;

        Ok(Self {
            length,
            id: id.0 as u8,
            data: buf,
        })
    }

    #[inline(always)]
    async fn read_compressed<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self, NetError> {
        let packet_length = VarInt::read_async(reader).await?.0 as usize;
        let data_length = VarInt::read_async(reader).await?.0 as usize;

        // Uncompressed packet when data length is 0
        if data_length == 0 {
            let mut buf = {
                let mut buf = vec![0; packet_length];
                reader.read_exact(&mut buf).await?;

                Cursor::new(buf)
            };

            let id = VarInt::read_async(&mut buf).await?;

            return Ok(Self {
                length: packet_length,
                id: id.0 as u8,
                data: buf,
            });
        }

        let compression_threshold = get_global_config().network_compression_threshold;

        // https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol#Packet_format
        // The Notchian server (but not client) rejects compressed packets smaller than the threshold.
        // Uncompressed packets exceeding the threshold, however, are accepted.
        if data_length < compression_threshold as usize {
            // Compressed packet smaller than threshold
            // Reject packet
            return Err(NetError::DecoderError(
                NetDecodeError::CompressedPacketTooSmall(data_length),
            ));
        }

        // Here, guaranteed that data_length >= compression_threshold
        let mut buf = {
            let mut buf = vec![];
            reader.read_to_end(&mut buf).await?;

            Cursor::new(buf)
        };

        // Decompress data
        let mut decompressed = Vec::new();
        {
            // Scope for decoder
            let mut decoder = flate2::read::ZlibDecoder::new(&mut buf);
            decoder.read_to_end(&mut decompressed)?;
        }

        let mut buf = Cursor::new(decompressed);

        let id = VarInt::read_async(&mut buf).await?;

        Ok(Self {
            length: packet_length,
            id: id.0 as u8,
            data: buf,
        })
    }
}

use crate::errors::{NetError, PacketError};
use crate::ConnState;
use ferrumc_config::server_config::get_global_config;
use ferrumc_macros::lookup_packet;
use ferrumc_net_codec::{decode::errors::NetDecodeError, net_types::var_int::VarInt};
use std::fmt::Debug;
use std::io::Cursor;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;
use tracing::{debug, error, trace};
use yazi::{decompress, Format};

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
        state: ConnState,
    ) -> Result<Self, NetError> {
        let pak = match compressed {
            true => Self::read_compressed(reader, state).await,
            false => Self::read_uncompressed(reader, state).await,
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

    async fn read_uncompressed<R: AsyncRead + Unpin>(
        reader: &mut R,
        state: ConnState,
    ) -> Result<Self, NetError> {
        loop {
            let length = VarInt::read_async(reader).await?.0 as usize;

            if length < 1 {
                return Err(NetError::Packet(PacketError::MalformedPacket(Some(
                    length as u8,
                ))));
            }

            if length > 2097151 {
                let id = VarInt::read_async(reader).await?.0 as u8;
                return Err(NetError::Packet(PacketError::MalformedPacket(Some(id))));
            }

            let mut buf = {
                let mut buf = vec![0; length];
                reader.read_exact(&mut buf).await?;
                Cursor::new(buf)
            };

            let id = VarInt::read_async(&mut buf).await?;

            if (id.0 == lookup_packet!("play", "serverbound", "custom_payload")
                && state == ConnState::Play)
                || (id.0 == lookup_packet!("configuration", "serverbound", "custom_payload")
                    && state == ConnState::Configuration)
            {
                // Ignore Plugin Message and read the next one
                trace!("Ignored serverbound plugin message (0x14)");
                continue;
            }

            return Ok(Self {
                length,
                id: id.0 as u8,
                data: buf,
            });
        }
    }

    async fn read_compressed<R: AsyncRead + Unpin>(
        reader: &mut R,
        state: ConnState,
    ) -> Result<Self, NetError> {
        loop {
            let packet_length = VarInt::read_async(reader).await?.0;

            if packet_length < 1 {
                return Err(NetError::Packet(PacketError::MalformedPacket(Some(
                    packet_length as u8,
                ))));
            }

            let data_length = VarInt::read_async(reader).await?.0;

            if data_length < 0 {
                return Err(NetError::Packet(PacketError::MalformedPacket(Some(
                    data_length as u8,
                ))));
            }

            if packet_length > 2097151 || data_length > 8388608 {
                return Err(NetError::Packet(PacketError::MalformedPacket(Some(
                    packet_length.max(data_length) as u8,
                ))));
            }

            let remaining_len = packet_length as usize - VarInt::new(data_length).len();

            if data_length == 0 {
                // Not compressed, just read the inner data
                let mut buf = vec![0; remaining_len];
                reader.read_exact(&mut buf).await?;
                let mut cursor = Cursor::new(buf);

                let id = VarInt::read_async(&mut cursor).await?;

                if (id.0 == lookup_packet!("play", "serverbound", "custom_payload")
                    && state == ConnState::Play)
                    || (id.0 == lookup_packet!("configuration", "serverbound", "custom_payload")
                        && state == ConnState::Configuration)
                {
                    trace!("Ignored uncompressed serverbound plugin message (0x14)");
                    continue;
                }

                return Ok(Self {
                    length: packet_length as usize,
                    id: id.0 as u8,
                    data: cursor,
                });
            }

            // Enforce compression threshold
            let compression_threshold = get_global_config().network_compression_threshold;
            if data_length < compression_threshold {
                return Err(NetError::DecoderError(
                    NetDecodeError::CompressedPacketTooSmall(data_length as usize),
                ));
            }

            // Read compressed data
            let mut compressed_buf = vec![0; remaining_len];
            reader.read_exact(&mut compressed_buf).await?;

            let (decompressed_data, checksum) = decompress(&compressed_buf, Format::Zlib)
                .map_err(|_| NetError::DecompressionError)?;

            if get_global_config().verify_decompressed_packets {
                let Some(actual_checksum) = checksum else {
                    error!("Missing checksum on decompressed packet");
                    return Err(NetError::DecompressionError);
                };

                let expected = yazi::Adler32::from_buf(&decompressed_data).finish();
                if actual_checksum != expected {
                    error!(
                        "Checksum mismatch: expected {}, got {}",
                        expected, actual_checksum
                    );
                    return Err(NetError::DecompressionError);
                }
            }

            if decompressed_data.len() != data_length as usize {
                error!(
                    "Decompressed packet length mismatch: expected {}, got {}",
                    data_length,
                    decompressed_data.len()
                );
                return Err(NetError::DecompressionError);
            }

            let mut cursor = Cursor::new(decompressed_data);
            let id = VarInt::read_async(&mut cursor).await?;

            if id.0 == lookup_packet!("play", "serverbound", "custom_payload") {
                trace!("Ignored compressed serverbound plugin message (0x14)");
                continue;
            }

            return Ok(Self {
                length: packet_length as usize,
                id: id.0 as u8,
                data: cursor,
            });
        }
    }
}

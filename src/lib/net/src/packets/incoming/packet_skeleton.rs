use crate::errors::CompressionError::{
    ChecksumMismatch, CompressedPacketTooSmall, GenericDecompressionError, MissingChecksum,
};
use crate::errors::{NetError, PacketError};
use crate::ConnState;
use ferrumc_config::server_config::get_global_config;
use ferrumc_macros::lookup_packet;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::fmt::Debug;
use std::io::Cursor;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;
use tracing::{debug, error, trace};
use yazi::{decompress, Format};

/// Represents a minimal parsed network packet (frame) read from the client.
///
/// The `PacketSkeleton` only extracts:
/// - The total packet length.
/// - The packet ID (VarInt, downcast to u8 for efficiency).
/// - The remaining data payload as a readable cursor.
///
/// This allows deferred decoding of the packet's contents until
/// it is dispatched to the appropriate handler.
pub struct PacketSkeleton {
    /// Total length of the full packet (prefix + ID + payload).
    pub length: usize,
    /// Packet ID (VarInt, truncated to `u8`).
    pub id: u8,
    /// Cursor pointing to the remaining packet bytes for further decoding.
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
    /// Constructs a new `PacketSkeleton` by reading from the given async reader.
    ///
    /// Supports both compressed and uncompressed packet formats as defined
    /// by the Minecraft protocol framing rules.
    ///
    /// # Parameters
    /// - `reader`: The asynchronous byte stream to read from.
    /// - `compressed`: Whether to interpret the packet as compressed.
    /// - `state`: Current connection state (e.g., Handshake, Play, Configuration).
    ///
    /// # Errors
    /// - Returns `ConnectionDropped` if the socket is closed.
    /// - Returns `MalformedPacket` if framing data is invalid.
    /// - Returns `DecompressionError` if compressed payload integrity checks fail.
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
                // Optional logging of every packet if `FERRUMC_LOG_PACKETS` env var is set
                if option_env!("FERRUMC_LOG_PACKETS").is_some() {
                    trace!("Received packet: {:?}", p);
                }
                Ok(p)
            }
            Err(e) => {
                if !matches!(e, NetError::ConnectionDropped) {
                    // Expected when client disconnects mid-read
                    trace!("Error reading packet: {:?}", e);
                } else {
                    debug!("Connection dropped: {:?}", e);
                }
                Err(e)
            }
        }
    }

    /// Reads an **uncompressed** packet from the client.
    ///
    /// Minecraft's packet format:
    /// ```text
    /// VarInt(length) | VarInt(packet_id) | [payload bytes...]
    /// ```
    ///
    /// - Filters out `custom_payload` plugin messages (0x14) in Play/Configuration states
    ///   to ignore unused plugin channels.
    async fn read_uncompressed<R: AsyncRead + Unpin>(
        reader: &mut R,
        state: ConnState,
    ) -> Result<Self, NetError> {
        loop {
            // Read total packet length (must be >= 1 byte)
            let length = VarInt::read_async(reader).await?.0 as usize;

            if length < 1 {
                return Err(NetError::Packet(PacketError::MalformedPacket(Some(
                    length as u8,
                ))));
            }

            // Sanity check to avoid maliciously large frames
            if length > 2097151 {
                let id = VarInt::read_async(reader).await?.0 as u8;
                return Err(NetError::Packet(PacketError::MalformedPacket(Some(id))));
            }

            // Read full packet data
            let mut buf = {
                let mut buf = vec![0; length];
                reader.read_exact(&mut buf).await?;

                Cursor::new(buf)
            };

            // Extract packet ID
            let id = VarInt::read_async(&mut buf).await?;

            // Ignore plugin messages (unused channels)
            if (id.0 == lookup_packet!("play", "serverbound", "custom_payload")
                && state == ConnState::Play)
                || (id.0 == lookup_packet!("configuration", "serverbound", "custom_payload")
                    && state == ConnState::Configuration)
            {
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

    /// Reads a **compressed** packet from the client.
    ///
    /// Minecraft's packet format under compression:
    /// ```text
    /// VarInt(packet_length) |
    /// VarInt(data_length)   |
    /// [possibly compressed bytes...]
    /// ```
    /// - `data_length = 0`: payload is uncompressed.
    /// - `data_length > 0`: payload is zlib-compressed to fit into packet_length.
    ///
    /// Compression threshold is enforced based on server config to prevent abuse.
    ///
    /// - Plugin messages (0x14) are ignored here as well.
    async fn read_compressed<R: AsyncRead + Unpin>(
        reader: &mut R,
        state: ConnState,
    ) -> Result<Self, NetError> {
        loop {
            // Total length of this packet frame
            let packet_length = VarInt::read_async(reader).await?.0;

            if packet_length < 1 {
                return Err(NetError::Packet(PacketError::MalformedPacket(Some(
                    packet_length as u8,
                ))));
            }

            // Declared length of decompressed payload (0 = no compression)
            let data_length = VarInt::read_async(reader).await?.0;

            if data_length < 0 {
                return Err(NetError::Packet(PacketError::MalformedPacket(Some(
                    data_length as u8,
                ))));
            }

            // Sanity checks to avoid huge memory allocations
            if packet_length > 2097151 || data_length > 8388608 {
                return Err(NetError::Packet(PacketError::MalformedPacket(Some(
                    packet_length.max(data_length) as u8,
                ))));
            }

            // Remaining bytes to read = total minus size of data_length field
            let remaining_len = packet_length as usize - VarInt::new(data_length).len();

            // Case 1: Uncompressed packet (data_length == 0)
            if data_length == 0 {
                let mut buf = vec![0; remaining_len];
                reader.read_exact(&mut buf).await?;

                let mut cursor = Cursor::new(buf);

                let id = VarInt::read_async(&mut cursor).await?;

                // Ignore plugin messages
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

            // Case 2: Compressed packet
            // Verify compression threshold to prevent trivial small packets from being compressed
            let compression_threshold = get_global_config().network_compression_threshold;
            if data_length < compression_threshold {
                return Err(NetError::CompressionError(CompressedPacketTooSmall(
                    data_length as usize,
                )));
            }

            // Read compressed bytes
            let mut compressed_buf = vec![0; remaining_len];
            reader.read_exact(&mut compressed_buf).await?;

            // Attempt decompression (Zlib format)
            let (decompressed_data, checksum) =
                decompress(&compressed_buf, Format::Zlib).map_err(|err| {
                    let msg = format!("Decompression error: {err:?}");
                    NetError::CompressionError(GenericDecompressionError(msg))
                })?;

            // Verify checksum if server has verification enabled
            if get_global_config().verify_decompressed_packets {
                let Some(actual_checksum) = checksum else {
                    error!("Missing checksum on decompressed packet");
                    return Err(NetError::CompressionError(MissingChecksum));
                };

                let expected = yazi::Adler32::from_buf(&decompressed_data).finish();
                if actual_checksum != expected {
                    error!(
                        "Checksum mismatch: expected {}, got {}",
                        expected, actual_checksum
                    );
                    return Err(NetError::CompressionError(ChecksumMismatch {
                        expected,
                        received: actual_checksum,
                    }));
                }
            }

            // Verify declared decompressed length matches actual size
            if decompressed_data.len() != data_length as usize {
                let error_msg = format!(
                    "Decompressed packet length mismatch: expected {}, got {}",
                    data_length,
                    decompressed_data.len()
                );
                error!(error_msg);
                return Err(NetError::CompressionError(GenericDecompressionError(
                    error_msg,
                )));
            }

            // Extract packet ID
            let mut cursor = Cursor::new(decompressed_data);
            let id = VarInt::read_async(&mut cursor).await?;

            // Ignore plugin messages
            if (state == ConnState::Play
                && id.0 == lookup_packet!("play", "serverbound", "custom_payload"))
                || (state == ConnState::Configuration
                    && id.0 == lookup_packet!("configuration", "serverbound", "custom_payload"))
            {
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

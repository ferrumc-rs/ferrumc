use crate::errors::CompressionError::{
    ChecksumMismatch, CompressedPacketTooSmall, GenericDecompressionError, MissingChecksum,
};
use crate::errors::{NetError, PacketError};
use ferrumc_net::ConnState;
use ferrumc_net_codec::net_types::var_int::VarInt;
// --- FIX: Use generated constants ---
use crate::ids;
// ------------------------------------

use std::fmt::Debug;
use std::io::Cursor;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;
use tracing::{debug, error, trace};
use yazi::{Format, decompress};

/// Represents a minimal parsed network packet (frame) read from the client.
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
    /// Constructs a new `PacketSkeleton` by reading from the given async reader.
    ///
    /// # Parameters
    /// - `reader`: The asynchronous byte stream.
    /// - `compressed`: Whether compression is enabled.
    /// - `state`: Current connection state.
    /// - `threshold`: Compression threshold (bytes). Pass 0 if unknown, but critical for `compressed=true`.
    /// - `verify_checksum`: Whether to verify adler32 checksums (config).
    pub async fn new<R: AsyncRead + Unpin>(
        reader: &mut R,
        compressed: bool,
        state: ConnState,
        threshold: usize,      // <-- INJECTED
        verify_checksum: bool, // <-- INJECTED
    ) -> Result<Self, NetError> {
        let pak = match compressed {
            true => Self::read_compressed(reader, state, threshold, verify_checksum).await,
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
                    trace!("Error reading packet: {:?}", e);
                } else {
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

            let mut buf = vec![0; length];
            reader.read_exact(&mut buf).await?;
            let mut buf = Cursor::new(buf);

            let id = VarInt::read_async(&mut buf).await?;

            // --- FIX: Use Constants instead of Macro ---
            // Note: Adjust CONSTANT_NAME if your generator output differs (e.g. PLUGIN_MESSAGE)
            let is_play_custom =
                state == ConnState::Play && id.0 == ids::PLAY_SERVERBOUND_CUSTOM_PAYLOAD as i32;

            let is_config_custom = state == ConnState::Configuration
                && id.0 == ids::CONFIGURATION_SERVERBOUND_CUSTOM_PAYLOAD as i32;

            if is_play_custom || is_config_custom {
                trace!("Ignored serverbound plugin message (0x{:02X})", id.0);
                continue;
            }
            // -------------------------------------------

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
        threshold: usize,      // <-- INJECTED
        verify_checksum: bool, // <-- INJECTED
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

            // Case 1: Uncompressed packet (data_length == 0)
            if data_length == 0 {
                let mut buf = vec![0; remaining_len];
                reader.read_exact(&mut buf).await?;
                let mut cursor = Cursor::new(buf);
                let id = VarInt::read_async(&mut cursor).await?;

                // --- FIX: Constants ---
                let is_play_custom =
                    state == ConnState::Play && id.0 == ids::PLAY_SERVERBOUND_CUSTOM_PAYLOAD as i32;
                let is_config_custom = state == ConnState::Configuration
                    && id.0 == ids::CONFIGURATION_SERVERBOUND_CUSTOM_PAYLOAD as i32;

                if is_play_custom || is_config_custom {
                    trace!("Ignored uncompressed serverbound plugin message");
                    continue;
                }
                // ---------------------

                return Ok(Self {
                    length: packet_length as usize,
                    id: id.0 as u8,
                    data: cursor,
                });
            }

            // Case 2: Compressed packet

            // --- FIX: Use Injected Threshold ---
            if (data_length as usize) < threshold {
                return Err(NetError::CompressionError(CompressedPacketTooSmall(
                    data_length as usize,
                )));
            }
            // ----------------------------------

            let mut compressed_buf = vec![0; remaining_len];
            reader.read_exact(&mut compressed_buf).await?;

            let (decompressed_data, checksum) =
                decompress(&compressed_buf, Format::Zlib).map_err(|err| {
                    let msg = format!("Decompression error: {err:?}");
                    NetError::CompressionError(GenericDecompressionError(msg))
                })?;

            // --- FIX: Use Injected Verification Config ---
            if verify_checksum {
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
            // ---------------------------------------------

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

            let mut cursor = Cursor::new(decompressed_data);
            let id = VarInt::read_async(&mut cursor).await?;

            // --- FIX: Constants ---
            let is_play_custom =
                state == ConnState::Play && id.0 == ids::PLAY_SERVERBOUND_CUSTOM_PAYLOAD as i32;
            let is_config_custom = state == ConnState::Configuration
                && id.0 == ids::CONFIGURATION_SERVERBOUND_CUSTOM_PAYLOAD as i32;

            if is_play_custom || is_config_custom {
                trace!("Ignored compressed serverbound plugin message");
                continue;
            }
            // ---------------------

            return Ok(Self {
                length: packet_length as usize,
                id: id.0 as u8,
                data: cursor,
            });
        }
    }
}

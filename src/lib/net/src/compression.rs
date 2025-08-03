// Imports standard library IO traits
use std::io::{Cursor, Read};

// Imports the global compression threshold setting from server config
use ferrumc_config::server_config::get_global_config;

// Imports the NetEncode trait and options enum for encoding network packets
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
// Imports Minecraft-style VarInt support (used for frame sizes and IDs)
use ferrumc_net_codec::net_types::var_int::VarInt;

// Error type for networking operations
use crate::errors::NetError;

// External compression lib (Zlib, Gzip, etc.) used to compress packet payloads
use yazi::{compress, CompressionLevel, Format};

// For error logging
use tracing::error;

/// Compresses a network packet if compression is enabled and threshold is met.
///
/// This function handles Minecraft-style compression for outgoing packets:
/// - Adds a length prefix (VarInt) to the final packet.
/// - Compresses the payload (ID + BODY) if the uncompressed size exceeds the threshold.
/// - Supports fallback to uncompressed mode depending on configuration.
///
/// # Arguments
/// * `packet` - A reference to a struct that implements `NetEncode` (e.g., a login or play packet).
/// * `compress_packet` - Flag indicating whether compression should be applied.
/// * `net_encode_opts` - Options for how the packet should be initially encoded (e.g., with or without length).
///
/// # Returns
/// A `Vec<u8>` containing the final byte stream, suitable for direct transmission.
pub fn compress_packet(
    packet: &(impl NetEncode + Send),
    compress_packet: bool,
    net_encode_opts: &NetEncodeOpts,
) -> Result<Vec<u8>, NetError> {
    /// Helper function to encode the packet as a full frame, then split into `(VarInt ID, body bytes)`.
    ///
    /// This is used to prepare the payload for compression (ID + body),
    /// by first encoding it with a length prefix, then stripping that prefix.
    fn encode_id_and_body(pkt: &(impl NetEncode + Send)) -> Result<(VarInt, Vec<u8>), NetError> {
        // Full encode with outer length prefix
        let mut full = Vec::new();
        pkt.encode(&mut full, &NetEncodeOpts::WithLength)?;

        // Use a Cursor to step through the byte buffer
        let mut cur = Cursor::new(full);

        // Skip the outer length prefix (not needed for inner compression payload)
        let _outer_len = VarInt::read(&mut cur)?; // ignored

        // Extract the VarInt packet ID
        let id_vi = VarInt::read(&mut cur)?;

        // Read the remaining bytes (the packet body)
        let mut body = Vec::new();
        cur.read_to_end(&mut body)?;

        Ok((id_vi, body))
    }

    // Main compression logic
    let raw_bytes = if compress_packet {
        // Construct the canonical uncompressed frame: VarInt ID + body
        let (id_vi, body) = encode_id_and_body(packet)?;

        // Preallocate buffer for performance
        let mut uncompressed_frame = Vec::with_capacity(id_vi.len() + body.len());
        id_vi.encode(&mut uncompressed_frame, &NetEncodeOpts::None)?;
        uncompressed_frame.extend_from_slice(&body);

        // Compression threshold (in bytes), retrieved from global config
        let threshold = get_global_config().network_compression_threshold as usize;

        let mut inner = Vec::new();

        // If the frame size exceeds the threshold, compress it
        if uncompressed_frame.len() >= threshold {
            let compressed = compress(
                &uncompressed_frame,
                Format::Zlib,                // Minecraft uses Zlib format
                CompressionLevel::BestSpeed, // Fastest compression option
            )
            .map_err(|err| {
                error!("Failed to compress packet: {:?}", err);
                NetError::CompressionError(format!("Failed to compress packet: {:?}", err))
            })?;

            // Prepend the uncompressed size as a VarInt
            VarInt::new(uncompressed_frame.len() as i32)
                .encode(&mut inner, &NetEncodeOpts::None)?;
            inner.extend_from_slice(&compressed);
        } else {
            // Below threshold: use uncompressed frame with 0 prefix
            VarInt::new(0).encode(&mut inner, &NetEncodeOpts::None)?;
            inner.extend_from_slice(&uncompressed_frame);
        }

        // Final output = VarInt(total inner len) + inner
        let mut final_data = Vec::with_capacity(inner.len() + 5); // Extra space for prefix
        VarInt::new(inner.len() as i32).encode(&mut final_data, &NetEncodeOpts::None)?;
        final_data.extend_from_slice(&inner);
        final_data
    } else {
        // Fallback: just encode using provided options (e.g., WithLength or None)
        let mut buffer = Vec::new();
        packet.encode(&mut buffer, net_encode_opts)?;
        buffer
    };

    Ok(raw_bytes)
}

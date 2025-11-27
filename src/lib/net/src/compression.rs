use std::io::{Cursor, Read};

use crate::errors::CompressionError::GenericCompressionError;
use crate::errors::NetError;
use ferrumc_protocol::codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_protocol::codec::net_types::var_int::VarInt;

use tracing::error;
use yazi::{compress, CompressionLevel, Format};

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
    threshold: usize,
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
        let mut id_buf = Vec::new();
        id_vi.encode(&mut id_buf, &NetEncodeOpts::None)?;

        let mut uncompressed_frame = Vec::with_capacity(id_buf.len() + body.len());
        uncompressed_frame.extend_from_slice(&id_buf);
        uncompressed_frame.extend_from_slice(&body);

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
                NetError::CompressionError(GenericCompressionError(format!(
                    "Failed to compress packet: {:?}",
                    err
                )))
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

#[cfg(test)]
mod tests {
    use crate::compression::compress_packet;
    use crate::packets::incoming::packet_skeleton::PacketSkeleton;
    use crate::ConnState;
    use ferrumc_config::server_config::set_global_config;
    use ferrumc_config::ServerConfig;

    use crate::errors::NetError;
    use ferrumc_protocol::codec::encode::errors::NetEncodeError;
    use ferrumc_protocol::codec::encode::{NetEncode, NetEncodeOpts};
    use ferrumc_protocol::codec::net_types::var_int::VarInt;
    use std::io::Cursor;

    struct TestPacket {
        test_vi: VarInt,
        body: Vec<u8>,
    }

    impl NetEncode for TestPacket {
        fn encode<W: std::io::Write>(
            &self,
            writer: &mut W,
            _opts: &NetEncodeOpts,
        ) -> Result<(), NetEncodeError> {
            use std::io::Write;
            let mut buffer = Cursor::new(Vec::new());
            // Normally packet id is provided by the macro, but here we manually encode it
            VarInt(99).encode(&mut buffer, &NetEncodeOpts::None)?;
            self.test_vi.encode(&mut buffer, &NetEncodeOpts::None)?;
            buffer.write_all(&self.body)?;

            let inner = buffer.into_inner();
            // Write the length prefix
            VarInt::new(inner.len() as i32).encode(writer, &NetEncodeOpts::None)?;
            // Write the actual data
            writer.write_all(&inner)?;
            Ok(())
        }

        async fn encode_async<W: tokio::io::AsyncWrite + Unpin>(
            &self,
            writer: &mut W,
            _opts: &NetEncodeOpts,
        ) -> Result<(), NetEncodeError> {
            use tokio::io::AsyncWriteExt;
            // Normally packet id is provided by the macro, but here we manually encode it
            let mut buffer = Cursor::new(Vec::new());
            VarInt(99).encode(&mut buffer, &NetEncodeOpts::None)?;
            self.test_vi.encode(&mut buffer, &NetEncodeOpts::None)?;
            buffer.write_all(&self.body).await?;
            let inner = buffer.into_inner();
            // Write the length prefix
            VarInt::new(inner.len() as i32)
                .encode_async(writer, &NetEncodeOpts::None)
                .await?;
            // Write the actual data
            writer.write_all(&inner).await?;
            Ok(())
        }
    }

    #[test]
    fn test_compress_packet() {
        let packet = TestPacket {
            test_vi: VarInt::new(42),
            body: vec![255; 1000], // Large enough to trigger compression
        };
        let compressed = compress_packet(&packet, true, &NetEncodeOpts::WithLength, 512);
        assert!(
            compressed.is_ok(),
            "Compression failed: {:?}",
            compressed.err()
        );
        let compressed_data = compressed.unwrap();
        assert!(
            !compressed_data.is_empty(),
            "Compressed data should not be empty"
        );
    }

    #[tokio::test]
    async fn test_decompress() {
        let compressed = compress_packet(
            &TestPacket {
                test_vi: VarInt::new(42),
                body: vec![255; 1000], // Large enough to trigger compression
            },
            true,
            &NetEncodeOpts::WithLength,
            512,
        )
        .unwrap();

        let mut async_reader = Cursor::new(compressed);

        let skel = PacketSkeleton::new(&mut async_reader, true, ConnState::Play).await;
        assert!(
            skel.is_ok(),
            "Failed to read packet skeleton: {:?}",
            skel.err()
        );
        let skel = skel.unwrap();
        assert_eq!(skel.id, 99, "Packet ID mismatch");
    }

    #[test]
    fn test_compress_packet_below_threshold() {
        let packet = TestPacket {
            test_vi: VarInt::new(42),
            body: vec![255; 100], // Small enough to not trigger compression
        };

        let mut uncompressed_buf = Vec::new();
        packet
            .encode(&mut uncompressed_buf, &NetEncodeOpts::None)
            .unwrap();

        let compressed = compress_packet(
            &packet,
            true,
            &NetEncodeOpts::WithLength,
            512, // <--- The threshold
        );

        assert!(
            compressed.is_ok(),
            "Compression failed: {:?}",
            compressed.err()
        );
        let compressed_data = compressed.unwrap();
        assert!(
            !compressed_data.is_empty(),
            "Compressed data should not be empty"
        );
        // Check that the compressed data isn't smaller than the uncompressed
        assert!(
            compressed_data.len() >= uncompressed_buf.len(),
            "Input data should not be compressed smaller than original"
        );
    }

    #[tokio::test]
    async fn test_decompress_below_threshold() {
        let packet = TestPacket {
            test_vi: VarInt::new(42),
            body: vec![255; 100], // Small enough to not trigger compression
        };
        set_global_config(ServerConfig {
            network_compression_threshold: 512,
            ..Default::default()
        });
        let compressed = compress_packet(&packet, true, &NetEncodeOpts::WithLength, 512).unwrap();

        let mut async_reader = Cursor::new(compressed);

        let skel = PacketSkeleton::new(&mut async_reader, true, ConnState::Play).await;
        assert!(
            skel.is_ok(),
            "Failed to read packet skeleton: {:?}",
            skel.err()
        );
        let skel = skel.unwrap();
        assert_eq!(skel.id, 99, "Packet ID mismatch");
    }

    #[tokio::test]
    async fn test_compress_packet_without_compression() {
        let packet = TestPacket {
            test_vi: VarInt::new(42),
            body: vec![255; 1000], // Large enough to trigger compression
        };
        let compressed = compress_packet(&packet, false, &NetEncodeOpts::WithLength, 512);
        assert!(
            compressed.is_ok(),
            "Compression failed: {:?}",
            compressed.err()
        );
        let compressed_data = compressed.unwrap();
        assert!(
            !compressed_data.is_empty(),
            "Compressed data should not be empty"
        );
        // Check that the output matches the uncompressed data
        let mut expected_buf = Vec::new();
        packet
            .encode(&mut expected_buf, &NetEncodeOpts::WithLength)
            .unwrap();
        assert_eq!(compressed_data, expected_buf);
    }

    #[tokio::test]
    async fn test_decompress_without_compression() {
        let packet = TestPacket {
            test_vi: VarInt::new(42),
            body: vec![255; 1000], // Large enough to trigger compression
        };
        let compressed = compress_packet(&packet, false, &NetEncodeOpts::WithLength, 512).unwrap();

        let mut async_reader = Cursor::new(compressed);

        let skel = PacketSkeleton::new(&mut async_reader, true, ConnState::Play).await;
        assert!(
            skel.is_err(),
            "Expected error reading uncompressed packet skeleton, got: {:?}",
            skel
        );
        // The error should indicate that the packet is not compressed
        if !matches!(skel, Err(NetError::CompressionError(_))) {
            panic!("Expected a decompression error, got: {:?}", skel);
        }
    }

    #[tokio::test]
    async fn test_decompress_bad_data() {
        // Test with invalid data that doesn't match expected format
        let bad_data = vec![0x00, 0x01, 0x02, 0x03]; // Not a valid VarInt or packet structure
        let mut async_reader = Cursor::new(bad_data);

        let skel = PacketSkeleton::new(&mut async_reader, true, ConnState::Play).await;
        assert!(
            skel.is_err(),
            "Expected error reading bad packet data, got: {:?}",
            skel
        );
    }

    #[tokio::test]
    async fn test_decompress_empty_data() {
        // Test with empty data, which should fail to read a packet skeleton
        let empty_data = vec![];
        let mut async_reader = Cursor::new(empty_data);
        let skel = PacketSkeleton::new(&mut async_reader, true, ConnState::Play).await;
        assert!(
            skel.is_err(),
            "Expected error reading empty packet data, got: {:?}",
            skel
        );
    }
}

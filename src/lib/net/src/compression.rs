use std::io::{Cursor, Read};
use tracing::error;
use yazi::{compress, CompressionLevel, Format};
use ferrumc_config::server_config::get_global_config;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use crate::errors::NetError;

pub(crate) fn compress_packet(
    packet: &(impl NetEncode + Send),
    compress_packet: bool,
    net_encode_opts: NetEncodeOpts
) -> Result<Vec<u8>, NetError> {
    // Helper: encode full frame (outer length + id + body), then split into (id_varint, body)
    fn encode_id_and_body(pkt: &(impl NetEncode + Send)) -> Result<(VarInt, Vec<u8>), NetError> {
        // Authoritative path: whatever "normal" is for your packets on the wire.
        let mut full = Vec::new();
        // This MUST be the same mode your plain/uncompressed path uses.
        pkt.encode(&mut full, &NetEncodeOpts::WithLength)?;

        // Strip outer length
        let mut cur = Cursor::new(full);
        let _outer_len = VarInt::read(&mut cur)?; // discard

        // Read ID (keep the actual VarInt for length calculations if you need)
        let id_vi = VarInt::read(&mut cur)?;

        // Remaining = body
        let mut body = Vec::new();
        cur.read_to_end(&mut body)?;
        Ok((id_vi, body))
    }

    let raw_bytes = if compress_packet {
        // Build a canonical uncompressed frame (ID + BODY) from the authoritative encode
        let (id_vi, body) = encode_id_and_body(packet)?;

        let mut uncompressed_frame = Vec::with_capacity(id_vi.len() + body.len());
        id_vi.encode(&mut uncompressed_frame, &NetEncodeOpts::None)?;
        uncompressed_frame.extend_from_slice(&body);

        // Compare against threshold using the *uncompressed frame length* (ID+BODY)
        let threshold = get_global_config().network_compression_threshold as usize;

        // Inner payload = VarInt(uncompressed_len or 0) + (compressed or uncompressed_frame)
        let mut inner = Vec::new();
        if uncompressed_frame.len() >= threshold {
            // compress ID+BODY
            let compressed = compress(
                &uncompressed_frame,
                Format::Zlib,
                CompressionLevel::BestSpeed,
            )
                .map_err(|err| {
                    error!("Failed to compress packet: {:?}", err);
                    NetError::CompressionError(format!("Failed to compress packet: {:?}", err))
                })?;

            // write uncompressed_length (as VarInt), then compressed bytes
            VarInt::new(uncompressed_frame.len() as i32)
                .encode(&mut inner, &NetEncodeOpts::None)?;
            inner.extend_from_slice(&compressed);
        } else {
            // below threshold: uncompressed; write 0 then ID+BODY
            VarInt::new(0).encode(&mut inner, &NetEncodeOpts::None)?;
            inner.extend_from_slice(&uncompressed_frame);
        }

        // Outer = VarInt(total inner len) + inner
        let mut final_data = Vec::with_capacity(inner.len() + 5);
        VarInt::new(inner.len() as i32).encode(&mut final_data, &NetEncodeOpts::None)?;
        final_data.extend_from_slice(&inner);
        final_data
    } else {
        // No compression path: just do whatever caller asked (likely WithLength).
        let mut buffer = Vec::new();
        packet.encode(&mut buffer, &net_encode_opts)?;
        buffer
    };
    Ok(raw_bytes)
}
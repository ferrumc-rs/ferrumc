use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts, NetDecodeResult};
use ferrumc_net_codec::net_types::{var_int::VarInt};
use std::io::Read;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;

// According to packets.json, serverbound play chat packet is ID 7.
// The wiki.vg page for 1.21 protocol also lists "Serverbound Chat Message" as 0x07 in Play state.
// https://wiki.vg/Protocol#Chat_Message_.28serverbound.29
// Fields: Message (String), Timestamp (Long), Salt (Long), Signature (Optional ByteArray), Message Count (VarInt), Acknowledged (BitSet)

// For a basic MVP, we'll just parse the message string.
// More fields can be added later as needed.

#[derive(NetDecode, Debug)]
#[packet(packet_id = "chat", state = "play")]
#[allow(unused)]
pub struct ChatMessage {
    pub message: String,
    timestamp: i64,
    /// The salt used to verify the signature hash.
    salt: i64,
    signature: ChatSignature,
    message_count: VarInt,
    acknowledged: ChatAckBitSet,
    checksum: u8,
}

#[derive(Debug)]
pub struct ChatSignature {
    ///The signature used to verify the chat message's authentication. When present, always 256 bytes and not length-prefixed.
    pub signature: Option<[u8; 256]>,
}

impl NetDecode for ChatSignature {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> NetDecodeResult<Self> {
        // Whether the next field is present.
        let has_signature = <bool>::decode(reader, opts)?;
        if !has_signature {
            return Ok(ChatSignature { signature: None });
        }

        let mut signature = [0; 256];
        reader.read_exact(&mut signature)?;

        Ok(ChatSignature {
            signature: Some(signature),
        })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> NetDecodeResult<Self> {
        let has_signature = <bool>::decode_async(reader, opts).await?;
        if !has_signature {
            return Ok(ChatSignature { signature: None });
        }

        let mut signature = [0; 256];
        reader.read_exact(&mut signature).await?;

        Ok(ChatSignature {
            signature: Some(signature),
        })
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct ChatAckBitSet {
    /// Fixed BitSet (20).
    //
    // Bit sets of type Fixed BitSet (n) have a fixed length of n bits, encoded as ceil(n / 8) bytes. Note that this is different from BitSet, which uses longs.
    // Field Name 	Field Type 	Meaning
    // Data 	Byte Array (n) 	A packed representation of the bit set as created by BitSet.toByteArray, padded with zeroes at the end to fit the specified length.
    //
    // The ith bit is set when (Data[i / 8] & (1 << (i % 8))) != 0, where i starts at 0. This encoding is not equivalent to the long array in BitSet.
    bits: Vec<u64>,
}

impl NetDecode for ChatAckBitSet {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> NetDecodeResult<Self> {
        let length = VarInt::decode(reader, opts)?.0 as usize;
        if length % 8 != 0 {
            return Err(NetDecodeError::InvalidLength {
                expected: length,
                actual: 0,
                field: "ChatAckBitSet".to_string(),
            });
        }

        let mut data = vec![0u8; length / 8];
        reader.read_exact(&mut data)?;

        let mut bits = Vec::new();
        for chunk in data.chunks(8) {
            let mut value = 0u64;
            for (i, byte) in chunk.iter().enumerate() {
                value |= (*byte as u64) << (i * 8);
            }
            bits.push(value);
        }

        Ok(ChatAckBitSet { bits })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> NetDecodeResult<Self> {
        let length = VarInt::decode_async(reader, opts).await?.0 as usize;
        if length % 8 != 0 {
            return Err(NetDecodeError::InvalidLength {
                expected: length,
                actual: 0,
                field: "ChatAckBitSet".to_string(),
            });
        }

        let mut data = vec![0u8; length / 8];
        reader.read_exact(&mut data).await?;

        let mut bits = Vec::new();
        for chunk in data.chunks(8) {
            let mut value = 0u64;
            for (i, byte) in chunk.iter().enumerate() {
                value |= (*byte as u64) << (i * 8);
            }
            bits.push(value);
        }

        Ok(ChatAckBitSet { bits })
    }
}

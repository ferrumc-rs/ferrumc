use crate::{errors::NetError, NetResult};
use ferrumc_config::statics::get_global_config;
use ferrumc_net_codec::{decode::errors::NetDecodeError, net_types::var_int::VarInt};
use std::io::Cursor;
use std::{fmt::Debug, io::Read};

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
    pub fn new<R: Read + Unpin>(reader: &mut R, compressed: bool) -> NetResult<Self> {
        match compressed {
            true => Self::read_compressed(reader),
            false => Self::read_uncompressed(reader),
        }
    }

    #[inline(always)]
    fn read_uncompressed<R: Read + Unpin>(reader: &mut R) -> NetResult<Self> {
        let length = VarInt::read(reader)?.val as usize;
        let mut buf = {
            let mut buf = vec![0; length];
            reader.read_exact(&mut buf)?;

            Cursor::new(buf)
        };

        let id = VarInt::read(&mut buf)?;

        Ok(Self {
            length,
            id: id.val as u8,
            data: buf,
        })
    }

    #[inline(always)]
    fn read_compressed<R: Read + Unpin>(reader: &mut R) -> NetResult<Self> {
        let packet_length = VarInt::read(reader)?.val as usize;
        let data_length = VarInt::read(reader)?.val as usize;

        // Uncompressed packet when data length is 0
        if data_length == 0 {
            let mut buf = {
                let mut buf = vec![0; packet_length];
                reader.read_exact(&mut buf)?;

                Cursor::new(buf)
            };

            let id = VarInt::read(&mut buf)?;

            return Ok(Self {
                length: packet_length,
                id: id.val as u8,
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
            return NetResult::Err(NetError::DecoderError(
                NetDecodeError::CompressedPacketTooSmall(data_length),
            ));
        }

        // Here, guaranteed that data_length >= compression_threshold
        let mut buf = {
            let mut buf = vec![];
            reader.read_to_end(&mut buf)?;

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

        let id = VarInt::read(&mut buf)?;

        Ok(Self {
            length: packet_length,
            id: id.val as u8,
            data: buf,
        })
    }
}

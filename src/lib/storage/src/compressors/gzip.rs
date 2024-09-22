use std::io::{Cursor, Read};
use flate2::Compression;
use crate::Compressor;
use flate2::read::{GzDecoder, GzEncoder};
use tracing::error;
use ferrumc_macros::profile;

#[derive(Debug)]
pub struct GzipCompressor {
    level: u32,
}

impl Compressor for GzipCompressor {
    fn new(level: i32) -> Self {
        Self {
            level: level as u32,
        }
    }

    #[profile("compress/gzip")]
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        let mut encoder = GzEncoder::new(data, Compression::new(self.level));
        let mut compressed = Vec::new();
        if let Err(e) = encoder.read_to_end(&mut compressed) {
            error!("Error compressing data: {}", e);
            Vec::new()
        } else {
            compressed
        }
    }

    #[profile("decompress/gzip")]
    fn decompress(&self, data: &[u8]) -> Vec<u8> {
        let mut decoder = GzDecoder::new(Cursor::new(data));
        let mut decompressed = Vec::new();
        if let Err(e) = decoder.read_to_end(&mut decompressed) {
            error!("Error decompressing data: {}", e);
            Vec::new()
        } else {
            decompressed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Compressor;
    

    #[test]
    fn test_compress_decompress() {
        let compressor = GzipCompressor::new(6);
        let data = std::fs::read("../../../.etc/codec.nbt").unwrap();
        let compressed = compressor.compress(data.as_slice());
        let decompressed = compressor.decompress(&compressed);
        assert_eq!(data, decompressed.as_slice());
    }
    
    #[test]
    fn test_positive_compression_ratio() {
        let compressor = GzipCompressor::new(6);
        let data = std::fs::read("../../../.etc/codec.nbt").unwrap();
        let compressed = compressor.compress(data.as_slice());
        assert!(data.len() > compressed.len());
    }
}

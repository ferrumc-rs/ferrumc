use std::io::{Cursor, Read};
use flate2::Compression;
use crate::Compressor;
use flate2::read::{GzDecoder, GzEncoder};
use tracing::error;
use ferrumc_macros::profile;
use crate::errors::StorageError;

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
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, StorageError> {
        let mut encoder = GzEncoder::new(data, Compression::new(self.level));
        let mut compressed = Vec::new();
        encoder.read_to_end(&mut compressed).map_err(|e| {
            error!("Error compressing data: {}", e);
            StorageError::CompressionError(e.to_string())
        })?;
        Ok(compressed)
    }

    #[profile("decompress/gzip")]
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, StorageError> {
        let mut decoder = GzDecoder::new(Cursor::new(data));
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).map_err(|e| {
            error!("Error decompressing data: {}", e);
            StorageError::DecompressionError(e.to_string())
        })?;
        Ok(decompressed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Compressor;
    use ferrumc_utils::root;
    

    #[test]
    fn test_compress_decompress() {
        let compressor = GzipCompressor::new(6);
        let data = std::fs::read(root!(".etc/codec.nbt")).unwrap();
        let compressed = compressor.compress(data.as_slice()).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        assert_eq!(data, decompressed.as_slice());
    }
    
    #[test]
    fn test_positive_compression_ratio() {
        let compressor = GzipCompressor::new(6);
        let data = std::fs::read(root!(".etc/codec.nbt")).unwrap();
        let compressed = compressor.compress(data.as_slice()).unwrap();
        assert!(data.len() > compressed.len());
    }
}

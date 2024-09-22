use std::io::Read;
use crate::Compressor;
use crate::errors::StorageError;

pub struct BrotliCompressor {
    level: i32,
}

impl Compressor for BrotliCompressor {
    fn new(level: i32) -> Self {
        Self {
            level,
        }
    }

    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, StorageError> {
        let mut compressor = brotli::CompressorReader::new(data, 4096, self.level as u32, 22);
        let mut compressed = Vec::new();
        compressor.read_to_end(&mut compressed).map_err(|e| {
            StorageError::CompressionError(e.to_string())
        })?;
        Ok(compressed)
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, StorageError> {
        let mut decompressor = brotli::Decompressor::new(data, 4096);
        let mut decompressed = Vec::new();
        decompressor.read_to_end(&mut decompressed).map_err(|e| {
            StorageError::DecompressionError(e.to_string())
        })?;
        Ok(decompressed)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Compressor;
    use ferrumc_utils::root;

    #[test]
    fn test_compress_decompress() {
        let compressor = BrotliCompressor::new(6);
        let data = std::fs::read(root!(".etc/codec.nbt")).unwrap();
        let compressed = compressor.compress(data.as_slice()).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        assert_eq!(data, decompressed.as_slice());
    }

    #[test]
    fn test_positive_compression_ratio() {
        let compressor = BrotliCompressor::new(6);
        let data = std::fs::read(root!(".etc/codec.nbt")).unwrap();
        let compressed = compressor.compress(data.as_slice()).unwrap();
        assert!(data.len() > compressed.len());
    }
}
use crate::errors::StorageError;
use crate::Compressor;
use std::io::Read;

pub struct ZlibCompressor {
    level: u32,
}

impl Compressor for ZlibCompressor {
    fn new(level: i32) -> Self {
        Self {
            level: level as u32,
        }
    }

    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, StorageError> {
        let mut compressor =
            flate2::read::ZlibEncoder::new(data, flate2::Compression::new(self.level));
        let mut compressed = Vec::new();
        compressor
            .read_to_end(&mut compressed)
            .map_err(|e| StorageError::CompressionError(e.to_string()))?;
        Ok(compressed)
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, StorageError> {
        let mut decompressor = flate2::read::ZlibDecoder::new(data);
        let mut decompressed = Vec::new();
        decompressor
            .read_to_end(&mut decompressed)
            .map_err(|e| StorageError::DecompressionError(e.to_string()))?;
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
        let compressor = ZlibCompressor::new(6);
        let data = std::fs::read(root!(".etc/codec.nbt")).unwrap();
        let compressed = compressor.compress(data.as_slice()).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        assert_eq!(data, decompressed.as_slice());
    }

    #[test]
    fn test_positive_compression_ratio() {
        let compressor = ZlibCompressor::new(6);
        let data = std::fs::read(root!(".etc/codec.nbt")).unwrap();
        let compressed = compressor.compress(data.as_slice()).unwrap();
        assert!(data.len() > compressed.len());
    }
}

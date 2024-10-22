use crate::errors::StorageError;
use crate::Compressor;
use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct ZstdCompressor {
    level: i32,
}


    pub(crate) fn compress_zstd(level: u32, data: &[u8]) -> Result<Vec<u8>, StorageError> {
        zstd::encode_all(data, level as i32)
            .map_err(|e| StorageError::CompressionError(e.to_string()))
    }

    pub(crate) fn decompress_zstd(data: &[u8]) -> Result<Vec<u8>, StorageError> {
        let mut decoder = zstd::Decoder::new(Cursor::new(data))
            .map_err(|e| StorageError::DecompressionError(e.to_string()))?;
        let mut decompressed = Vec::new();
        decoder
            .read_to_end(&mut decompressed)
            .map_err(|e| StorageError::DecompressionError(e.to_string()))?;
        Ok(decompressed)
    }


#[cfg(test)]
mod tests {

    use ferrumc_utils::root;
    use crate::compressors::{Compressor, CompressorType};

    #[test]
    fn test_compress_decompress() {
        let compressor = Compressor::create(CompressorType::Zstd, 6);
        let data = std::fs::read(root!(".etc/codec.nbt")).unwrap();
        let compressed = compressor.compress(data.as_slice()).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        assert_eq!(data, decompressed.as_slice());
    }

    #[test]
    fn test_positive_compression_ratio() {
        let compressor = Compressor::create(CompressorType::Zstd, 6);
        let data = std::fs::read(root!(".etc/codec.nbt")).unwrap();
        let compressed = compressor.compress(data.as_slice()).unwrap();
        assert!(data.len() > compressed.len());
    }
}

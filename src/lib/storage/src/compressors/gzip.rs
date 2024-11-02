use crate::errors::StorageError;
use flate2::read::{GzDecoder, GzEncoder};
use flate2::Compression;
use std::io::{Cursor, Read};
use tracing::error;

pub(crate) fn compress_gzip(level: u32, data: &[u8]) -> Result<Vec<u8>, StorageError> {
    let mut encoder = GzEncoder::new(data, Compression::new(level));
    let mut compressed = Vec::new();
    encoder.read_to_end(&mut compressed).map_err(|e| {
        error!("Error compressing data: {}", e);
        StorageError::CompressionError(e.to_string())
    })?;
    Ok(compressed)
}
pub(crate) fn decompress_gzip(data: &[u8]) -> Result<Vec<u8>, StorageError> {
    let mut decoder = GzDecoder::new(Cursor::new(data));
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).map_err(|e| {
        error!("Error decompressing data: {}", e);
        StorageError::DecompressionError(e.to_string())
    })?;
    Ok(decompressed)
}

#[cfg(test)]
mod tests {
    use crate::compressors::{Compressor, CompressorType};
    use ferrumc_utils::root;

    #[test]
    fn test_compress_decompress() {
        let compressor = Compressor::create(CompressorType::Gzip, 6);
        let data = std::fs::read(root!(".etc/codec.nbt")).unwrap();
        let compressed = compressor.compress(data.as_slice()).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        assert_eq!(data, decompressed.as_slice());
    }

    #[test]
    fn test_positive_compression_ratio() {
        let compressor = Compressor::create(CompressorType::Gzip, 6);
        let data = std::fs::read(root!(".etc/codec.nbt")).unwrap();
        let compressed = compressor.compress(data.as_slice()).unwrap();
        assert!(data.len() > compressed.len());
    }

    #[test]
    fn test_compress_decompress_gzip() {
        let data = std::fs::read(root!(".etc/codec.nbt")).unwrap();
        let compressed = super::compress_gzip(6, data.as_slice()).unwrap();
        let decompressed = super::decompress_gzip(&compressed).unwrap();
        assert_eq!(data, decompressed.as_slice());
    }
}

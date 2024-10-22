use crate::errors::StorageError;
use std::io::Read;

pub(crate) fn compress_zlib(level: u32, data: &[u8]) -> Result<Vec<u8>, StorageError> {
    let mut compressor = flate2::read::ZlibEncoder::new(data, flate2::Compression::new(level));
    let mut compressed = Vec::new();
    compressor
        .read_to_end(&mut compressed)
        .map_err(|e| StorageError::CompressionError(e.to_string()))?;
    Ok(compressed)
}

pub(crate) fn decompress_zlib(data: &[u8]) -> Result<Vec<u8>, StorageError> {
    let mut decompressor = flate2::read::ZlibDecoder::new(data);
    let mut decompressed = Vec::new();
    decompressor
        .read_to_end(&mut decompressed)
        .map_err(|e| StorageError::DecompressionError(e.to_string()))?;
    Ok(decompressed)
}

#[cfg(test)]
mod test {
    use crate::compressors::{Compressor, CompressorType};
    use ferrumc_utils::root;

    #[test]
    fn test_compress_decompress() {
        let compressor = Compressor::create(CompressorType::Zlib, 6);
        let data = std::fs::read(root!(".etc/codec.nbt")).unwrap();
        let compressed = compressor.compress(data.as_slice()).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        assert_eq!(data, decompressed.as_slice());
    }

    #[test]
    fn test_positive_compression_ratio() {
        let compressor = Compressor::create(CompressorType::Zlib, 6);
        let data = std::fs::read(root!(".etc/codec.nbt")).unwrap();
        let compressed = compressor.compress(data.as_slice()).unwrap();
        assert!(data.len() > compressed.len());
    }
}

use crate::compressors::brotli::{compress_brotli, decompress_brotli};
use crate::compressors::deflate::{compress_deflate, decompress_deflate};
use crate::compressors::gzip::{compress_gzip, decompress_gzip};
use crate::compressors::zlib::{compress_zlib, decompress_zlib};
use crate::compressors::zstd::{compress_zstd, decompress_zstd};
use crate::errors::StorageError;

pub mod brotli;
pub mod deflate;
pub mod gzip;
pub mod zlib;
pub mod zstd;

#[derive(Clone, Copy)]
pub enum CompressorType {
    Gzip,
    Zstd,
    Brotli,
    Deflate,
    Zlib,
}

#[derive(Clone, Copy)]
pub struct Compressor {
    pub algorithm: CompressorType,
    pub level: u32,
}

impl Compressor {
    pub fn create(algorithm: CompressorType, level: u32) -> Self {
        Self { algorithm, level }
    }

    pub fn compress(&self, data: &[u8]) -> Result<Vec<u8>, StorageError> {
        match self.algorithm {
            CompressorType::Gzip => compress_gzip(self.level, data),
            CompressorType::Zstd => compress_zstd(self.level, data),
            CompressorType::Brotli => compress_brotli(self.level, data),
            CompressorType::Deflate => compress_deflate(self.level, data),
            CompressorType::Zlib => compress_zlib(self.level, data),
        }
    }

    pub fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, StorageError> {
        match self.algorithm {
            CompressorType::Gzip => decompress_gzip(data),
            CompressorType::Zstd => decompress_zstd(data),
            CompressorType::Brotli => decompress_brotli(data),
            CompressorType::Deflate => decompress_deflate(data),
            CompressorType::Zlib => decompress_zlib(data),
        }
    }
}

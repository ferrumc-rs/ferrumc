use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum StorageError {
    #[error("Error initializing database: {0}")]
    DatabaseInitError(String),
    #[error("Compression error: {0}")]
    CompressionError(String),
    #[error("Decompression error: {0}")]
    DecompressionError(String),
}

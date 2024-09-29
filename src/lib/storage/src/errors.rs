use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum StorageError {
    #[error("Error initializing database: {0}")]
    DatabaseInitError(String),
    #[error("Compression error: {0}")]
    CompressionError(String),
    #[error("Decompression error: {0}")]
    DecompressionError(String),
    #[error("Invalid path")]
    InvalidPath,
    #[error("Failed to write to database: {0}")]
    WriteError(String),
    #[error("Failed to read from database: {0}")]
    ReadError(String),
    #[error("Key not found: {0:X}")]
    KeyNotFound(u64),
    #[error("Key already exists: {0:X}")]
    KeyExists(u64),
    #[error("Failed to delete key: {0}")]
    DeleteError(String),
    #[error("Failed to update key: {0}")]
    UpdateError(String),
    #[error("Failed to open table: {0}")]
    TableError(String),
    #[error("Failed to commit transaction: {0}")]
    CommitError(String),
    #[error("Failed to flush database: {0}")]
    FlushError(String),
    #[error("Failed to close database: {0}")]
    CloseError(String),
}

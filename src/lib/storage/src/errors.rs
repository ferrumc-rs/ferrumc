use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum StorageError {
    #[error("Error initializing database: {0}")]
    DatabaseInitError(String),
}

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum StorageError {
    #[error("Something failed lol")]
    SomeError,
}
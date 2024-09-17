use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum NetEncryptionError {
    #[error("Something failed lol")]
    SomeError,
}
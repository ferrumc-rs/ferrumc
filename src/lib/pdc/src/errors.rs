use thiserror::Error;

#[derive(Debug, Error)]
pub enum PersistentDataError {
    #[error("Key was not found...")]
    KeyNotFound,

    #[error("Type is mismatched...")]
    TypeMismatch { expected: &'static str },

    #[error("Something went wrong with deserialization...")]
    DeserializationError,

    #[error("Failed to open PDC Database...")]
    FailedToOpenDatabase,

    #[error("Failed to read database")]
    FailedToReadDatabase,

    #[error("Unable to load file from {0}...")]
    UnableToLoadFile(String),
}

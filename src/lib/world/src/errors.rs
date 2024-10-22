use crate::errors::WorldError::{GenericIOError, PermissionError};
use std::io::ErrorKind;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum WorldError {
    #[error("Invalid World Path: {0}")]
    InvalidWorldPath(String),
    #[error("Invalid Backend: {0}")]
    InvalidBackend(String),
    #[error("Invalid Compressor: {0}")]
    InvalidCompressor(String),
    #[error("Invalid Cache Size: {0}")]
    InvalidCacheSize(String),
    #[error("Invalid Import Path: {0}")]
    InvalidImportPath(String),
    #[error("Unable to obtain permission to access file/folder: {0}")]
    PermissionError(String),
    #[error("Some kind of IO error occurred: {0}")]
    GenericIOError(String),
}

impl From<std::io::Error> for WorldError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            ErrorKind::PermissionDenied => PermissionError(err.to_string()),
            ErrorKind::ReadOnlyFilesystem => PermissionError(err.to_string()),
            _ => GenericIOError(err.to_string()),
        }
    }
}

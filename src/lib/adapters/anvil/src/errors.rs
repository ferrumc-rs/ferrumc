use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnvilError {
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),
    #[error("Invalid tables: {0}")]
    InvalidTables(PathBuf),
    #[error("Unable to read file {0}: {1}")]
    UnableToReadFile(PathBuf, std::io::Error),
    #[error("Unable to map file {0}: {1}")]
    UnableToMapFile(PathBuf, std::io::Error),
    #[error("Invalid offset or size")]
    InvalidOffsetOrSize,
}

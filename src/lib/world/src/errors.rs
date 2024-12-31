use crate::errors::WorldError::{GenericIOError, PermissionError};
use crate::vanilla_chunk_format::BlockData;
use errors::AnvilError;
use ferrumc_anvil::errors;
use ferrumc_general_purpose::data_packing::errors::DataPackingError;
use ferrumc_storage::errors::StorageError;
use std::io::ErrorKind;
use thiserror::Error;

#[derive(Debug, Error)]
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
    #[error("No region files")]
    NoRegionFiles,
    #[error("Unable to obtain permission to access file/folder: {0}")]
    PermissionError(String),
    #[error("Some kind of IO error occurred: {0}")]
    GenericIOError(String),
    #[error("A database error occurred from the world crate: {0}")]
    DatabaseError(StorageError),
    #[error("There was an error with bitcode's decoding: {0}")]
    BitcodeDecodeError(String),
    #[error("There was an error with bitcode's encoding: {0}")]
    BitcodeEncodeError(String),
    #[error("Chunk not found")]
    ChunkNotFound,
    #[error("Anvil Decode Error: {0}")]
    AnvilDecodeError(AnvilError),
    #[error("Missing block mapping: {0}")]
    MissingBlockMapping(BlockData),
    #[error("Invalid memory map size: {0}")]
    InvalidMapSize(u64),
    #[error("Task Join Error: {0}")]
    TaskJoinError(String),
    #[error("Section out of bounds: {0}")]
    SectionOutOfBounds(i32),
    #[error("Invalid block state data")]
    InvalidBlockStateData(String),
}

// implemente AcquireError for WorldError
use tokio::sync::AcquireError;
impl From<AcquireError> for WorldError {
    fn from(err: AcquireError) -> Self {
        WorldError::TaskJoinError(err.to_string())
    }
}

impl From<std::io::Error> for WorldError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            ErrorKind::PermissionDenied => PermissionError(err.to_string()),
            // ErrorKind::ReadOnlyFilesystem => PermissionError(err.to_string()),
            _ => GenericIOError(err.to_string()),
        }
    }
}

impl From<StorageError> for WorldError {
    fn from(err: StorageError) -> Self {
        WorldError::DatabaseError(err)
    }
}

impl From<AnvilError> for WorldError {
    fn from(err: errors::AnvilError) -> Self {
        WorldError::AnvilDecodeError(err)
    }
}

impl From<DataPackingError> for WorldError {
    fn from(e: DataPackingError) -> Self {
        WorldError::InvalidBlockStateData(e.to_string())
    }
}

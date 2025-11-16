use crate::block_state_id::BlockStateId;
use crate::errors::WorldError::{CompressionError, GenericIOError, PermissionError};
use errors::AnvilError;
use ferrumc_anvil::errors;
use ferrumc_general_purpose::data_packing::errors::DataPackingError;
use ferrumc_storage::errors::StorageError;
use std::io::ErrorKind;
use thiserror::Error;
use yazi::Error;

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
    MissingBlockMapping(BlockStateId),
    #[error("Invalid memory map size: {0}")]
    InvalidMapSize(u64),
    #[error("Task Join Error: {0}")]
    TaskJoinError(String),
    #[error("Section out of bounds: {0}")]
    SectionOutOfBounds(i32),
    #[error("Invalid block state data")]
    InvalidBlockStateData(String),
    #[error("Invalid block: {0}")]
    InvalidBlock(BlockStateId),
    #[error("Invalid batching operation: {0}")]
    InvalidBatchingOperation(String),
    #[error("Invalid block state ID: {0}")]
    InvalidBlockStateId(u32),
    #[error("World generation error: {0}")]
    WorldGenerationError(String),
    #[error("Compression error: {0}")]
    CompressionError(String),
    #[error("Decompression error: {0}")]
    DecompressionError(String),
    #[error("Corrupted chunk data: got checksum {0}, expected checksum {1}")]
    CorruptedChunkData(u32, u32),
    #[error("NBT data error: {0}")]
    NBTError(#[from] ferrumc_nbt::errors::NBTError),
}

impl From<std::io::Error> for WorldError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            ErrorKind::PermissionDenied => PermissionError(err.to_string()),
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

impl From<yazi::Error> for WorldError {
    fn from(e: yazi::Error) -> Self {
        match e {
            Error::Underflow => CompressionError("Underflow error during compression".to_string()),
            Error::InvalidBitstream => {
                CompressionError("Invalid bitstream error during compression".to_string())
            }
            Error::Overflow => CompressionError("Overflow error during compression".to_string()),
            Error::Finished => CompressionError("Finished error during compression".to_string()),
            Error::Io(io_err) => GenericIOError(io_err.to_string()),
        }
    }
}

use ferrumc_storage::errors::StorageError;
use thiserror::Error;
use yazi::Error;

#[derive(Debug, Error)]
pub enum PlayerDataError {
    #[error("Compression error: {0}")]
    CompressionError(String),
    #[error("A database error occurred from the playerstate crate: {0}")]
    DatabaseError(StorageError),
    #[error("Some kind of IO error occurred: {0}")]
    GenericIOError(String),
}

impl From<StorageError> for PlayerDataError {
    fn from(err: StorageError) -> Self {
        PlayerDataError::DatabaseError(err)
    }
}

impl From<yazi::Error> for PlayerDataError {
    fn from(e: yazi::Error) -> Self {
        match e {
            Error::Underflow => {
                PlayerDataError::CompressionError("Underflow error during compression".to_string())
            }
            Error::InvalidBitstream => PlayerDataError::CompressionError(
                "Invalid bitstream error during compression".to_string(),
            ),
            Error::Overflow => {
                PlayerDataError::CompressionError("Overflow error during compression".to_string())
            }
            Error::Finished => {
                PlayerDataError::CompressionError("Finished error during compression".to_string())
            }
            Error::Io(io_err) => PlayerDataError::GenericIOError(io_err.to_string()),
        }
    }
}

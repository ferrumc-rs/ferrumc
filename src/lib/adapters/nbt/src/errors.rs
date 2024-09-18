use thiserror::Error;

#[derive(Error, Debug)]
pub enum NBTError {
    #[error("Tried to read past the end of the NBT data")]
    ReachedEOF,
    #[error("Invalid UTF-8 string")]
    InvalidUTF8(#[from] std::str::Utf8Error),
    #[error("Invalid NBT tag ID: {0}")]
    InvalidRootCompound(u8),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unexpected end of data (EOF)")]
    UnexpectedEndOfData,
    #[error("The NBT data is invalid.")]
    InvalidNBTData,
    #[error("Try from slice error: {0}")]
    TryFromSlice(#[from] std::array::TryFromSliceError),
    #[error("The NBT data is compressed.")]
    CompressedData,
}

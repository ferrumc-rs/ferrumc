use thiserror::Error;

#[derive(Error, Debug)]
pub enum NBTError {
    #[error("Tried to read past the end of the NBT data")]
    ReachedEOF,
    #[error("Invalid UTF-8 string")]
    InvalidUTF8(#[from] std::str::Utf8Error),
}

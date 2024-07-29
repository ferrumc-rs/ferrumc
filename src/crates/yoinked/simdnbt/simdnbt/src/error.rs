use thiserror::Error;

use crate::common::MAX_DEPTH;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Invalid root type {0}")]
    InvalidRootType(u8),
    #[error("Unknown tag id {0}")]
    UnknownTagId(u8),
    #[error("Unexpected end of data")]
    UnexpectedEof,
    #[error("Tried to read NBT tag with too high complexity, depth > {MAX_DEPTH}")]
    MaxDepthExceeded,
}

// these two structs exist to optimize errors, since Error is an entire 2 bytes
// which are often unnecessary
pub(crate) struct UnexpectedEofError;
impl From<UnexpectedEofError> for Error {
    fn from(_: UnexpectedEofError) -> Self {
        Error::UnexpectedEof
    }
}
pub struct NonRootError {
    // 0 = unexpected eof
    // 1 = max depth exceeded
    // anything else = unknown tag id, the id is value-1
    value: u8,
}
impl From<NonRootError> for Error {
    #[inline]
    fn from(e: NonRootError) -> Self {
        match e.value {
            0 => Error::UnexpectedEof,
            1 => Error::MaxDepthExceeded,
            _ => Error::UnknownTagId(e.value.wrapping_add(1)),
        }
    }
}
impl NonRootError {
    #[inline]
    pub fn unexpected_eof() -> Self {
        NonRootError { value: 0 }
    }
    #[inline]
    pub fn max_depth_exceeded() -> Self {
        NonRootError { value: 1 }
    }
    #[inline]
    pub fn unknown_tag_id(id: u8) -> Self {
        // the value can't be 1 or 2 (because those are always valid tag ids),
        // so we take advantage of that in our encoding
        NonRootError {
            value: id.wrapping_sub(1),
        }
    }
}
impl From<UnexpectedEofError> for NonRootError {
    #[inline]
    fn from(_: UnexpectedEofError) -> Self {
        NonRootError::unexpected_eof()
    }
}

#[derive(Error, Debug)]
pub enum DeserializeError {
    #[error("Missing field")]
    MissingField,
    #[error("Mismatched type for {0}")]
    MismatchedFieldType(String),
    #[error("Unknown field {0:?}")]
    UnknownField(String),
}

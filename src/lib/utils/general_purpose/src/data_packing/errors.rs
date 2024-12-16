use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataPackingError {
    #[error("Size ({0}) exceeds maximum size of data type: {1}")]
    SizeExceedsMaxSize(u8, u8),
    #[error("Not enough bits to read with size {0} at offset {1}")]
    NotEnoughBits(u8, u32),
}

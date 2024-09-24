use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum NetDataTypeError {
    #[error("Something failed lol")]
    SomeError,
}

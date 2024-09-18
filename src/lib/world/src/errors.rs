use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum WorldError {
    #[error("Something failed lol")]
    SomeError,
}
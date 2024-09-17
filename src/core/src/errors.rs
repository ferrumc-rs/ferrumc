use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum CoreError {
    #[error("Something failed lol")]
    SomeError,
}
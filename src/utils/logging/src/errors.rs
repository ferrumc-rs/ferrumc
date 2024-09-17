use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum LoggingError {
    #[error("Something failed lol")]
    SomeError,
}
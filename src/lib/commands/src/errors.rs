use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum CommandError {
    #[error("Something failed lol")]
    SomeError,
    #[error("Parser error: {0}")]
    ParserError(String),
}

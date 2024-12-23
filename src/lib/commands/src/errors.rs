use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum CommandError {
    #[error("Something failed lol")]
    SomeError,
}

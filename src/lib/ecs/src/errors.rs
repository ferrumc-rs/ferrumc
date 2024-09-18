use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ECSError {
    #[error("Something failed lol")]
    SomeError,
}
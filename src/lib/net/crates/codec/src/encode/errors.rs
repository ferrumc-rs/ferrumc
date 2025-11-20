#[derive(Debug, thiserror::Error)]
pub enum NetEncodeError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("External error: {0}")]
    ExternalError(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("JSON Error: {0}")]
    JsonError(String),
}

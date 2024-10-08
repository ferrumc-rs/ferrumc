
#[derive(Debug, thiserror::Error)]
pub enum NetEncodeError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
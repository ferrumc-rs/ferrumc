use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ProfilingError {
    #[error("Something failed lol")]
    SomeError,
}

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum PluginsError {
    #[error("Something failed lol")]
    SomeError,
}
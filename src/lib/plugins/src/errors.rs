use thiserror::Error;

/// Gets thrown when a plugin fails lol
#[derive(Debug, Clone, Error)]
pub enum PluginsError {
    #[error("Something failed lol")]
    SomeError,
}

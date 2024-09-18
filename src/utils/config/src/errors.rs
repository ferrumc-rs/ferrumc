use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Something failed lol")]
    SomeError,

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Configuration file TOML read error: {0}")]
    TomlError(#[from] toml::de::Error),
}
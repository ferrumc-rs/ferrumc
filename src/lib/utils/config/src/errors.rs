//! # Error types for the config module.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    /// IO error. No additional explanation needed.
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    /// TOML deserialization error. Contains the error message.
    #[error("Configuration file TOML read error:\n{0}")]
    TomlDeError(#[from] toml::de::Error),

    /// TOML serialization error. Contains the error message.
    #[error("Configuration file TOML write error:\n{0}")]
    TomlSerError(#[from] toml::ser::Error),

    /// Error when get_global_config is called before set_global_config.
    #[error("Failed to read configuration file: {0}")]
    ConfigLoadError(String),

    /// Error when set_global_config fails.
    #[error("Failed to set configuration file.")]
    ConfigSetError,

    /// Error when the root path cannot be found.
    #[error("Failed to get the root path.")]
    RootPathError(#[from] ferrumc_general_purpose::paths::RootPathError),

    /// YAML serialization/deserialization error.
    #[error("YAML error: {0}")]
    YamlError(#[from] serde_yaml_ng::Error),

    /// Custom error message.
    #[error("{0}")]
    Custom(String),
}

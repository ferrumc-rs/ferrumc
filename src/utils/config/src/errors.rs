use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Configuration file TOML read error:\n{0}")]
    TomlDeError(#[from] toml::de::Error),

    #[error("Configuration file TOML write error:\n{0}")]
    TomlSerError(#[from] toml::ser::Error),

    #[error("Failed to read configuration file.")]
    ConfigLoadError,

    #[error("get_global_config_path was called before set_global_config_path!")]
    GlobalConfigPathNotSet,

    #[error("Failed to get global_config path.")]
    SetGlobalConfigPathError,

    #[error("Failed to set configuration file.")]
    ConfigSetError,
}
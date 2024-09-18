//! # Statics module.
//!
//! Contains the static global configuration and its related functions.

use crate::errors::ConfigError;
use crate::server_config::ServerConfig;
use std::sync::OnceLock;

/// The server configuration that is stored in memory.
static CONFIG: OnceLock<ServerConfig> = OnceLock::new();

/// Helper function to get the server configuration.
///
/// **WARNING:** Configuration [ServerConfig::new] must be called before calling this function.
/// Otherwise, it will return an error.
///
/// Example of proper usage:
/// ```rust
/// # #![allow(unused_variables)]
/// # fn main() {
/// #   use ferrumc_config::{get_global_config, ServerConfig};
/// // Get config from default path.
/// ServerConfig::new(None).expect("Failed to read configuration file.");
///
/// // Do other stuff...
///
/// // Get the global configuration.
/// let config = get_global_config().expect("Failed to get global configuration.");
/// println!("{:?}", config);
/// # }
/// ```
///
/// Example of improper usage:
/// ```rust
/// # #![allow(unused_variables)]
/// # fn main() {
/// #   use ferrumc_config::get_global_config;
/// // Get the global configuration without setting the configuration first.
/// let config = get_global_config().expect("Failed to get global configuration."); // Error.
/// println!("{:?}", config);
/// # }
/// ```
pub fn get_global_config() -> Result<&'static ServerConfig, ConfigError> {
    CONFIG.get().ok_or(ConfigError::ConfigLoadError)
}

/// Sets the global configuration.
///
/// This function should be called once before calling `get_global_config()`.
///
/// Arguments:
/// - `config`: The configuration to be set globally.
pub(crate) fn set_global_config(config: ServerConfig) -> Result<(), ConfigError> {
    CONFIG.set(config).map_err(|_| ConfigError::ConfigSetError)
}

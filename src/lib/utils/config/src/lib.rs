//! # FerrumC Configuration Utilities
//!
//! This crate provides utilities for reading and storing server configurations.
//!
//! ## Examples
//!
//! Here are some examples of how to use the configuration utilities.
//!
//! ### Initialize and Read Configuration
//! ```rust
//! use ferrumc_config::ServerConfig;
//!
//! // In a scope, load the configuration from the default path.
//! {
//!     // Load the configuration from the default path.
//!     let config = ServerConfig::new(None).expect("Failed to read config file.");
//!     println!("{:?}", config);
//! }
//!
//! // In another scope, get the same configuration without loading it again.
//! {
//!    // Get the global configuration.
//!    let config = ferrumc_config::get_global_config().expect("Failed to get global config.");
//!    println!("{:?}", config);
//! }
//! ```
//!
//! ### Initialize Configuration With Custom Path
//! ```rust
//! use ferrumc_config::ServerConfig;
//!
//! // Load the configuration from a custom path.
//! let config = ServerConfig::new(Some("./custom_config.toml"))
//!     .expect("Failed to read config file.");
//!
//! println!("{:?}", config);
//! ```
//!
//! ## Organization
//!
//! The crate is organized into the following modules:
//! - [errors](errors/index.html): Error types for the config module.
//! - [server_config](server_config/index.html): Server configuration struct and functions.
//! - [statics](statics/index.html): Static global configuration and related functions.

pub mod errors;
pub mod server_config;
pub mod statics;

#[cfg(test)]
mod tests;

// Re-exports
pub use server_config::DatabaseCompression;
pub use server_config::DatabaseConfig;
pub use server_config::ServerConfig;
pub use statics::get_global_config;

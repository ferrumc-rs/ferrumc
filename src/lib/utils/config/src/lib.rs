#![feature(once_cell_try)]
//! # FerrumC Configuration Utilities
//!
//! This crate provides utilities for reading and storing server configurations.
//!
//! ## Organization
//!
//! The crate is organized into the following modules:
//! - [errors](errors/index.html): Error types for the config module.
//! - [server_config](server_config/index.html): Server configuration struct and functions.
//! - [statics](statics/index.html): Static global configuration and related functions.

pub mod errors;
pub mod favicon;
pub mod server_config;
pub mod setup;
pub mod statics;
pub mod whitelist;

// Re-exports
pub use server_config::DatabaseConfig;
pub use server_config::ServerConfig;

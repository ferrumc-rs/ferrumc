//! Command-line interface module for FerrumC.
//!
//! This module provides all CLI argument parsing and command handling
//! functionality for the FerrumC Minecraft server.
//!
//! # Submodules
//!
//! - [`clear`] - Clear command for removing server data
//!
//! # Example
//!
//! ```bash
//! # Run the server
//! ferrumc run
//!
//! # Clear all server data
//! ferrumc clear --all
//! ```

mod args;
mod clear;

pub use args::{CLIArgs, ClearArgs, Command, ImportArgs, LogLevel};
pub use clear::handle_clear;

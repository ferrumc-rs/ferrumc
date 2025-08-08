//! Command errors.

use thiserror::Error;

/// Errors related to commands.
#[derive(Debug, Clone, Error)]
pub enum CommandError {
    /// An argument parser failed
    #[error("failed to parse: {0}")]
    ParserError(String),

    /// A given argument could not be found.
    #[error("argument not found: {0}")]
    ArgumentNotFound(String),
}

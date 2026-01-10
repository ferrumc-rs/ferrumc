//! Clear command implementation for removing server data.
//!
//! This module provides functionality to clean up server data including:
//! - Configuration files (`configs/`)
//! - Whitelist file (`whitelist.txt`)
//! - Log files (`logs/`)
//! - World data (`world/`)
//!
//! The implementation is cross-platform and uses Rust's standard library
//! for file system operations.

use crate::errors::BinaryError;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use tracing::{info, warn};

use super::ClearArgs;

// ============================================================================
// Types
// ============================================================================

/// Represents the different types of data that can be cleared.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClearTarget {
    /// Configuration directory (`configs/`)
    Config,
    /// Whitelist file (`whitelist.txt`)
    Whitelist,
    /// Logs directory (`logs/`)
    Logs,
    /// World data directory (`world/`)
    World,
}

impl ClearTarget {
    /// Returns the relative path for this clear target.
    #[must_use]
    pub const fn path(&self) -> &'static str {
        match self {
            ClearTarget::Config => "configs",
            ClearTarget::Whitelist => "whitelist.txt",
            ClearTarget::Logs => "logs",
            ClearTarget::World => "world",
        }
    }

    /// Returns a human-readable description of this target.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            ClearTarget::Config => "configuration files",
            ClearTarget::Whitelist => "whitelist",
            ClearTarget::Logs => "log files",
            ClearTarget::World => "world data",
        }
    }

    /// Returns all available clear targets.
    #[must_use]
    pub const fn all() -> &'static [ClearTarget] {
        &[
            ClearTarget::Config,
            ClearTarget::Whitelist,
            ClearTarget::Logs,
            ClearTarget::World,
        ]
    }
}

/// Result of a clear operation for a single target.
#[derive(Debug)]
struct ClearResult {
    success: bool,
    message: String,
}

// ============================================================================
// Public API
// ============================================================================

/// Handles the clear command, removing selected server data.
///
/// This is the main entry point for the clear command. It confirms with the user
/// (unless `--yes` is provided), and performs the deletion of selected targets.
///
/// # Arguments
///
/// * `args` - The parsed command-line arguments for the clear command
///
/// # Errors
///
/// Returns an error if:
/// - The root path cannot be determined
/// - I/O operations fail during confirmation
/// - One or more clear operations fail
pub fn handle_clear(args: ClearArgs) -> Result<(), BinaryError> {
    let base_path = ferrumc_general_purpose::paths::get_root_path();
    let targets = collect_targets(&args);

    // Show what will be deleted
    print_deletion_preview(&base_path, &targets);

    // Confirmation prompt (unless --yes flag is provided)
    if !args.yes && !prompt_confirmation()? {
        println!("Operation cancelled.");
        return Ok(());
    }

    // Perform the clear operation
    info!("Starting clear operation...");
    let results = clear_targets(&base_path, &targets);

    // Print summary
    print_clear_summary(&results);

    // Check if any operations failed
    let failed_count = results.iter().filter(|r| !r.success).count();
    if failed_count > 0 {
        return Err(BinaryError::Custom(format!(
            "{} operation(s) failed",
            failed_count
        )));
    }

    info!("Clear operation completed successfully.");
    Ok(())
}

// ============================================================================
// Internal Functions
// ============================================================================

/// Collects the targets to clear based on the provided arguments.
fn collect_targets(args: &ClearArgs) -> Vec<ClearTarget> {
    if args.all {
        return ClearTarget::all().to_vec();
    }

    let mut targets = Vec::new();
    if args.config {
        targets.push(ClearTarget::Config);
    }
    if args.whitelist {
        targets.push(ClearTarget::Whitelist);
    }
    if args.logs {
        targets.push(ClearTarget::Logs);
    }
    if args.world {
        targets.push(ClearTarget::World);
    }
    targets
}

/// Prints a preview of what will be deleted.
fn print_deletion_preview(base_path: &Path, targets: &[ClearTarget]) {
    println!("The following will be deleted:");
    for target in targets {
        let path = base_path.join(target.path());
        let exists_marker = if path.exists() { "" } else { " (not found)" };
        println!("  - {}{}", target.path(), exists_marker);
    }
    println!();
}

/// Prompts the user for confirmation.
///
/// # Returns
///
/// `Ok(true)` if the user confirms, `Ok(false)` otherwise.
fn prompt_confirmation() -> Result<bool, BinaryError> {
    print!("Are you sure you want to continue? [y/N] ");
    io::stdout().flush().map_err(BinaryError::Io)?;

    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(BinaryError::Io)?;

    let input = input.trim().to_lowercase();
    Ok(input == "y" || input == "yes")
}

/// Removes a file or directory at the given path.
///
/// This function handles both files and directories, removing them recursively
/// if necessary. It's designed to be cross-platform compatible.
///
/// # Arguments
///
/// * `path` - The path to the file or directory to remove
///
/// # Returns
///
/// * `Ok(true)` - The path was successfully removed
/// * `Ok(false)` - The path did not exist (nothing to remove)
/// * `Err(io::Error)` - An error occurred during removal
fn remove_path(path: &Path) -> io::Result<bool> {
    if !path.exists() {
        return Ok(false);
    }

    if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }

    Ok(true)
}

/// Clears a single target from the server directory.
///
/// # Arguments
///
/// * `base_path` - The base directory of the server
/// * `target` - The target to clear
///
/// # Returns
///
/// A `ClearResult` containing the outcome of the operation.
fn clear_target(base_path: &Path, target: ClearTarget) -> ClearResult {
    let target_path = base_path.join(target.path());

    match remove_path(&target_path) {
        Ok(true) => {
            info!(
                "Cleared {}: {}",
                target.description(),
                target_path.display()
            );
            ClearResult {
                success: true,
                message: format!("Successfully removed {}", target.description()),
            }
        }
        Ok(false) => {
            info!(
                "Skipped {} (not found): {}",
                target.description(),
                target_path.display()
            );
            ClearResult {
                success: true,
                message: format!("{} not found (already clean)", target.description()),
            }
        }
        Err(e) => {
            warn!(
                "Failed to clear {}: {} - {}",
                target.description(),
                target_path.display(),
                e
            );
            ClearResult {
                success: false,
                message: format!("Failed to remove {}: {}", target.description(), e),
            }
        }
    }
}

/// Clears multiple targets from the server directory.
///
/// # Arguments
///
/// * `base_path` - The base directory of the server
/// * `targets` - The targets to clear
///
/// # Returns
///
/// A vector of `ClearResult` containing the outcome of each operation.
fn clear_targets(base_path: &Path, targets: &[ClearTarget]) -> Vec<ClearResult> {
    targets
        .iter()
        .map(|target| clear_target(base_path, *target))
        .collect()
}

/// Prints a summary of clear results to the console.
fn print_clear_summary(results: &[ClearResult]) {
    let successful = results.iter().filter(|r| r.success).count();
    let total = results.len();

    println!();
    println!("Clear Summary");
    println!("{}", "-".repeat(50));

    for result in results {
        let status = if result.success { "✓" } else { "✗" };
        println!("  {} {}", status, result.message);
    }

    println!("{}", "-".repeat(50));
    println!("Completed: {}/{} operations successful", successful, total);
}

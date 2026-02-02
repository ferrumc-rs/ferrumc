//! CLI argument definitions and parsing.
//!
//! This module contains all the argument structures used by the FerrumC CLI,
//! powered by the `clap` crate.

use clap::{ArgGroup, Parser, Subcommand, ValueEnum};
use tracing::Level;

/// Main CLI arguments for FerrumC.
#[derive(Parser)]
pub struct CLIArgs {
    /// The subcommand to execute
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Log level for the application
    #[clap(long)]
    #[arg(value_enum)]
    #[cfg_attr(debug_assertions, arg(default_value_t = LogLevel(Level::DEBUG)))]
    #[cfg_attr(not(debug_assertions), arg(default_value_t = LogLevel(Level::INFO)))]
    pub log: LogLevel,

    /// Disable interactive TUI
    #[arg(long, default_value_t = false)]
    pub no_tui: bool,
}

/// Available CLI commands.
#[derive(Subcommand, Clone)]
pub enum Command {
    /// Sets up the config
    Setup,
    /// Import the world data
    Import(ImportArgs),
    /// Start the server
    Run,
    /// Clear server data (configs, whitelist, logs, world)
    Clear(ClearArgs),
}

/// Arguments for the clear command.
///
/// At least one target must be specified (or use `--all` for everything).
#[derive(Debug, Clone, Parser)]
#[command(group(
    ArgGroup::new("targets")
        .required(true)
        .multiple(true)
        .args(["config", "whitelist", "logs", "world", "all"]),
))]
pub struct ClearArgs {
    /// Clear configuration files (configs/)
    #[arg(long, short = 'c')]
    pub config: bool,

    /// Clear whitelist file (whitelist.txt)
    #[arg(long, short = 'w')]
    pub whitelist: bool,

    /// Clear log files (logs/)
    #[arg(long, short = 'l')]
    pub logs: bool,

    /// Clear world data (world/)
    #[arg(long, short = 'W')]
    pub world: bool,

    /// Clear all data (equivalent to --config --whitelist --logs --world)
    #[arg(long, short = 'a')]
    pub all: bool,

    /// Skip confirmation prompt (use with caution)
    #[arg(long, short = 'y')]
    pub yes: bool,
}

/// Arguments for the import command.
#[derive(Debug, Clone, Parser)]
pub struct ImportArgs {
    /// Path to world import folder
    ///
    /// This should point to the folder that contains directories such as `region`, `poi`,
    /// `playerdata`, etc. Usually found at %APPDATA%/.minecraft/saves.
    #[clap(long, required = true)]
    pub import_path: String,

    /// Number of chunks to process at a time
    #[clap(env, default_value_t = 1000)]
    pub batch_size: usize,

    /// Number of concurrent tasks (limits memory spending)
    #[clap(env, default_value_t = 512)]
    pub max_concurrent_tasks: usize,
}

/// Wrapper struct for the tracing `Level` enum to implement `ValueEnum`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogLevel(pub Level);

impl ValueEnum for LogLevel {
    fn value_variants<'a>() -> &'a [Self] {
        static VARIANTS: &[LogLevel] = &[
            LogLevel(Level::TRACE),
            LogLevel(Level::DEBUG),
            LogLevel(Level::INFO),
            LogLevel(Level::WARN),
            LogLevel(Level::ERROR),
        ];
        VARIANTS
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self.0 {
            Level::TRACE => Some(clap::builder::PossibleValue::new("trace")),
            Level::DEBUG => Some(clap::builder::PossibleValue::new("debug")),
            Level::INFO => Some(clap::builder::PossibleValue::new("info")),
            Level::WARN => Some(clap::builder::PossibleValue::new("warn")),
            Level::ERROR => Some(clap::builder::PossibleValue::new("error")),
        }
    }
}

impl From<LogLevel> for Level {
    fn from(log_level: LogLevel) -> Self {
        log_level.0
    }
}

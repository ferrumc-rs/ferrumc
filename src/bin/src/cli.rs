use clap::{Parser, Subcommand, ValueEnum};
use tracing::Level;

/// Parser for command arguments
#[derive(Parser)]
pub struct CLIArgs {
    /// The Subcommands
    #[command(subcommand)]
    pub command: Option<Command>,
    /// Customizable Logging level
    #[clap(long)]
    #[arg(value_enum)]
    #[cfg_attr(debug_assertions, arg(default_value_t = LogLevel(Level::DEBUG)))]
    #[cfg_attr(not(debug_assertions), arg(default_value_t = LogLevel(Level::INFO)))]
    pub log: LogLevel,
}

/// The subcommands that could be given when executing the application from the binary.
#[derive(Subcommand, Clone)]
pub enum Command {
    /// Sets up the config
    Setup,
    /// Import the world data
    Import(ImportArgs),
    /// Start the server
    Run,
}

/// Arguments that could be given if the subcommand used was [Command::Import]
#[derive(Debug, Clone, Parser)]
pub struct ImportArgs {
    /// Path to world import folder
    ///
    /// This should point to the folder that contains directories such as `region`, `poi`,  `playerdata`, etc. Usually found at %APPDATA%/.minecraft/saves.
    #[clap(long, required = true)]
    pub import_path: String,
    /// Number of chunks to process at a time
    #[clap(env, default_value_t = 1000)]
    pub batch_size: usize,
    /// Number of concurrent tasks (limits memory spending)
    #[clap(env, default_value_t = 512)]
    pub max_concurrent_tasks: usize,
}

/// Wrapper struct for the Level enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogLevel(Level);

/// Implement [`ValueEnum`] for the wrapper
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

/// Add a conversion method to make using the wrapper easier
impl From<LogLevel> for Level {
    fn from(log_level: LogLevel) -> Self {
        log_level.0
    }
}

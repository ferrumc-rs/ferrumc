use clap::{Parser, Subcommand, ValueEnum};
use tracing::Level;

#[derive(Parser)]
pub struct CLIArgs {
    #[command(subcommand)]
    pub command: Option<Command>,
    #[clap(long)]
    #[arg(value_enum, default_value_t = LogLevel(Level::TRACE))]
    pub log: LogLevel,
}

#[derive(Subcommand, Clone)]
pub enum Command {
    /// Sets up the config, etc.
    Setup,
    /// Import the world data
    Import,
    /// Start the server
    Run,
}

// Wrapper struct for the Level enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogLevel(Level);

// Implement `ValueEnum` for the wrapper
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

// Add a conversion method to make using the wrapper easier
impl From<LogLevel> for Level {
    fn from(log_level: LogLevel) -> Self {
        log_level.0
    }
}

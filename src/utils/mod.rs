use tracing_subscriber::filter::Directive;

use crate::utils::constants::DEFAULT_LOG_LEVEL;
use crate::utils::prelude::*;

pub mod encoding;
pub mod error;
pub mod prelude;
pub mod type_impls;

pub mod components;
pub mod config;
pub mod constants;
pub mod nbt_impls;

/// Sets up the logger. Needs to be run before anything else in order for logging to run end.
pub fn setup_logger() -> Result<()> {
    let trace_level = std::env::args()
        .find(|arg| arg.starts_with("--log="))
        .map(|arg| arg.replace("--log=", ""));

    let mut trace_level: &str = trace_level.as_deref().unwrap_or("");
    if trace_level.is_empty() {
        eprintln!(
            "No log level specified, using default: {}",
            DEFAULT_LOG_LEVEL
        );
        trace_level = DEFAULT_LOG_LEVEL;
    }

    let trace_level = match trace_level.trim().parse::<tracing::Level>() {
        Ok(level) => level,
        Err(_) => {
            eprintln!("Invalid log level: {}", trace_level);
            eprintln!("Possible values: trace, debug, info, warn, error");
            std::process::exit(1);
        }
    };

    let env_filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive(trace_level.into())
        .add_directive(str_to_directive("sled=off")?);

    Ok(tracing_subscriber::fmt().with_env_filter(env_filter).init())
}

fn str_to_directive(s: &str) -> Result<Directive> {
    s.parse()
        .map_err(|_| Error::InvalidDirective(s.to_string()))
}

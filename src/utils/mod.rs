pub mod config;

pub mod encoding;

use crate::constants::DEFAULT_LOG_LEVEL;

pub fn setup_logger() {
    let mut trace_level = std::env::args()
        .find(|arg| arg.starts_with("--log="))
        .map(|arg| arg.replace("--log=", ""));

    let mut trace_level : &str = trace_level.as_deref().unwrap_or("");
    if trace_level.is_empty() {
        eprintln!("No log level specified, using default: {}", DEFAULT_LOG_LEVEL);
        trace_level = DEFAULT_LOG_LEVEL;
    }

    let trace_level = match trace_level.trim().parse::<log::LevelFilter>() {
        Ok(level) => level,
        Err(_) => {
            eprintln!("Invalid log level: {}", trace_level);
            eprintln!("Possible values: trace, debug, info, warn, error");
            std::process::exit(1);
        }
    };

    env_logger::builder()
        .filter_module("warp", log::LevelFilter::Info)
        .filter_module("hyper", log::LevelFilter::Info)
        .filter_module("tracing", log::LevelFilter::Info)
        .filter_module("tokio_util", log::LevelFilter::Info)
        .filter_level(trace_level)
        .init();
}

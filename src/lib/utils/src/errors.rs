use ferrumc_config::errors::ConfigError;
use ferrumc_logging::errors::LoggingError;
use ferrumc_profiling::errors::ProfilingError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UtilsError {
    #[error("Something failed lol")]
    SomeError,

    #[error("Logging error: {0}")]
    LoggingError(#[from] LoggingError),

    #[error("Profiling error: {0}")]
    ProfilingError(#[from] ProfilingError),

    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),
}

use thiserror::Error;
use ferrumc_logging::errors::LoggingError;
use ferrumc_profiling::errors::ProfilingError;

#[derive(Debug, Clone, Error)]
pub enum UtilsError {
    #[error("Something failed lol")]
    SomeError,

    #[error("Logging error: {0}")]
    LoggingError(#[from] LoggingError),

    #[error("Profiling error: {0}")]
    ProfilingError(#[from] ProfilingError),
}
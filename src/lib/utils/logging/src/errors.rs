use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoggingError {
    /// Rolling file appender could not create or open the log directory.
    #[error(
        "failed to create log directory at `{path}`: {source}\n\
         Ensure the process has permission to create and write files in that directory."
    )]
    LogDirectory {
        path: String,
        #[source]
        source: io::Error,
    },
}

impl LoggingError {
    /// Builds a [`LoggingError::LogDirectory`] from a tracing-appender init failure.
    pub(crate) fn from_appender_init(
        path: impl AsRef<std::path::Path>,
        err: tracing_appender::rolling::InitError,
    ) -> Self {
        use std::error::Error as _;

        let path = path.as_ref().display().to_string();
        let source = err
            .source()
            .and_then(|source| source.downcast_ref::<io::Error>())
            .map(|io_err| io::Error::new(io_err.kind(), io_err.to_string()))
            .unwrap_or_else(|| io::Error::other(err.to_string()));

        Self::LogDirectory { path, source }
    }
}

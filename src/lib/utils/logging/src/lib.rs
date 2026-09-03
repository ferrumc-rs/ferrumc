pub mod errors;

use crate::errors::LoggingError;
use ferrumc_general_purpose::paths::get_root_path;
use ferrumc_profiling::ProfilerTracingLayer;
use std::path::Path;
use tracing::Level;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

/// Initializes console and rolling-file logging under `{root}/logs`.
///
/// Returns [`LoggingError`] when the log directory cannot be created or opened
/// (for example, permission denied), instead of panicking.
pub fn init_logging(trace_level: Level) -> Result<(), LoggingError> {
    init_logging_to(trace_level, get_root_path().join("logs"))
}

/// Same as [`init_logging`], but writes under `logs_dir` (used by tests).
pub(crate) fn init_logging_to(
    trace_level: Level,
    logs_dir: impl AsRef<Path>,
) -> Result<(), LoggingError> {
    let logs_dir = logs_dir.as_ref();

    let env_filter = EnvFilter::builder()
        .with_default_directive(trace_level.into())
        .parse_lossy("");

    // Disallow request and hyper-util debug prints since they spam the console
    let env_filter = env_filter
        .add_directive(
            "reqwest=warn"
                .parse()
                .expect("static filter directive `reqwest=warn` must parse"),
        )
        .add_directive(
            "h2=warn"
                .parse()
                .expect("static filter directive `h2=warn` must parse"),
        )
        .add_directive(
            "rustls=warn"
                .parse()
                .expect("static filter directive `rustls=warn` must parse"),
        )
        .add_directive(
            "hyper_util=warn"
                .parse()
                .expect("static filter directive `hyper_util=warn` must parse"),
        );

    let file_appender = tracing_appender::rolling::Builder::new()
        .rotation(Rotation::DAILY)
        .filename_prefix("ferrumc")
        .filename_suffix("log.txt")
        .build(logs_dir)
        .map_err(|err| LoggingError::from_appender_init(logs_dir, err))?;

    let fmt_layer = {
        #[cfg(debug_assertions)]
        {
            tracing_subscriber::fmt::layer()
                .with_file(true)
                .with_line_number(true)
                .with_level(true)
                .with_target(false)
        }
        #[cfg(not(debug_assertions))]
        {
            tracing_subscriber::fmt::layer()
                .with_thread_ids(false)
                .with_thread_names(false)
                .with_file(false)
                .with_line_number(false)
                .with_level(true)
                .with_target(false)
        }
    };

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(file_appender)
        .with_ansi(false);

    let profiler_layer = ProfilerTracingLayer;

    let registry = tracing_subscriber::registry()
        .with(file_layer)
        .with(env_filter)
        .with(profiler_layer)
        .with(fmt_layer);

    #[cfg(not(feature = "tracy"))]
    {
        registry.init();
    }

    #[cfg(feature = "tracy")]
    {
        let tracy_layer = tracing_tracy::TracyLayer::default();
        // Registry becomes a different type when a layer is added, so it is
        // shadowed here and initialized separately.
        let registry = registry.with(tracy_layer);
        registry.init();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;

    #[cfg(unix)]
    use std::fs;
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;
    #[cfg(unix)]
    use std::path::PathBuf;

    #[test]
    fn log_directory_error_message_is_actionable() {
        let err = LoggingError::LogDirectory {
            path: "/unwritable/logs".to_string(),
            source: std::io::Error::new(ErrorKind::PermissionDenied, "Permission denied"),
        };
        let message = err.to_string();
        assert!(
            message.contains("failed to create log directory at `/unwritable/logs`"),
            "message missing path context: {message}"
        );
        assert!(
            message.contains("Permission denied"),
            "message missing OS reason: {message}"
        );
        assert!(
            message.contains("permission to create and write"),
            "message missing remediation hint: {message}"
        );
    }

    #[test]
    #[cfg(unix)]
    fn init_logging_to_unwritable_dir_returns_error() {
        let temp = tempfile::tempdir().expect("tempdir");
        let logs_dir: PathBuf = temp.path().join("logs");
        fs::create_dir(&logs_dir).expect("create logs dir");

        let mut perms = fs::metadata(&logs_dir).expect("metadata").permissions();
        // Owner read+execute only — create_dir/open for append should fail.
        perms.set_mode(0o500);
        fs::set_permissions(&logs_dir, perms).expect("chmod");

        let result = init_logging_to(Level::INFO, &logs_dir);

        // Restore writable so TempDir cleanup succeeds.
        let mut perms = fs::metadata(&logs_dir).expect("metadata").permissions();
        perms.set_mode(0o700);
        fs::set_permissions(&logs_dir, perms).expect("chmod restore");

        let err = result.expect_err("expected permission failure");
        let LoggingError::LogDirectory { path, source } = err;
        assert_eq!(path, logs_dir.display().to_string());
        assert_eq!(source.kind(), ErrorKind::PermissionDenied);
    }
}

pub mod errors;

use ferrumc_general_purpose::paths::get_root_path;
use ferrumc_profiling::ProfilerTracingLayer;
use tracing::Level;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub fn init_logging(trace_level: Level) {
    //let console = console_subscriber::spawn();
    let env_filter = EnvFilter::builder()
        .with_default_directive(trace_level.into())
        .parse_lossy("");

    let env_filter = env_filter
        .add_directive("ureq=warn".parse().unwrap())
        .add_directive("rustls=warn".parse().unwrap());

    let file_appender = tracing_appender::rolling::Builder::new()
        .rotation(Rotation::DAILY)
        .filename_prefix("ferrumc")
        .filename_suffix("log.txt")
        .build(get_root_path().join("logs"))
        .unwrap();

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

    tracing_subscriber::registry()
        .with(file_layer)
        .with(env_filter)
        .with(profiler_layer)
        .with(fmt_layer)
        .init();
}

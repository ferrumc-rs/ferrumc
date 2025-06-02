pub mod errors;

use ferrumc_general_purpose::paths::get_root_path;
use ferrumc_profiling::ProfilerTracingLayer;
use tracing::Level;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::fmt::{layer, Layer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub fn init_logging(trace_level: Level) {
    //let console = console_subscriber::spawn();
    let env_filter = EnvFilter::from_default_env().add_directive(trace_level.into());

    let is_verbose = trace_level > Level::INFO;

    let file_layer = {
        let file_appender = tracing_appender::rolling::Builder::new()
            .rotation(Rotation::DAILY)
            .filename_prefix("ferrumc")
            .filename_suffix("log.txt")
            .build(get_root_path().join("logs"))
            .unwrap();

        if is_verbose {
            layer().with_writer(file_appender).with_ansi(false)
        } else {
            layer()
                .with_ansi(false)
                .with_writer(file_appender)
                .with_target(false)
                .with_thread_ids(false)
                .with_line_number(false)
                .with_file(false)
        }
    };

    let mut fmt_layer = Layer::default();

    // remove path from logs if log level is INFO
    if !is_verbose {
        fmt_layer = fmt_layer
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false);
    }
    #[cfg(debug_assertions)]
    {
        fmt_layer = fmt_layer.with_file(true).with_line_number(true);
    }

    let profiler_layer = ProfilerTracingLayer;

    tracing_subscriber::registry()
        .with(file_layer)
        .with(env_filter)
        .with(profiler_layer)
        .with(fmt_layer)
        .init();
}

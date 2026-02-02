pub mod errors;
pub mod tui_formatter;

use ferrumc_general_purpose::paths::get_root_path;
use ferrumc_profiling::ProfilerTracingLayer;
use log::LevelFilter::Debug;
use tracing::Level;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
use tui_logger::TuiTracingSubscriberLayer;

pub fn init_logging(trace_level: Level, no_tui: bool) {
    //let console = console_subscriber::spawn();
    let env_filter = EnvFilter::builder()
        .with_default_directive(trace_level.into())
        .parse_lossy("");

    // Disallow request and hyper-util debug prints since they spam the console
    let env_filter = env_filter
        .add_directive("reqwest=warn".parse().unwrap())
        .add_directive("h2=warn".parse().unwrap())
        .add_directive("rustls=warn".parse().unwrap())
        .add_directive("hyper_util=warn".parse().unwrap());

    let file_appender = tracing_appender::rolling::Builder::new()
        .rotation(Rotation::DAILY)
        .filename_prefix("ferrumc")
        .filename_suffix("log.txt")
        .build(get_root_path().join("logs"))
        .unwrap();

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(file_appender)
        .with_ansi(false);

    let profiler_layer = ProfilerTracingLayer;

    let registry = tracing_subscriber::registry()
        .with(file_layer)
        .with(env_filter)
        .with(profiler_layer);

    #[cfg(feature = "tracy")]
    let tracy_layer = tracing_tracy::TracyLayer::default();
    // Registry becomes a different type when a layer is added, so we need to
    // shadow it here.
    #[cfg(feature = "tracy")]
    let registry = registry.with(tracy_layer);

    if no_tui {
        let layer = {
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
        registry.with(layer).init();
    } else {
        tui_logger::init_logger(Debug).expect("Failed to initialize TUI logger");
        registry.with(TuiTracingSubscriberLayer).init();
    };
}

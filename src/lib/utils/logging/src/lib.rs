pub mod errors;

use ferrumc_profiling::ProfilerTracingLayer;
use tracing::Level;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

const LOG_LEVEL: &str = "trace";

pub fn init_logging() {
    let trace_level = {
        /*let trace_level = std::env::args()
            .find(|arg| arg.starts_with("--log="))
            .map(|arg| arg.replace("--log=", ""));*/
        let trace_level = std::env::var("FERRUMC_LOG").ok();

        let trace_level = trace_level.unwrap_or_else(|| LOG_LEVEL.to_string());

        let trace_level = match trace_level.trim().parse::<tracing::Level>() {
            Ok(level) => level,
            Err(_) => {
                eprintln!("Invalid log level: {}", trace_level);
                eprintln!("Possible values: trace, debug, info, warn, error");
                eprintln!("Using default log level: trace");
                Level::TRACE
            }
        };

        trace_level
    };

    let env_filter = EnvFilter::from_default_env()
            .add_directive(trace_level.into());

    let mut fmt_layer = Layer::default();

    // remove path from logs if log level is INFO
    if trace_level == Level::INFO {
        fmt_layer = fmt_layer.with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false);
    }

    let profiler_layer = ProfilerTracingLayer;

    tracing_subscriber::registry()
        .with(env_filter)
        .with(profiler_layer)
        .with(fmt_layer)
        .init();
}

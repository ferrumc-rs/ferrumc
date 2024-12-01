pub mod errors;

use ferrumc_profiling::ProfilerTracingLayer;
use tracing::Level;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init_logging(trace_level: tracing::Level) {

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

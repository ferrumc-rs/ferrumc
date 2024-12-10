pub mod errors;

use ferrumc_profiling::ProfilerTracingLayer;
use tracing::Level;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub fn init_logging(trace_level: Level) {
    //let console = console_subscriber::spawn();
    let env_filter = EnvFilter::from_default_env()
        .add_directive(trace_level.into())
        .add_directive("tokio=off".parse().unwrap())
        .add_directive("runtime=off".parse().unwrap());

    let mut fmt_layer = Layer::default();

    // remove path from logs if log level is INFO
    if trace_level == Level::INFO {
        fmt_layer = fmt_layer
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false);
    }

    let profiler_layer = ProfilerTracingLayer;

    tracing_subscriber::registry()
        //    .with(console)
        .with(env_filter)
        .with(profiler_layer)
        .with(fmt_layer)
        .init();
}

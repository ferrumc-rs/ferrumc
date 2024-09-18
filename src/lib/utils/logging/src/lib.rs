pub mod errors;

use ferrumc_profiling::ProfilerTracingLayer;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

static LOG_LEVEL: Level = Level::TRACE;

pub fn init_logging() {
    let env_filter =
        tracing_subscriber::EnvFilter::from_default_env().add_directive(LOG_LEVEL.into());

    let fmt_layer = tracing_subscriber::fmt::Layer::default();

    let profiler_layer = ProfilerTracingLayer;
    tracing_subscriber::registry()
        .with(env_filter)
        .with(profiler_layer)
        .with(fmt_layer)
        .init();
}

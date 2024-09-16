use tracing::Level;

static LOG_LEVEL: Level = Level::INFO;

pub async fn init_logging() {
    let env_filter =
        tracing_subscriber::EnvFilter::from_default_env().add_directive(LOG_LEVEL.into());

    tracing_subscriber::fmt().with_env_filter(env_filter).init();
}

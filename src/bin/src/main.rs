// Security or something like that
#![forbid(unsafe_code)]
extern crate core;

use crate::errors::BinaryError;
use clap::{ Parser, ValueEnum };
use ferrumc_config::statics::get_global_config;
use ferrumc_ecs::Universe;
use ferrumc_general_purpose::paths::get_root_path;
use ferrumc_net::server::create_server_listener;
use ferrumc_net::ServerState;
use ferrumc_world::World;
use std::sync::Arc;
use systems::definition;
use tracing::{error, info, Level};

#[derive(clap::Parser)]
struct CLIArgs {
    #[clap(long)]
    import: bool,
    #[clap(long)]
    #[arg(value_enum, default_value_t = LogLevel(Level::TRACE))]
    log: LogLevel,
}

// Wrapper struct for the Level enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogLevel(Level);

// Implement `ValueEnum` for the wrapper
impl ValueEnum for LogLevel {
    fn value_variants<'a>() -> &'a [Self] {
        static VARIANTS: &[LogLevel] = &[
            LogLevel(Level::TRACE),
            LogLevel(Level::DEBUG),
            LogLevel(Level::INFO),
            LogLevel(Level::WARN),
            LogLevel(Level::ERROR),
        ];
        VARIANTS
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self.0 {
            Level::TRACE => Some(clap::builder::PossibleValue::new("trace")),
            Level::DEBUG => Some(clap::builder::PossibleValue::new("debug")),
            Level::INFO => Some(clap::builder::PossibleValue::new("info")),
            Level::WARN => Some(clap::builder::PossibleValue::new("warn")),
            Level::ERROR => Some(clap::builder::PossibleValue::new("error")),
        }
    }
}

// Add a conversion method to make using the wrapper easier
impl From<LogLevel> for Level {
    fn from(log_level: LogLevel) -> Self {
        log_level.0
    }
}
pub(crate) mod errors;
mod packet_handlers;
mod systems;

pub type Result<T> = std::result::Result<T, BinaryError>;

#[tokio::main]
async fn main() {
    let cli_args = CLIArgs::parse();
    ferrumc_logging::init_logging(cli_args.log.into());

    info!("Starting server...");

    if let Err(e) = entry(cli_args).await {
        error!("Server exited with the following error: {}", e.to_string());
    } else {
        info!("Server exited successfully.");
    }
}

async fn entry(cli_args: CLIArgs) -> Result<()> {
    if handle_import(cli_args.import).await? {
        return Ok(());
    }

    let state = create_state().await?;
    let global_state = Arc::new(state);


    let all_system_handles = tokio::spawn(definition::start_all_systems(global_state.clone()));

    // Start the systems and wait until all of them are done
    all_system_handles.await??;

    // Stop all systems
    definition::stop_all_systems(global_state).await?;

    Ok(())
}


async fn handle_import(import: bool) -> Result<bool> {
    //! Handles the import of the world if the `--import` flag is set.
    //! Returns `true` if program should exit after this function, `false` otherwise.
    if !import {
        return Ok(false);
    }

    info!("`--import` flag detected. Importing world...");

    // Import the world
    let config = get_global_config();
    let mut world = World::new().await;

    let root_path = get_root_path();
    let database_opts = &config.database;


    let mut import_path = root_path.join(database_opts.import_path.clone());
    if import_path.is_relative() {
        import_path = root_path.join(import_path);
    }
    let mut db_path = root_path.join(database_opts.db_path.clone());
    if db_path.is_relative() {
        db_path = root_path.join(db_path);
    }

    if let Err(e) = world.import(import_path, db_path).await {
        error!("Could not import world: {}", e.to_string());
        return Err(BinaryError::Custom("Could not import world.".to_string()));
    }

    Ok(true)
}

async fn create_state() -> Result<ServerState> {
    let listener = create_server_listener().await?;

    Ok(ServerState {
        universe: Universe::new(),
        tcp_listener: listener,
        world: World::new().await,
    })
}

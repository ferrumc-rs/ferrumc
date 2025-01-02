#![feature(portable_simd)]
#![forbid(unsafe_code)]
extern crate core;

use crate::cli::{CLIArgs, Command, ImportArgs};
use crate::errors::BinaryError;
use clap::Parser;
use ferrumc_config::statics::get_global_config;
use ferrumc_config::whitelist::create_whitelist;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_ecs::Universe;
use ferrumc_general_purpose::paths::get_root_path;
use ferrumc_net::packets::outgoing::login_success::LoginSuccessPacket;
use ferrumc_net::server::create_server_listener;
use ferrumc_net::{connection::StreamWriter, NetResult};
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_state::ServerState;
use ferrumc_world::World;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use systems::definition;
use tracing::{error, info, trace};

mod cli;
pub(crate) mod errors;
mod packet_handlers;
mod systems;

pub mod events;

mod velocity;
mod whitelist;

pub type Result<T> = std::result::Result<T, BinaryError>;

pub async fn send_login_success(
    state: Arc<ServerState>,
    conn_id: usize,
    identity: PlayerIdentity,
) -> NetResult<()> {
    //Send a Login Success Response to further the login sequence
    let mut writer = state.universe.get_mut::<StreamWriter>(conn_id)?;

    writer
        .send_packet(
            &LoginSuccessPacket::new(identity.clone()),
            &NetEncodeOpts::WithLength,
        )
        .await?;

    state
        .universe
        .add_component::<PlayerIdentity>(conn_id, identity)?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let cli_args = CLIArgs::parse();
    ferrumc_logging::init_logging(cli_args.log.into());

    check_deadlocks();

    {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::any::TypeId::of::<ChunkReceiver>().hash(&mut hasher);
        let digest = hasher.finish();
        trace!("ChunkReceiver: {:X}", digest);
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::any::TypeId::of::<StreamWriter>().hash(&mut hasher);
        let digest = hasher.finish();
        trace!("StreamWriter: {:X}", digest);
    }

    match cli_args.command {
        Some(Command::Setup) => {
            info!("Starting setup...");
            if let Err(e) = ferrumc_config::setup::setup() {
                error!("Could not set up the server: {}", e.to_string());
            } else {
                info!("Server setup complete.");
            }
        }

        Some(Command::Import(import_args)) => {
            info!("Starting import...");
            if let Err(e) = handle_import(import_args).await {
                error!("Import failed with the following error: {}", e.to_string());
            } else {
                info!("Import completed successfully.");
            }
        }
        Some(Command::Run) | None => {
            info!("Starting server...");
            if let Err(e) = entry().await {
                error!("Server exited with the following error: {}", e.to_string());
            } else {
                info!("Server exited successfully.");
            }
        }
    }
}

async fn entry() -> Result<()> {
    let state = create_state().await?;
    let global_state = Arc::new(state);
    create_whitelist().await;

    let all_system_handles = tokio::spawn(definition::start_all_systems(global_state.clone()));

    //Start the systems and wait until all of them are done
    all_system_handles.await??;

    // Stop all systems
    definition::stop_all_systems(global_state).await?;

    Ok(())
}

async fn handle_import(import_args: ImportArgs) -> Result<()> {
    //! Handles the import of the world.
    info!("Importing world...");

    let config = get_global_config();
    let mut world = World::new().await;

    let root_path = get_root_path();
    let database_opts = &config.database;

    let mut import_path = root_path.join(import_args.import_path);
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

    Ok(())
}

async fn create_state() -> Result<ServerState> {
    let listener = create_server_listener().await?;

    Ok(ServerState {
        universe: Universe::new(),
        tcp_listener: listener,
        world: World::new().await,
    })
}

fn check_deadlocks() {
    {
        use parking_lot::deadlock;
        use std::thread;
        use std::time::Duration;

        // Create a background thread which checks for deadlocks every 10s
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(10));
            let deadlocks = deadlock::check_deadlock();
            if deadlocks.is_empty() {
                continue;
            }

            println!("{} deadlocks detected", deadlocks.len());
            for (i, threads) in deadlocks.iter().enumerate() {
                println!("Deadlock #{}", i);
                for t in threads {
                    println!("Thread Id {:#?}", t.thread_id());
                    println!("{:#?}", t.backtrace());
                }
            }
        });
    }
}

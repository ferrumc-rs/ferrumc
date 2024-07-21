#![feature(box_into_inner)]
#![feature(fs_try_exists)]

use std::env;
use std::sync::Arc;

#[warn(unused_imports)]
use clap::Parser;
use ferrumc_net::{Connection, ConnectionWrapper, GET_WORLD};
use ferrumc_net::packets::outgoing::keep_alive::KeepAlivePacketOut;
use ferrumc_net::systems::{ALL_SYSTEMS, kill_all_systems, start_all_systems};
use ferrumc_utils::components::keep_alive::KeepAlive;
use ferrumc_utils::components::player::Player;
use ferrumc_utils::config::get_global_config;
use ferrumc_utils::encoding::position::Position;
use ferrumc_utils::prelude::*;
#[allow(unused_imports)]
use tokio::fs::try_exists;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tracing::{debug, error, info, info_span, Instrument, trace};

mod setup;
mod tests;
mod utils;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(long, default_value = "false")]
    setup: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    if handle_setup().await? {
        return Ok(());
    }

    utils::setup_logger();
    info!("Initializing server...");

    let start = std::time::Instant::now();
    let config = ferrumc_utils::config::ServerConfig::new()?;
    let elapsed = start.elapsed();

    debug!("Found Config: {:?} in {:?}", config, elapsed);


    start_server()
        .await
        .expect("Server failed to start!");

    tokio::signal::ctrl_c().await?;

    Ok(())
}

/// Starts the server. Sets up the sockets and listens for incoming connections
///
/// The actual management of connections in handled by [ferrumc_net::init_connection]
async fn start_server() -> Result<()> {
    let config = get_global_config();
    trace!("Starting server on {}:{}", config.host, config.port);

    let tcp_addr = format!("{}:{}", config.host, config.port);

    let listener = TcpListener::bind(tcp_addr).await?;
    let addr = listener.local_addr()?;

    info!("Server started on {}", addr);


    let read_connections = tokio::spawn(read_connections(listener));


    /*ALL_SYSTEMS.iter().for_each(|system| {
        tokio::spawn(system.run().instrument(info_span!("system", system = system.name())));
    });*/
    /*let systems = tokio::task::spawn(async {
        loop {
            let world = GET_WORLD().read().await;
            // an example system (like just log all players)
            for (id, (player, position)) in world.query::<(Player, Position)>().iter() {
                info!("[Entity {}] Player: {:?}, Position: {:?}", id, player, position);
            }
            drop(world);

            let mut world = GET_WORLD().write().await;

            let keep_alive_data: Vec<(usize, (String, i64, Arc<RwLock<Connection>>))> = world
                .query_mut::<(Player, KeepAlive, ConnectionWrapper)>()
                .iter_mut()
                .map(|(entity_id, (player, keep_alive, conn))| {
                    keep_alive.data += 1;
                    keep_alive.last_sent = std::time::Instant::now();
                    (entity_id, (player.get_username().to_string(), keep_alive.data, conn.0.clone()))
                })
                .collect();

            drop(world);


            for (_,(player, data, conn)) in keep_alive_data {
                let keep_alive_out = KeepAlivePacketOut::new_auto(data);
                let mut conn = conn.write().await;
                debug!("Sending keep alive packet to player: {:?}", player);
                if let Err(e) = conn.send_packet(keep_alive_out).await {
                    error!("Error sending keep alive packet: {:?}", e);
                }
                drop(conn);
            }

            // wait for a tick (1/20)
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    });*/
    
    // Start all systems (separate task)
    let all_systems = tokio::task::spawn(start_all_systems());
    let (con, systems) = tokio::try_join!(read_connections, all_systems)?;
    con?;
    systems?;
    
    // Kill all systems since we're done.
    kill_all_systems().await?;
    

    Ok(())
}

async fn read_connections(listener: TcpListener) -> Result<()> {
    loop {
        let (socket, addy) = listener.accept().await?;
        // show a line of 100 dashes
        trace!("{}", "-".repeat(100));
        debug!("Accepted connection from: {:?}", socket.peer_addr()?);

        tokio::task::spawn(
            async {
                if let Err(e) = ferrumc_net::init_connection(socket).await {
                    error!("Error handling connection: {:?}", e);
                }
            }
                .instrument(info_span!("handle_connection", %addy)),
        );
    }
}

/// Handles the setup of the server
///
/// If the server is running in a CI environment, it will set the log level to info
///
/// Returns True if the server should exit after setup
///
/// Runs [setup::setup] if the server needs setting up
async fn handle_setup() -> Result<bool> {
    let args = Cli::parse();

    // This env var will be present if the server is running in a CI environment
    // This will lead to set up not running, but we just need to check for compilation success, not actual functionality
    if env::var("GITHUB_ACTIONS").is_ok() {
        env::set_var("RUST_LOG", "info");
        Ok(false)
        // If the setup flag is passed, run the setup regardless of the config file
    } else if args.setup {
        setup::setup().await?;
        return Ok(true);
        // Check if the config file exists already and run the setup if it doesn't
    } else {
        // Get the path to the current executable
        let exe = std::env::current_exe()?;
        // This should be the directory the executable is in.
        // This should always work but if it doesn't, we'll just return an error
        let dir = exe.parent();
        match dir {
            Some(dir) => {
                let config_path = dir.join("config.toml");
                if !config_path.exists() {
                    setup::setup().await?;
                }
                Ok(false)
            }
            None => {
                error!("Failed to get the directory of the executable. Exiting...");
                return Ok(true);
            }
        }
    }
}

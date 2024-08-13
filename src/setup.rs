use std::env::current_exe;

use crate::utils::error::Error;
use tokio::fs;
use tracing::info;

/// Handles the setup of the server
///
/// This function is called when the server is started with the `--setup` flag or when the server is run for the first time
///
/// This function will create the necessary files and directories for the server to run. Also generates a default config file
pub(crate) async fn setup() -> Result<(), Error> {
    info!("Creating files...");
    let exe = current_exe()?;
    let dir = exe.parent().unwrap();
    fs::write(dir.join("config.toml"), BASE_CONFIG.as_bytes()).await?;
    fs::create_dir(dir.join("logs")).await?;
    fs::create_dir(dir.join("plugins")).await?;
    fs::write(
        dir.join("plugins").join("README.txt"),
        "Unfortunately plugins are not yet available",
    )
    .await?;

    info!("Files setup successfully!");
    Ok(())
}

/// The default configuration file
/// Not using ServerConfig::default(), since it doesn't have documentation on the usage of each field.
static BASE_CONFIG: &str = r#"
# The network address to bind to. Usually just 0.0.0.0 or 127.0.0.1 if you don't want to expose the server to the internet.
host = "0.0.0.0"
# The port to bind to. Default is 25565.
port = 25565
# The message displayed in the server list.
motd = "A Minecraft Server written in Rust"
# The maximum number of players that can be connected at once.
max_players = 20
# How many network updates to process per second per user. 0 means no limit.
# This is the number of times per second the server will send updates to the client.
# Having this too low will cause noticable lag for clients but may improve server performance.
network_tick_rate = 0
# The default world name. You can switch between mutliple worlds by changing this value.
world = "world"

[database]
# The path to the database folder. Generally you don't need to change this.
path = "data"
# The port the database runs on. This number should be changed if it conflicts with another program.
port = 29325
# The mode the database should run in. Options are "file" and "memory". "file" is recommended for persistence.
# Memory mode will lose all data when the server is stopped and will have higher RAM use but will be noticeably faster.
mode = "file"
"#;

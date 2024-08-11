use std::env;
use std::env::current_exe;

use include_flate::flate;
use tokio::fs;
use tracing::info;

use crate::utils::error::Error;

flate!(pub static BASE_CONFIG: [u8] from "config.toml");

/// Handles the setup of the server
///
/// This function is called when the server is started with the `--setup` flag or when the server is run for the first time
///
/// This function will create the necessary files and directories for the server to run. Also generates a default config file
pub(crate) async fn setup() -> Result<(), Error> {
    info!("Creating files...");
    let exe = current_exe()?;
    let dir = exe.parent().unwrap();
    fs::write(dir.join("../.etc/config.toml"), BASE_CONFIG.to_vec()).await?;
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

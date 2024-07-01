use tokio::fs;
use std::env::current_exe;
use ferrumc_utils::error::Error;
use spinners::{Spinner, Spinners};
use include_flate::flate;
use tracing::{info};

flate!(pub static BASE_CONFIG: [u8] from "config.toml");

/// Handles the setup of the server
/// 
/// This function is called when the server is started with the `--setup` flag or when the server is run for the first time
/// 
/// This function will create the necessary files and directories for the server to run. Also generates a default config file
pub(crate) async fn setup() -> Result<(), Error> {
    // Create a spinner to show the user that the server is setting up.
    // Will be near-instant if its just creating files and directories but if we ever need to do io heavy setup, it will be useful
    let mut sp = Spinner::new(Spinners::Dots, "Setting up files...".into());
    let exe = current_exe()?;
    let dir = exe.parent().unwrap();
    fs::write(dir.join("config.toml"), BASE_CONFIG.to_vec()).await?;
    fs::create_dir(dir.join("logs")).await?;
    fs::create_dir(dir.join("data")).await?;
    fs::create_dir(dir.join("plugins")).await?;
    fs::write(dir.join("plugins").join("README.txt"), "Unfortunately plugins are not yet available").await?;
    fs::create_dir(dir.join("worlds")).await?;
    sp.stop();
    // This is a stupid hack but the spinner doesn't clear the screen after, so we just write 36 backspaces to clear the line
    for _ in 0..36 {
        print!("{}", 8u8 as char)
    }
    info!("Files setup successfully!");
    Ok(())
}
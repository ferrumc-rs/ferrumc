use tokio::fs;
use std::env::current_exe;
use ferrumc_utils::error::Error;
use spinners::{Spinner, Spinners};
use include_flate::flate;

flate!(pub static BASE_CONFIG: [u8] from "config.toml");

pub(crate) async fn setup() -> Result<(), Error> {
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
    for _ in 0..36 {
        print!("{}", 8u8 as char)
    }
    println!("Files setup successfully!");
    Ok(())
}
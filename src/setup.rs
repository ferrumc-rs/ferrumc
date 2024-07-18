use std::{env, thread};
use std::collections::HashMap;
use std::env::consts::OS;
use std::env::current_exe;
use std::process::Stdio;

use include_flate::flate;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::process::Command;
use tracing::{error, info};

use ferrumc_utils::config::get_global_config;
use ferrumc_utils::error::Error;

flate!(pub static BASE_CONFIG: [u8] from "config.toml");

/// Handles the setup of the server
///
/// This function is called when the server is started with the `--setup` flag or when the server is run for the first time
///
/// This function will create the necessary files and directories for the server to run. Also generates a default config file
pub(crate) async fn setup() -> Result<(), Error> {
    println!("> Creating files...");
    let exe = current_exe()?;
    let dir = exe.parent().unwrap();
    fs::write(dir.join("config.toml"), BASE_CONFIG.to_vec()).await?;
    fs::create_dir(dir.join("logs")).await?;
    fs::create_dir(dir.join("plugins")).await?;
    fs::write(
        dir.join("plugins").join("README.txt"),
        "Unfortunately plugins are not yet available",
    )
    .await?;

    // Match statements are for cowards

    let (url, mut executable_name) = match OS {
        "windows" => (
            "https://github.com/surrealdb/surrealdb/releases/download/v1.5.4/surreal-v1.5.4.windows-amd64.exe", "surreal.exe"),
        "macos" => (
            "https://github.com/surrealdb/surrealdb/releases/download/v1.5.4/surreal-v1.5.4.darwin-amd64.tgz", "surreal.tgz"),
        "linux" => (
            "https://github.com/surrealdb/surrealdb/releases/download/v1.5.4/surreal-v1.5.4.linux-amd64.tgz", "surreal.tgz"),
        _ => {
            error!("Unsupported OS");
            return Err(Error::Generic("Unsupported OS".to_string()));
        }
    };
    println!("> Downloading database...");
    if which::which("curl").is_ok() {
        let command_result = Command::new("curl")
            .arg("-L")
            .arg("-o")
            .arg(executable_name)
            .arg(url)
            .stdout(Stdio::null())
            .spawn()
            .unwrap()
            .wait_with_output()
            .await?;
        if command_result.status.success() {
            info!("Downloaded surreal successfully!");
        } else {
            error!("Failed to download surreal");
            error!("{}", String::from_utf8_lossy(&command_result.stderr));
            return Err(Error::Generic("Failed to download surreal".to_string()));
        }
    } else if which::which("wget").is_ok() {
        let command_result = Command::new("wget")
            .arg("-O")
            .arg(executable_name)
            .arg(url)
            .stdout(Stdio::null())
            .spawn()
            .unwrap()
            .wait_with_output()
            .await?;
        if command_result.status.success() {
            info!("Downloaded surreal successfully!");
        } else {
            error!("Failed to download surreal");
            error!("{}", String::from_utf8_lossy(&command_result.stderr));
            return Err(Error::Generic("Failed to download surreal".to_string()));
        }
    } else if which::which("powershell").is_ok() {
        let command_result = Command::new("powershell")
            .arg("-Command")
            .arg(format!(
                "Invoke-WebRequest -Uri {} -OutFile {}",
                url, executable_name
            ))
            .stdout(Stdio::null())
            .spawn()
            .unwrap()
            .wait_with_output()
            .await?;
        if command_result.status.success() {
            info!("Downloaded surreal successfully!");
        } else {
            error!("Failed to download surreal");
            error!("{}", String::from_utf8_lossy(&command_result.stderr));
            return Err(Error::Generic("Failed to download surreal".to_string()));
        }
    } else {
        error!("No download tool found. Please install curl, wget, or powershell");
        return Err(Error::Generic("No download tool found".to_string()));
    };

    if OS != "windows" {
        println!("> Unpacking tarball database...");
        let command_result = Command::new("tar")
            .arg("-xvf")
            .arg(executable_name)
            .spawn()
            .unwrap()
            .wait_with_output()
            .await?;
        if command_result.status.success() {
            info!("Extracted surreal successfully!");
        } else {
            error!("Failed to extract surreal");
            error!("{}", String::from_utf8_lossy(&command_result.stderr));
            return Err(Error::Generic("Failed to extract surreal".to_string()));
        }
        fs::remove_file(executable_name).await?;
        executable_name = "surreal";
    }
    println!("> Setting up database...");
    let envs = HashMap::from([
        (
            "SURREAL_PATH",
            format!("file:{}", get_global_config().database.path),
        ),
        ("SURREAL_BIND", "127.0.0.1:0".to_string()),
        ("SURREAL_LOG_LEVEL", "info".to_string()),
        ("SURREAL_NO_BANNER", "true".to_string()),
        ("SURREAL_USER", "ferrumc".to_string()),
        ("SURREAL_PASS", "ferrumc".to_string()),
    ]);

    let mut surreal_setup_process = Command::new(executable_name)
        .arg("start")
        .envs(envs)
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    // Why the fuck does surrealdb use stderr by default???
    let mut stderr = BufReader::new(surreal_setup_process.stderr.take().unwrap());

    loop {
        let mut output = String::new();
        stderr.read_line(&mut output).await?;
        if output.contains("Started web server on") {
            info!("Surreal setup successfully!");
            break;
        }
    }

    surreal_setup_process.kill().await.unwrap();
    println!("Files setup successfully!");
    Ok(())
}

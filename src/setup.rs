use std::{env, thread};
use std::collections::HashMap;
use std::env::consts::OS;
use std::env::current_exe;
use std::process::Stdio;

use include_flate::flate;
use spinners::{Spinner, Spinners};
use surrealdb::engine::remote::http::Http;
use surrealdb::opt::auth::Root;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::process::Command;
use tracing::info;

use ferrumc_utils::config::get_global_config;
use ferrumc_utils::error::Error;

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
    fs::create_dir(dir.join("plugins")).await?;
    fs::write(
        dir.join("plugins").join("README.txt"),
        "Unfortunately plugins are not yet available",
    )
    .await?;
    match OS {
        "windows" => {
            let exe_data = reqwest::get("https://github.com/surrealdb/surrealdb/releases/download/v1.5.4/surreal-v1.5.4.windows-amd64.exe")
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap()
                .to_vec();
            fs::write(dir.join("surreal.exe"), exe_data).await?;
        }
        "linux" => {
            let exe_data = reqwest::get("
                https://github.com/surrealdb/surrealdb/releases/download/v1.5.4/surreal-v1.5.4.linux-amd64.tgz")
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap()
                .to_vec();
            fs::write(dir.join("surreal.tgz"), exe_data).await?;
            Command::new("tar")
                .arg("-xvf")
                .arg("surreal.tgz")
                .current_dir(dir)
                .output()
                .await?;
            tokio::fs::remove_file(dir.join("surreal.tgz")).await?;
        }
        "macos" => {
            let exe_data = reqwest::get("https://github.com/surrealdb/surrealdb/releases/download/v1.5.4/surreal-v1.5.4.darwin-amd64.tgz")
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap()
                .to_vec();
            fs::write(dir.join("surreal.tgz"), exe_data).await?;
            Command::new("tar")
                .arg("-xvf")
                .arg("surreal.tgz")
                .current_dir(dir)
                .output()
                .await?;
            tokio::fs::remove_file(dir.join("surreal.tgz")).await?;
        }
        _ => {
            sp.stop();
            return Err(Error::Generic("Unsupported OS".to_string()));
        }
    }

    let db_port = get_global_config().database.port;

    let envs = HashMap::from([
        (
            "SURREAL_PATH",
            format!("file:{}", get_global_config().database.path),
        ),
        ("SURREAL_BIND", format!("127.0.0.1:{}", db_port.to_string())),
        ("SURREAL_LOG_LEVEL", "info".to_string()),
        ("SURREAL_NO_BANNER", "true".to_string()),
        ("SURREAL_USER", "ferrumc".to_string()),
        ("SURREAL_PASS", "ferrumc".to_string()),
    ]);

    let mut surreal_setup_process = Command::new("surreal.exe")
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

    let db = surrealdb::Surreal::new::<Http>(format!("127.0.0.1:{}", db_port))
        .await
        .unwrap();
    db.signin(Root {
        username: "ferrumc",
        password: "ferrumc",
    })
    .await
    .unwrap();
    db.use_ns("ferrumc").await.unwrap();
    db.use_db(get_global_config().world.clone()).await.unwrap();

    db.query("DEFINE TABLE chunks")
        .query("DEFINE TABLE entities")
        .await
        .unwrap();

    surreal_setup_process.kill().await.unwrap();

    sp.stop();
    // This is a stupid hack but the spinner doesn't clear the screen after, so we just write 36 backspaces to clear the line
    for _ in 0..36 {
        print!("{}", 8u8 as char)
    }
    info!("Files setup successfully!");
    Ok(())
}

use std::collections::HashMap;
use std::env::consts::OS;
use std::process::{exit, Stdio};

use surrealdb::engine::remote::http::{Client, Http};
use surrealdb::opt::auth::Root;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::oneshot::error::TryRecvError;
use tokio::sync::oneshot::Sender;
use tracing::{error, info, warn};

use crate::utils::config::get_global_config;
use crate::utils::error::Error;

pub enum DatabaseState {
    NotStarted,
    Starting,
    Running,
}

pub struct Database {
    pub conn: surrealdb::Surreal<Client>,
    pub state: DatabaseState,
}

pub async fn start_database() -> Result<(Database, Sender<bool>), Error> {
    let store_path = if get_global_config().database.mode == "file" {
        format!("file:{}", get_global_config().database.path)
    } else {
        "memory".to_string()
    };
    let envs: HashMap<&str, String> = HashMap::from([
        (
            "SURREAL_BIND",
            format!(
                "127.0.0.1:{}",
                get_global_config().database.port.to_string()
            ),
        ),
        ("SURREAL_PATH", store_path),
        ("SURREAL_LOG_LEVEL", "info".to_string()),
        ("SURREAL_NO_BANNER", "true".to_string()),
    ]);

    let mut executable_name = match OS {
        "windows" => ".\\surreal.exe",
        "macos" => "./surreal",
        "linux" => "./surreal",
        _ => {
            return Err(Error::Generic("Unsupported OS".to_string()));
        }
    };

    if !tokio::fs::try_exists(executable_name).await.unwrap() {
        if which::which("surreal").is_ok() {
            warn!("Using system surreal executable. This may be outdated.");
            executable_name = "surreal";
        } else {
            error!("Surreal executable not found at {}", executable_name);
            exit(1);
        }
    }

    // start memory -A --auth --user root --pass root
    let mut surreal_setup_process = tokio::process::Command::new(executable_name)
        .arg("start")
        .args(&["-A", "--auth", "--user", "ferrumc", "--pass", "ferrumc"])
        //.args(&["-A", "--auth"])
        .envs(envs)
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let db =
        surrealdb::Surreal::new::<Http>(format!("127.0.0.1:{}", get_global_config().database.port))
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

    let query = "\
        DEFINE TABLE chunks;
        DEFINE TABLE entities;";
    db.query(query).await.unwrap();

    let (tx, mut rx) = tokio::sync::oneshot::channel();
    tokio::task::spawn(async move {
        // Why the fuck does surrealdb use stderr by default???
        let mut stderr = BufReader::new(surreal_setup_process.stderr.take().unwrap());
        let mut output = String::new();
        loop {
            stderr.read_line(&mut output).await.unwrap();
            if !output.is_empty() {
                print!("{output}");
                output.clear();
            }

            let res = rx.try_recv();

            match res {
                Ok(true) => {
                    info!("Recieved kill for child");
                    surreal_setup_process.kill().await.unwrap()
                }
                Err(TryRecvError::Closed) => {
                    info!("Other pipe is closed");
                    surreal_setup_process.kill().await.unwrap()
                }
                Err(TryRecvError::Empty) => {}
                _ => {}
            }
        }
    });

    Ok((
        Database {
            conn: db,
            state: DatabaseState::NotStarted,
        },
        tx,
    ))
}

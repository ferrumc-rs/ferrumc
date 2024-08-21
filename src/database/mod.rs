use std::env;
use std::ops::Deref;
use std::path::PathBuf;

use tokio::fs;
use tracing::{debug, info};

use crate::utils::config::get_global_config;
use crate::utils::error::Error;

pub mod chunks;

pub struct Database {
    pub db: sled::Db,
}

pub async fn start_database() -> Result<Database, Error> {
    debug!("Starting database");

    let root = if env::var("FERRUMC_ROOT").is_ok() {
        PathBuf::from(env::var("FERRUMC_ROOT").unwrap())
    } else {
        PathBuf::from(
            env::current_exe()
                .unwrap()
                .parent()
                .ok_or(Error::Generic("Failed to get exe directory".to_string()))?,
        )
    };

    let world = get_global_config().world.clone();
    let world_path = root.join("data").join(world);
    debug!("Database path: {:?}", world_path.to_str().unwrap());

    if !fs::try_exists(&world_path).await? {
        fs::create_dir_all(&world_path).await?;
    }

    let database = sled::open(world_path)
        .map_err(|e| Error::DatabaseError(format!("Failed to open database: {}", e)))
        .unwrap();

    info!("Database started");

    Ok(Database { db: database })
}

use crate::utils::config::get_global_config;
use crate::utils::error::Error;
use tokio::fs;
use tracing::{debug, info};

pub mod chunks;

pub struct Database {
    pub db: sled::Db,
}

pub async fn start_database() -> Result<Database, Error> {
    debug!("Starting database");

    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .ok_or(Error::Generic("Failed to get exe directory".to_string()))?;

    let world = get_global_config().world.clone();
    let world_path = exe_dir.join("data").join(world);

    if !fs::try_exists(&world_path).await? {
        fs::create_dir_all(&world_path).await?;
    }

    let database = sled::open(world_path)
        .map_err(|e| Error::DatabaseError(format!("Failed to open database: {}", e)))?;

    info!("Database started");

    Ok(Database { db: database })
}

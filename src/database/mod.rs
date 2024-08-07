use tokio::fs;

use crate::utils::config::get_global_config;
use crate::utils::error::Error;

pub mod chunks;

pub struct Database {
    pub db: sled::Db,
}

pub async fn start_database() -> Result<Database, Error> {
    if !fs::try_exists(format!("data/{}", get_global_config().world)).await? {
        fs::create_dir(format!("data/{}", get_global_config().world)).await?;
    }
    let database = sled::open(format!("data/{}", get_global_config().world))
        .map_err(|e| Error::DatabaseError(format!("Failed to open database: {}", e)))?;
    Ok(Database { db: database })
}

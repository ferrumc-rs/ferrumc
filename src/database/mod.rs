use redb::backends::FileBackend;
use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tracing::{debug, info};

use crate::utils::config::get_global_config;
use crate::utils::error::Error;

use redb::Database as RedbDatabase;

pub mod chunks;

pub struct Database {
    pub db: Arc<RedbDatabase>,
}

pub async fn start_database() -> Result<Database, Error> {
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

    debug!("Opening database at {:?}", world_path);

    if !fs::try_exists(&world_path).await? {
        fs::create_dir_all(&world_path).await?;
    }

    let file = File::options()
        .create(true)
        .write(true)
        .read(true)
        .open(world_path.join("test"))?;

    let cache_size = get_global_config().database.cache_size;

    let database = redb::Database::builder()
        .set_cache_size((cache_size * 1024) as usize)
        .create_with_backend(FileBackend::new(file).expect("Failed to create backend"))
        .unwrap();

    info!("Database started");

    Ok(Database {
        db: Arc::new(database),
    })
}

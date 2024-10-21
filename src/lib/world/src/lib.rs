pub mod errors;
mod importing;

use std::path::{Path, PathBuf};
use std::process::exit;
use tokio::fs::create_dir_all;
use ferrumc_storage::{Compressor, DatabaseBackend};
use ferrumc_config::get_global_config;
use tracing::{error, warn};
use crate::errors::WorldError;

pub struct World {
    storage_backend: Box<dyn DatabaseBackend>,
    compressor: Box<dyn Compressor>,
    // TODO: Cache
}

async fn check_config_validity() -> Result<(), WorldError> {
    
    // We don't actually check if the import path is valid here since that would brick a server
    // if the world is imported then deleted after the server starts. Those checks are handled in
    // the importing logic.
    
    let config = get_global_config();
    if config.database.backend.is_empty() {
        error!("No backend specified. Please set the backend in the configuration file.");
        return Err(WorldError::InvalidBackend(config.database.backend.clone()));
    }
    if !Path::new(&config.database.world_path).exists() {
        warn!("World path does not exist. Attempting to create it.");
        if create_dir_all(&config.database.world_path).await.is_err() {
            error!("Could not create world path: {}", config.database.world_path);
            return Err(WorldError::InvalidWorldPath(config.database.world_path.clone()));
        }
    }
    if Path::new(&config.database.world_path).is_file() {
        error!("World path is a file. Please set the world path to a directory.");
        return Err(WorldError::InvalidWorldPath(config.database.world_path.clone()));
    }
    if let Err(e) = Path::new(&config.database.world_path).read_dir() {
        error!("Could not read world path: {}", e);
        e.into()
    }
    
    if config.database.compression.is_empty() {
        error!("No compressor specified. Please set the compressor in the configuration file.");
        return Err(WorldError::InvalidCompressor(config.database.compression.clone()));
    }
    if config.database.import_path.is_empty() { 
        error!("No import path specified. Please set the import path in the configuration file.");
        return Err(WorldError::InvalidImportPath(config.database.import_path.clone()));
    }
    Ok(())
    
}


impl World {
    pub async fn new() -> Self {
        if let Err(e) = check_config_validity().await {
            error!("Fatal error in database config: {}", e);
            exit(1);
        }
        let backend_string = get_global_config().database.backend.clone().to_lowercase().trim();
        let backend_path = get_global_config().database.world_path.clone();
        let storage_backend: Result<Box<dyn DatabaseBackend>, WorldError> = match backend_string {
            "surrealkv" => {
                #[cfg(feature = "surrealkv")]
                match ferrumc_storage::backends::surrealkv::SurrealKVBackend::initialize(Some(PathBuf::from(backend_path))).await {
                    Ok(backend) => Ok(Box::new(backend)),
                    Err(e) => Err(WorldError::InvalidBackend(e.to_string()))
                }
                #[cfg(not(feature = "surrealkv"))]
                {
                    error!("SurrealKV backend is not enabled. Please enable the 'surrealkv' feature in the Cargo.toml file.");
                    exit(1);
                }
            },
            "sled" => {
                #[cfg(feature = "sled")]
                match ferrumc_storage::backends::sled::SledBackend::initialize(Some(PathBuf::from(backend_path))).await {
                    Ok(backend) => Ok(Box::new(backend)),
                    Err(e) => Err(WorldError::InvalidBackend(e.to_string()))
                }
                #[cfg(not(feature = "sled"))]
                {
                    error!("Sled backend is not enabled. Please enable the 'sled' feature in the Cargo.toml file.");
                    exit(1);
                }
            },
            "rocksdb" => {
                #[cfg(feature = "rocksdb")]
                match ferrumc_storage::backends::rocksdb::RocksDBBackend::initialize(Some(PathBuf::from(backend_path))).await {
                    Ok(backend) => Ok(Box::new(backend)),
                    Err(e) => Err(WorldError::InvalidBackend(e.to_string()))
                }
                #[cfg(not(feature = "rocksdb"))]
                {
                    error!("RocksDB backend is not enabled. Please enable the 'rocksdb' feature in the Cargo.toml file.");
                    exit(1);
                }
            },
            "redb" => {
                #[cfg(feature = "redb")]
                match ferrumc_storage::backends::redb::RedbBackend::initialize(Some(PathBuf::from(backend_path))).await {
                    Ok(backend) => Ok(Box::new(backend)),
                    Err(e) => Err(WorldError::InvalidBackend(e.to_string()))
                }
                #[cfg(not(feature = "redb"))]
                {
                    error!("Redb backend is not enabled. Please enable the 'redb' feature in the Cargo.toml file.");
                    exit(1);
                }
            },
            _ => {
                error!("Invalid storage backend: {}", get_global_config().database.backend);
                exit(1);
            }
        };
        let storage_backend = if let Ok(backend) = storage_backend {
            backend
        } else {
            exit(1);
        };
        
        
        
    }
}

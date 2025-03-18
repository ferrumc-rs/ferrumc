#![feature(hash_raw_entry)]
#![feature(hasher_prefixfree_extras)]
#![feature(int_roundings)]

pub mod chunk_format;
mod db_functions;
pub mod edits;
pub mod errors;
mod importing;
pub mod vanilla_chunk_format;

use crate::chunk_format::Chunk;
use crate::errors::WorldError;
use deepsize::DeepSizeOf;
use ferrumc_config::statics::get_global_config;
use ferrumc_general_purpose::paths::get_root_path;
use ferrumc_storage::compressors::Compressor;
use ferrumc_storage::lmdb::LmdbBackend;
use moka::future::{Cache, FutureExt};
use moka::notification::ListenerFuture;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::Duration;
use tokio::fs::create_dir_all;
use tracing::{error, info, trace, warn};

#[derive(Clone)]
pub struct World {
    storage_backend: LmdbBackend,
    compressor: Compressor,
    cache: Cache<(i32, i32, String), Chunk>,
}

async fn check_config_validity() -> Result<(), WorldError> {
    // We don't actually check if the import path is valid here since that would brick a server
    // if the world is imported then deleted after the server starts. Those checks are handled in
    // the importing logic.

    let config = get_global_config();
    let db_path = get_root_path().join(&config.database.db_path);

    if config.database.map_size == 0 {
        error!("Map size is set to 0. Please set the map size in the configuration file.");
        return Err(WorldError::InvalidMapSize(config.database.map_size));
    }
    if !Path::new(&db_path).exists() {
        warn!("World path does not exist. Attempting to create it.");
        if create_dir_all(&db_path).await.is_err() {
            error!("Could not create world path: {}", db_path.display());
            return Err(WorldError::InvalidWorldPath(
                db_path.to_string_lossy().to_string(),
            ));
        }
    }
    if Path::new(&db_path).is_file() {
        error!("World path is a file. Please set the world path to a directory.");
        return Err(WorldError::InvalidWorldPath(
            db_path.to_string_lossy().to_string(),
        ));
    }
    if let Err(e) = Path::new(&db_path).read_dir() {
        error!("Could not read world path: {}", e);
        return Err(WorldError::InvalidWorldPath(
            db_path.to_string_lossy().to_string(),
        ));
    }

    if config.database.compression.is_empty() {
        error!("No compressor specified. Please set the compressor in the configuration file.");
        return Err(WorldError::InvalidCompressor(
            config.database.compression.clone(),
        ));
    }

    // Check if doing map_size * 1024^3 would overflow usize. You probably don't need a database
    // that's 18 exabytes anyway.
    if config.database.map_size as usize > ((usize::MAX / 1024) / 1024) / 1024 {
        error!(
            "Map size is too large, this would exceed the usize limit. You probably don't need a \
        database this big anyway. Are you sure you have set the map size in GB, not bytes?"
        );
        return Err(WorldError::InvalidMapSize(config.database.map_size));
    }
    Ok(())
}

impl World {
    /// Creates a new world instance.
    ///
    /// You'd probably want to call this at the start of your program. And then use the returned
    /// in a state struct or something.
    pub async fn new() -> Self {
        if let Err(e) = check_config_validity().await {
            error!("Fatal error in database config: {}", e);
            exit(1);
        }
        // Clones are kinda ok here since this is only run once at startup.
        let mut backend_path = PathBuf::from(get_global_config().database.db_path.clone());
        if backend_path.is_relative() {
            backend_path = get_root_path().join(backend_path);
        }
        let storage_backend = LmdbBackend::initialize(Some(backend_path))
            .await
            .expect("Failed to initialize database");

        let compressor_string = get_global_config().database.compression.trim();

        info!("Using {} compression algorithm", compressor_string);

        let compression_algo = match compressor_string.to_lowercase().as_str() {
            "zstd" => Compressor::create(
                ferrumc_storage::compressors::CompressorType::Zstd,
                get_global_config().database.compression_level as u32,
            ),
            "brotli" => Compressor::create(
                ferrumc_storage::compressors::CompressorType::Brotli,
                get_global_config().database.compression_level as u32,
            ),
            "deflate" => Compressor::create(
                ferrumc_storage::compressors::CompressorType::Deflate,
                get_global_config().database.compression_level as u32,
            ),
            "gzip" => Compressor::create(
                ferrumc_storage::compressors::CompressorType::Gzip,
                get_global_config().database.compression_level as u32,
            ),
            "zlib" => Compressor::create(
                ferrumc_storage::compressors::CompressorType::Zlib,
                get_global_config().database.compression_level as u32,
            ),
            _ => {
                error!(
                    "Invalid compression algorithm: {}",
                    get_global_config().database.compression
                );
                exit(1);
            }
        };

        if get_global_config().database.cache_ttl != 0
            && get_global_config().database.cache_capacity == 0
        {
            error!("Cache TTL and capacity must both be set to 0 or both be set to a value greater than 0.");
            exit(1);
        }

        let eviction_listener = move |key, _, cause| -> ListenerFuture {
            async move {
                trace!("Evicting key: {:?}, cause: {:?}", key, cause);
            }
            .boxed()
        };

        let cache = Cache::builder()
            .async_eviction_listener(eviction_listener)
            .weigher(|_k, v: &Chunk| v.deep_size_of() as u32)
            .time_to_live(Duration::from_secs(get_global_config().database.cache_ttl))
            .max_capacity(get_global_config().database.cache_capacity * 1024)
            .build();

        World {
            storage_backend,
            compressor: compression_algo,
            cache,
        }
    }
}

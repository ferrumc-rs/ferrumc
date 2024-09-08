use deepsize::DeepSizeOf;
use futures::FutureExt;
use moka::notification::{ListenerFuture, RemovalCause};
use rocksdb::{Cache, Options, DB};
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tracing::{debug, info};

use crate::utils::config::get_global_config;
use crate::utils::error::Error;

use crate::world::chunkformat::Chunk;

pub mod chunks;

pub struct Database {
    db: Arc<DB>,
    cache: Arc<moka::future::Cache<u64, Chunk>>,
}

fn evict_chunk(_key: Arc<u64>, value: Chunk, cause: RemovalCause) -> ListenerFuture {
    async move {
        if cause == RemovalCause::Expired {
            info!("Evicting chunk: {}, {}", value.x_pos, value.z_pos);
        }
    }
    .boxed()
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

    let mut options = Options::default();
    options.create_if_missing(true);
    options.create_missing_column_families(true);
    options.enable_statistics();
    options.increase_parallelism(num_cpus::get() as i32);
    options.set_db_log_dir(root.join("logs"));
    options.set_compression_type(rocksdb::DBCompressionType::Zstd);
    options.set_compression_options_parallel_threads(num_cpus::get() as i32);
    let mut block_based_options = rocksdb::BlockBasedOptions::default();
    let cache = Cache::new_lru_cache(0);
    block_based_options.set_block_cache(&cache);
    options.set_block_based_table_factory(&block_based_options);

    let database = DB::open_cf(&options, world_path, &["chunks", "entities"])
        .expect("Failed to open database");

    info!("Database started");

    info!("Initializing cache");

    let cache = moka::future::Cache::builder()
        .async_eviction_listener(evict_chunk)
        .weigher(|_, v| v.deep_size_of() as u32)
        .eviction_policy(moka::policy::EvictionPolicy::tiny_lfu())
        .max_capacity(get_global_config().database.cache_size as u64 * 1024)
        .build();

    Ok(Database {
        db: Arc::new(database),
        cache: Arc::new(cache),
    })
}

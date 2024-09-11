use deepsize::DeepSizeOf;
use futures::FutureExt;
use moka::notification::{ListenerFuture, RemovalCause};
use rocksdb::{Cache, ColumnFamilyDescriptor, Options, DB};
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
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
            debug!("Evicting chunk: {}, {}", value.x_pos, value.z_pos);
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

    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.create_missing_column_families(true);
    opts.increase_parallelism(num_cpus::get() as i32);
    opts.set_db_log_dir(root.join("logs"));
    opts.set_compression_type(rocksdb::DBCompressionType::Lz4);
    // opts.set_compression_options_parallel_threads(num_cpus::get() as i32);
    let cache = Cache::new_lru_cache(512 * 1024); // 1MB cache
    {
        let mut bb_opts = rocksdb::BlockBasedOptions::default();
        bb_opts.set_block_cache(&cache);
        bb_opts.set_checksum_type(rocksdb::ChecksumType::NoChecksum);
        opts.set_block_based_table_factory(&bb_opts);
    }
    opts.set_row_cache(&cache);
    opts.set_paranoid_checks(false);
    opts.set_disable_auto_compactions(true);
    opts.set_compaction_readahead_size(0);
    opts.set_allow_mmap_reads(true);
    opts.set_allow_mmap_writes(true);

    let cf_names = vec!["chunks", "entities"];
    let cf_descriptors = cf_names
        .into_iter()
        .map(|name| {
            ColumnFamilyDescriptor::new(name, opts.clone())
        })
        .collect::<Vec<_>>();

    let database = DB::open_cf_descriptors(&opts, world_path, cf_descriptors)
        .expect("Failed to open database");

    info!("Database started");

    info!("Initializing cache");

    let cache = moka::future::Cache::builder()
        .async_eviction_listener(evict_chunk)
        .weigher(|_, v| v.deep_size_of() as u32)
        .eviction_policy(moka::policy::EvictionPolicy::tiny_lfu())
        .max_capacity(get_global_config().database.cache_size as u64 * 1024)
        .time_to_live(Duration::from_millis(1000))
        .build();

    Ok(Database {
        db: Arc::new(database),
        cache: Arc::new(cache),
    })
}


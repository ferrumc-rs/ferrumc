use byteorder::LE;
use chunks::Zstd;
use deepsize::DeepSizeOf;
use futures::FutureExt;
use heed::types::U64;
use heed::{Env as LMDBDatabase, EnvFlags, EnvOpenOptions};
use moka::notification::{ListenerFuture, RemovalCause};
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use tokio::fs;
use tracing::{debug, info, trace};

use crate::utils::config::get_global_config;
use crate::utils::error::Error;

use crate::world::chunk_format::Chunk;
pub mod chunks;

// MDBX constants
const LMDB_PAGE_SIZE: usize = 50 * 1024usize.pow(3); // 50GiB
const LMDB_MAX_DBS: u32 = 10;

// Database threadpool
static LMDB_THREADPOOL: OnceLock<ThreadPool> = OnceLock::new();

/// Global database structure
///
/// Internally contain a handle to the persistent database and a
/// cache for all in-memory updates
pub struct Database {
    db: LMDBDatabase,
    cache: Arc<moka::future::Cache<u64, Chunk>>,
}

fn evict_chunk(_key: Arc<u64>, value: Chunk, cause: RemovalCause) -> ListenerFuture {
    async move {
        if cause == RemovalCause::Expired {
            trace!(
                "Evicting chunk from cache: {}, {}",
                value.x_pos,
                value.z_pos
            );
        }
    }
    .boxed()
}

/// Start database
pub async fn start_database() -> Result<Database, Error> {
    // Parse root directory from environment variable
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

    // Obtain global config to locate which world folder to load
    let world = get_global_config().world.clone();
    let world_path = root.join("data").join(world);

    debug!("Opening database at {}", world_path.display());

    if !fs::try_exists(&world_path).await? {
        fs::create_dir_all(&world_path).await?;
    }

    // Database Options
    let mut opts = EnvOpenOptions::new();
    opts.max_readers(num_cpus::get() as u32)
        .map_size(LMDB_PAGE_SIZE)
        .max_dbs(LMDB_MAX_DBS);

    // Open database (This operation is safe as we assume no other process touched the database)
    let lmdb = unsafe {
        opts.flags(EnvFlags::empty())
            .open(&world_path)
            .expect("Unable to open LMDB environment located at {world_path:?}")
    };

    // Start database threadpool
    LMDB_THREADPOOL.get_or_init(|| {
        ThreadPoolBuilder::new().num_threads(num_cpus::get() / 2).build().unwrap()
    });
    
    // Check if database is built. Otherwise, initialize it
    let mut rw_tx = lmdb.write_txn().unwrap();
    if lmdb
        .open_database::<U64<LE>, Zstd<Chunk>>(&rw_tx, Some("chunks"))
        .unwrap()
        .is_none()
    {
        lmdb.create_database::<U64<LE>, Zstd<Chunk>>(&mut rw_tx, Some("chunks"))
            .expect("Unable to create database");
    }
    // `entities` table to be added, but needs the type to do so

    rw_tx.commit().unwrap();

    info!("Database started");

    info!("Initializing cache");

    // Initializing moka cache
    let cache = moka::future::Cache::builder()
        .async_eviction_listener(evict_chunk)
        .weigher(|_, v| v.deep_size_of() as u32)
        .eviction_policy(moka::policy::EvictionPolicy::tiny_lfu())
        .max_capacity(get_global_config().database.cache_size as u64 * 1024)
        .time_to_live(Duration::from_millis(1000))
        .build();

    Ok(Database {
        db: lmdb,
        cache: Arc::new(cache),
    })
}

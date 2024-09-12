use byteorder::LE;
use deepsize::DeepSizeOf;
use futures::FutureExt;
use heed::types::{Bytes, U64};
use heed::{Env as LMDBDatabase, Env, EnvFlags, EnvOpenOptions, MdbError};
use moka::notification::{ListenerFuture, RemovalCause};
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::env;
use std::future::Future;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, Mutex, OnceLock, RwLock};
use std::time::Duration;
use tokio::fs;
use tokio::sync::oneshot;
use tracing::{debug, info, trace, warn};

use crate::utils::config::get_global_config;
use crate::utils::error::Error;

use crate::world::chunk_format::Chunk;
pub mod chunks;
pub(crate) mod encoding;

const LMDB_MIN_PAGE_SIZE: usize = 1800 * 1024usize.pow(2); // 1800MB
const LMDB_PAGE_SIZE_INCREMENT: usize = 250 * 1024usize.pow(2); // 250MB
const LMDB_MAX_DBS: u32 = 10;

// Database threadpool
static LMDB_THREADPOOL: OnceLock<ThreadPool> = OnceLock::new();

// Global size
static LMDB_PAGE_SIZE: LazyLock<Arc<Mutex<usize>>> =
    LazyLock::new(|| Arc::new(Mutex::new(LMDB_MIN_PAGE_SIZE)));
static LMDB_READER_SYNC: LazyLock<Arc<RwLock<()>>> = LazyLock::new(|| Arc::new(RwLock::new(())));

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
        .map_size(LMDB_MIN_PAGE_SIZE)
        .max_dbs(LMDB_MAX_DBS);

    // Open database (This operation is safe as we assume no other process touched the database)
    let lmdb = unsafe {
        opts.flags(EnvFlags::WRITE_MAP | EnvFlags::NO_SYNC)
            .open(&world_path)
            .expect("Unable to open LMDB environment located at {world_path:?}")
    };

    // Start database threadpool
    LMDB_THREADPOOL.get_or_init(|| {
        ThreadPoolBuilder::new()
            .num_threads(num_cpus::get() / 2)
            .build()
            .unwrap()
    });

    // Check if database is built. Otherwise, initialize it
    let mut rw_tx = lmdb.write_txn()?;
    if lmdb
        // .open_database::<U64<LE>, Zstd<Chunk>>(&rw_tx, Some("chunks"))
        .open_database::<U64<LE>, Bytes>(&rw_tx, Some("chunks"))?
        .is_none()
    {
        lmdb.create_database::<U64<LE>, Bytes>(&mut rw_tx, Some("chunks"))
            .expect("Unable to create database");
    }
    // `entities` table to be added, but needs the type to do so

    rw_tx.commit()?;

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

/// LMDB will follow a linear growth as opposed to MDBX which
/// uses a geometric growth.
pub(super) fn new_page_size(old_size: usize) -> usize {
    old_size + LMDB_PAGE_SIZE_INCREMENT
}

/// Spawn a blocking task to interact with the database
/// This is used to prevent the database from being blocked
/// by a single thread
///
/// # Arguments
///
/// * `db` - The database environment
/// * `f` - The function to execute
///
/// # Returns
///
/// A future that resolves to the result of the function
pub(super) fn spawn_blocking_db<F, R>(
    db: Env,
    f: F,
) -> impl Future<Output = Result<Result<R, heed::Error>, oneshot::error::RecvError>>
where
    F: Fn() -> Result<R, heed::Error> + Send + 'static,
    R: Send + 'static + std::fmt::Debug,
{
    let (tx, res) = oneshot::channel::<Result<R, heed::Error>>();

    let pool = LMDB_THREADPOOL.get().unwrap();
    pool.spawn(move || {

        let read_lock = LMDB_READER_SYNC.read()
            .expect("Database RWLock has been poisoned. A thread should have crashed somewhere.");

        let mut res = f();
        if let Err(heed::Error::Mdb(MdbError::MapFull)) = res {

            warn!("Database page is full. Resizing...");

            drop(read_lock);

            let _resize_guard = LMDB_READER_SYNC.write()
                .expect("Database RWLock has been poisoned. A thread should have crashed somewhere.");

            let mut global_size_lock = LMDB_PAGE_SIZE.lock().unwrap();
            let old_size = *global_size_lock;
            *global_size_lock = new_page_size(old_size);
            unsafe { db.resize(*global_size_lock).expect("Unable to resize LMDB environment.") };

            tracing::info!("Successfully resized LMDB page from {} MiB to {} MiB", old_size / 1024usize.pow(2), *global_size_lock / 1024usize.pow(2));

            drop(global_size_lock);
            drop(_resize_guard);

            res = f();
        } else {
            drop(read_lock)
        }

        if tx.send(res).is_err() {
            warn!("A database task has been unable to send its result because the receiver at other end have closed.")
        }
    });

    res
}

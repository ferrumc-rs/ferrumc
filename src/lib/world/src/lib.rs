pub mod block_id;
pub mod chunk_format;
mod chunk_ops;
mod db_functions;
pub mod edit_batch;
pub mod errors;
mod importing;
pub mod section_ops;
pub mod vanilla_chunk_format;
mod world_ops;

use crate::chunk_format::Chunk;
use crate::errors::WorldError;
use bevy_math::IVec3;
use deepsize::DeepSizeOf;
use ferrumc_config::server_config::get_global_config;
use ferrumc_general_purpose::paths::get_root_path;
use ferrumc_storage::lmdb::LmdbBackend;
use moka::sync::Cache;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, trace, warn};

/// Converts a global block position to an index in a chunk section (0-4095).
pub fn to_index(pos: IVec3) -> usize {
    let x = (pos.x & 0xF) as usize;
    let y = (pos.y & 0xF) as usize;
    let z = (pos.z & 0xF) as usize;
    (y << 8) | (z << 4) | x
}

#[derive(Clone)]
pub struct World {
    storage_backend: LmdbBackend,
    cache: Cache<(i32, i32, String), Arc<Chunk>>,
}

fn check_config_validity() -> Result<(), WorldError> {
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
        if create_dir_all(&db_path).is_err() {
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
    pub fn new(backend_path: impl Into<PathBuf>) -> Self {
        if let Err(e) = check_config_validity() {
            error!("Fatal error in database config: {}", e);
            exit(1);
        }
        let mut backend_path = backend_path.into();
        // Clones are kinda ok here since this is only run once at startup.
        if backend_path.is_relative() {
            backend_path = get_root_path().join(backend_path);
        }
        let storage_backend =
            LmdbBackend::initialize(Some(backend_path)).expect("Failed to initialize database");

        if get_global_config().database.cache_ttl != 0
            && get_global_config().database.cache_capacity == 0
        {
            error!("Cache TTL and capacity must both be set to 0 or both be set to a value greater than 0.");
            exit(1);
        }

        let eviction_listener = move |key, _, cause| {
            trace!("Evicting key: {:?}, cause: {:?}", key, cause);
        };

        let cache = Cache::builder()
            .eviction_listener(eviction_listener)
            .weigher(|_k, v: &Arc<Chunk>| v.deep_size_of() as u32)
            .time_to_live(Duration::from_secs(get_global_config().database.cache_ttl))
            .max_capacity(get_global_config().database.cache_capacity * 1024)
            .build();

        World {
            storage_backend,
            cache,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_math::IVec2;

    #[test]
    #[ignore]
    fn dump_chunk() {
        let world = World::new(std::env::current_dir().unwrap().join("../../../world"));
        let chunk = world.load_chunk(IVec2::new(0, 0), "overworld").expect(
            "Failed to load chunk. If it's a bitcode error, chances are the chunk format \
             has changed since last generating a world so you'll need to regenerate",
        );
        let encoded = bitcode::encode(&chunk);
        std::fs::write("../../../.etc/raw_chunk.dat", encoded).unwrap();
    }

    #[test]
    fn test_to_index_basic() {
        // (x, y, z) = (0, 0, 0) should map to 0
        assert_eq!(to_index(IVec3::new(0, 0, 0)), 0);

        // (x, y, z) = (15, 15, 15) should map to the last index in a chunk section
        assert_eq!(to_index(IVec3::new(15, 15, 15)), 4095);

        // (x, y, z) = (1, 2, 3)
        let expected = (2 << 8) | (3 << 4) | 1;
        assert_eq!(to_index(IVec3::new(1, 2, 3)), expected);

        // Values outside 0-15 should be masked
        assert_eq!(to_index(IVec3::new(16, 16, 16)), 0);
        assert_eq!(to_index(IVec3::new(17, 18, 19)), (2 << 8) | (3 << 4) | 1);
    }
}

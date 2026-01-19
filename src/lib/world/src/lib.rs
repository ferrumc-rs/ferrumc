pub mod block_state_id;
pub mod chunk;
mod db_functions;
pub mod errors;
mod importing;
mod player;
pub mod pos;
pub mod vanilla_chunk_format;

use crate::chunk::Chunk;
use crate::errors::WorldError;
use crate::pos::ChunkPos;
use dashmap::DashMap;
use ferrumc_config::server_config::get_global_config;
use ferrumc_general_purpose::paths::get_root_path;
use ferrumc_storage::lmdb::LmdbBackend;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::exit;
use tracing::{error, warn};
use wyhash::WyHasherBuilder;

type ChunkCache = DashMap<(ChunkPos, String), Chunk, WyHasherBuilder>;

pub type MutChunk<'a> = dashmap::mapref::one::RefMut<'a, (ChunkPos, String), Chunk>;
pub type RefChunk<'a> = dashmap::mapref::one::Ref<'a, (ChunkPos, String), Chunk>;

#[derive(Clone)]
pub struct World {
    storage_backend: LmdbBackend,
    cache: ChunkCache,
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

        let rand_seed = rand::random();

        let cache = ChunkCache::with_hasher(WyHasherBuilder::new(rand_seed));

        World {
            storage_backend,
            cache,
        }
    }

    pub fn get_cache(&self) -> &ChunkCache {
        &self.cache
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn dump_chunk() {
        let world = World::new(std::env::current_dir().unwrap().join("../../../world"));
        let chunk = world.load_chunk(ChunkPos::new(1, 1), "overworld").expect(
            "Failed to load chunk. If it's a bitcode error, chances are the chunk format \
             has changed since last generating a world so you'll need to regenerate",
        );
        let encoded = bitcode::encode(&*chunk);
        std::fs::write("../../../.etc/raw_chunk.dat", encoded).unwrap();
    }
}

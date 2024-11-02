use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use indicatif::ProgressBar;
use rayon::prelude::*;
use tokio::task::JoinSet;
use tracing::{error, info};
use crate::errors::WorldError;
use crate::World;
use ferrumc_anvil::load_anvil_file;
use crate::db_functions::save_chunk_internal;
use crate::vanilla_chunk_format::VanillaChunk;

/// This function is used to check if the import path is valid. It checks if the path exists, if it
/// is a file, if the region folder exists, if the region folder is a file, and if the region folder
/// is empty.
fn check_paths_validity(import_dir: PathBuf) -> Result<(), WorldError> {
    if !import_dir.exists() {
        error!("Import path does not exist: {}", import_dir.to_string_lossy());
        return Err(WorldError::InvalidImportPath(import_dir.to_string_lossy().to_string()));
    }
    if import_dir.is_file() {
        error!("Import path is a file: {}", import_dir.to_string_lossy());
        return Err(WorldError::InvalidImportPath(import_dir.to_string_lossy().to_string()));
    }
    if !import_dir.join("region").exists() {
        error!("Import path does not contain a region folder: {}", import_dir.to_string_lossy());
        return Err(WorldError::InvalidImportPath(import_dir.to_string_lossy().to_string()));
    }
    if import_dir.join("region").is_file() {
        error!("Import path's region folder is a file: {}", import_dir.to_string_lossy());
        return Err(WorldError::InvalidImportPath(import_dir.to_string_lossy().to_string()));
    }
    if let Ok(dir) = import_dir.join("region").read_dir() {
        if dir.count() == 0 {
            error!("Import path's region folder is empty: {}", import_dir.to_string_lossy());
            return Err(WorldError::InvalidImportPath(import_dir.to_string_lossy().to_string()));
        }
    } else {
        error!("Could not read import path's region folder: {}", import_dir.to_string_lossy());
        return Err(WorldError::InvalidImportPath(import_dir.to_string_lossy().to_string()));
    }
    Ok(())
}

impl World {
    
    fn get_chunk_count(&self, import_dir: PathBuf) -> Result<u64, WorldError> {
        info!("Counting chunks in import directory...");
        let regions_dir = import_dir.join("region").read_dir()?;
        let chunk_count = AtomicU64::new(0);
        regions_dir.into_iter().par_bridge().for_each(|region_file| {
            match region_file {
                Ok(dir_entry) => {
                    if dir_entry.path().is_dir() {
                        error!("Region file is a directory: {}", dir_entry.path().to_string_lossy());
                    } else {
                        let file_path = dir_entry.path();
                        let Ok(anvil_file) = load_anvil_file(file_path.clone()) else {
                            error!("Could not load region file: {}", file_path.clone().display());
                            return;
                        };
                        let locations = anvil_file.get_locations();
                        chunk_count.fetch_add(locations.len() as u64, std::sync::atomic::Ordering::Relaxed);
                    }
                }
                Err(e) => {
                    error!("Could not read region file: {}", e);
                }
            }
        });
        Ok(chunk_count.load(std::sync::atomic::Ordering::Relaxed))
    }
    
    pub async fn import(&mut self, import_dir: PathBuf, _: PathBuf) -> Result<(), WorldError> {
        // Check if the import path is valid. We can assume the database path is valid since we
        // checked it in the config validity check.
        check_paths_validity(import_dir.clone())?;
        let regions_dir = import_dir.join("region").read_dir()?;
        let progress_bar = Arc::new(ProgressBar::new(self.get_chunk_count(import_dir)?));
        info!("Importing chunks from import directory...");
        let start = std::time::Instant::now();
        let mut task_set = JoinSet::new();
        for region_file in regions_dir {
            match region_file {
                Ok(dir_entry) => {
                    if dir_entry.path().is_dir() {
                        error!("Region file is a directory: {}", dir_entry.path().to_string_lossy());
                    } else {
                        let file_path = dir_entry.path();
                        let Ok(anvil_file) = load_anvil_file(file_path.clone()) else {
                            error!("Could not load region file: {}", file_path.clone().display());
                            continue;
                        };
                        let locations = anvil_file.get_locations();
                        for location in locations {
                            if let Some(chunk) = anvil_file.get_chunk_from_location(location) {
                                match VanillaChunk::from_bytes(&chunk) {
                                    Ok(vanilla_chunk) => {
                                        let cloned_progress_bar = progress_bar.clone();
                                        let self_clone = self.clone();
                                        task_set.spawn(async move {
                                            if let Ok(chunk) = vanilla_chunk.to_custom_format() {
                                                if let Err(e) = save_chunk_internal(&self_clone, chunk).await {
                                                    error!("Could not save chunk: {}", e);
                                                } else {
                                                    cloned_progress_bar.inc(1);
                                                }
                                            } else {
                                                error!("Could not convert chunk to custom format: {:?}", chunk);
                                            }
                                        });
                                    }
                                    Err(e) => {
                                        error!("Could not convert chunk to vanilla format: {}", e);
                                    }
                                }
                            }
                        };
                    }
                }
                Err(e) => {
                    error!("Could not read region file: {}", e);
                }
            }
        };
        while (task_set.join_next().await).is_some() {}
        self.sync().await?;
        progress_bar.clone().finish();
        info!("Imported {} chunks in {:?}", progress_bar.clone().position(), start.elapsed());
        Ok(())
    }
}

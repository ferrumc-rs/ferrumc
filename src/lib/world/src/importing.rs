use crate::db_functions::save_chunk_internal;
use crate::errors::WorldError;
use crate::vanilla_chunk_format::VanillaChunk;
use crate::World;
use ferrumc_anvil::load_anvil_file;
use ferrumc_general_purpose::paths::BetterPathExt;
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;
use tokio::task::JoinSet;
use tracing::{error, info};

/// This function is used to check if the import path is valid. It checks if the path exists, if it
/// is a file, if the region folder exists, if the region folder is a file, and if the region folder
/// is empty.
fn check_paths_validity(import_dir: PathBuf) -> Result<(), WorldError> {
    if !import_dir.exists() {
        error!(
            "Import path does not exist: {}",
            import_dir.better_display()
        );
        return Err(WorldError::InvalidImportPath(import_dir.better_display()));
    }
    if import_dir.is_file() {
        error!("Import path is a file: {}", import_dir.better_display());
        return Err(WorldError::InvalidImportPath(import_dir.better_display()));
    }

    if let Ok(dir) = import_dir.read_dir() {
        if dir.count() == 0 {
            error!(
                "Import path's region folder is empty: {}",
                import_dir.better_display()
            );
            return Err(WorldError::NoRegionFiles);
        }
    } else {
        error!(
            "Could not read import path's region folder: {}",
            import_dir.better_display()
        );
        return Err(WorldError::InvalidImportPath(import_dir.better_display()));
    }
    Ok(())
}

impl World {
    fn get_chunk_count(&self, import_dir: PathBuf) -> Result<u64, WorldError> {
        info!("Counting chunks in import directory...");
        let regions_dir = import_dir.join("region").read_dir()?;
        let chunk_count = AtomicU64::new(0);
        regions_dir
            .into_iter()
            .par_bridge()
            .for_each(|region_file| match region_file {
                Ok(dir_entry) => {
                    if dir_entry.path().is_dir() {
                        error!(
                            "Region file is a directory: {}",
                            dir_entry.path().to_string_lossy()
                        );
                    } else {
                        let file_path = dir_entry.path();
                        let Ok(anvil_file) = load_anvil_file(file_path.clone()) else {
                            error!(
                                "Could not load region file: {}",
                                file_path.clone().display()
                            );
                            return;
                        };
                        let locations = anvil_file.get_locations();
                        chunk_count.fetch_add(
                            locations.len() as u64,
                            std::sync::atomic::Ordering::Relaxed,
                        );
                    }
                }
                Err(e) => {
                    error!("Could not read region file: {}", e);
                }
            });
        Ok(chunk_count.load(std::sync::atomic::Ordering::Relaxed))
    }

    pub async fn import(&mut self, import_dir: PathBuf, _: PathBuf) -> Result<(), WorldError> {
        // Validate the import directory
        check_paths_validity(import_dir.clone())?;
        let regions_dir = import_dir.join("region").read_dir()?;

        // Set up progress bar and task set
        let progress_bar = Arc::new(ProgressBar::new(self.get_chunk_count(import_dir)?));
        info!("Importing chunks from import directory...");
        let start = std::time::Instant::now();
        let mut task_set = JoinSet::new();

        // Ensure the "chunks" table exists
        self.storage_backend
            .create_table("chunks".to_string())
            .await?;

        // Helper function to process a single region file
        let mut process_region_file = |file_path: PathBuf| -> Result<(), WorldError> {
            let anvil_file = load_anvil_file(file_path.clone()).map_err(|e| {
                error!("Could not load region file: {}", file_path.display());
                e
            })?;
            for location in anvil_file.get_locations() {
                if let Ok(Some(chunk)) = anvil_file.get_chunk_from_location(location) {
                    if let Ok(vanilla_chunk) = VanillaChunk::from_bytes(&chunk) {
                        let cloned_progress_bar = progress_bar.clone();
                        let self_clone = self.clone();
                        task_set.spawn(async move {
                            match vanilla_chunk.to_custom_format() {
                                Ok(custom_chunk) => {
                                    if let Err(e) = save_chunk_internal(&self_clone, custom_chunk).await {
                                        error!("Could not save chunk: {}", e);
                                    } else {
                                        cloned_progress_bar.inc(1);
                                    }
                                }
                                Err(e) => {
                                    error!("Could not convert chunk to custom format: {:?}", e);
                                }
                            }
                        });
                    } else {
                        error!("Could not convert chunk to vanilla format.");
                    }
                } else {
                    error!("Chunk is empty or could not be retrieved from location.");
                }
            }
            Ok(())
        };

        // Iterate through region files and process each one
        for region_file in regions_dir {
            if let Ok(entry) = region_file {
                let file_path = entry.path();
                if file_path.is_dir() {
                    error!("Region file is a directory: {}", file_path.to_string_lossy());
                } else if let Err(e) = process_region_file(file_path) {
                    error!("Error processing region file: {}", e);
                }
            } else if let Err(e) = region_file {
                error!("Could not read region file: {}", e);
            }
        }

        // Await task completion
        while task_set.join_next().await.is_some() {}

        // Finalize progress and flush storage
        self.sync().await?;
        progress_bar.finish();
        info!(
        "Imported {} chunks in {:?}",
        progress_bar.position(),
        start.elapsed()
    );

        self.storage_backend.flush().await?;
        Ok(())
    }

}

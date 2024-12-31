use crate::db_functions::save_chunk_internal_batch;
use crate::errors::WorldError;
use crate::vanilla_chunk_format::VanillaChunk;
use crate::Chunk;
use crate::World;
use ferrumc_anvil::load_anvil_file;
//use ferrumc_general_purpose::paths::BetterPathExt;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tracing::{error, info};

/// TODO: dynamically find the best according to the system
const BATCH_SIZE: usize = 1000; // Number of chunks to process before flushing
const MAX_CONCURRENT_TASKS: usize = 512; // Limit concurrent tasks to prevent memory issues
const FLUSH_INTERVAL: u64 = 10_000; // Flush every 10,000 chunks

impl World {
    async fn process_chunk_batch(
        &self,
        chunks: Vec<VanillaChunk>,
        progress: Arc<ProgressBar>,
        processed_since_flush: Arc<AtomicU64>,
    ) -> Result<(), WorldError> {
        let chunk_objects: Vec<Chunk> = chunks
            .into_iter()
            .filter_map(|chunk| chunk.to_custom_format().ok())
            .collect();

        let mut success_count = 0;
        if let Ok(()) = save_chunk_internal_batch(self, chunk_objects.clone()).await {
            success_count = chunk_objects.len();
        }

        progress.inc(success_count.try_into().unwrap());

        let total_processed = processed_since_flush
            .fetch_add(success_count as u64, Ordering::Relaxed)
            + success_count as u64;
        if total_processed >= FLUSH_INTERVAL {
            self.storage_backend.flush().await?;
            processed_since_flush.store(0, Ordering::Relaxed);
            info!("Performed periodic flush after {} chunks", total_processed);
        }

        Ok(())
    }

    fn get_chunk_count(&self, import_dir: &Path) -> Result<u64, WorldError> {
        info!("Counting chunks in import directory...");
        let regions_dir = import_dir.join("region").read_dir()?;
        let chunk_count = AtomicU64::new(0);

        regions_dir
            .par_bridge()
            .try_for_each(|region_file| -> Result<(), WorldError> {
                let entry = region_file?;
                if entry.path().is_dir() {
                    return Ok(());
                }

                if let Ok(anvil_file) = load_anvil_file(entry.path()) {
                    chunk_count
                        .fetch_add(anvil_file.get_locations().len() as u64, Ordering::Relaxed);
                }
                Ok(())
            })?;

        Ok(chunk_count.load(Ordering::Relaxed))
    }

    pub async fn import(&mut self, import_dir: PathBuf, _: PathBuf) -> Result<(), WorldError> {
        check_paths_validity(&import_dir)?;

        let total_chunks = self.get_chunk_count(&import_dir)?;
        let progress_style = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap();

        let progress = Arc::new(ProgressBar::new(total_chunks));
        progress.set_style(progress_style);

        self.storage_backend
            .create_table("chunks".to_string())
            .await?;

        info!("Starting chunk import...");
        let start = std::time::Instant::now();

        let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_TASKS));
        let mut task_set = JoinSet::new();

        let processed_since_flush = Arc::new(AtomicU64::new(0));

        let regions_dir = import_dir.join("region").read_dir()?;
        let mut current_batch = Vec::with_capacity(BATCH_SIZE);

        for region_result in regions_dir {
            let region_entry = region_result?;
            if region_entry.path().is_dir() {
                continue;
            }

            let anvil_file = match load_anvil_file(region_entry.path()) {
                Ok(file) => file,
                Err(e) => {
                    error!(
                        "Failed to load region file {}: {}",
                        region_entry.path().display(),
                        e
                    );
                    continue;
                }
            };

            for location in anvil_file.get_locations() {
                if let Ok(Some(chunk_data)) = anvil_file.get_chunk_from_location(location) {
                    if let Ok(vanilla_chunk) = VanillaChunk::from_bytes(&chunk_data) {
                        current_batch.push(vanilla_chunk);

                        if current_batch.len() >= BATCH_SIZE {
                            let batch = std::mem::replace(
                                &mut current_batch,
                                Vec::with_capacity(BATCH_SIZE),
                            );
                            let progress_clone = Arc::clone(&progress);
                            let self_clone = self.clone();
                            let permit = Arc::clone(&semaphore).acquire_owned().await?;
                            let processed_since_flush_clone = Arc::clone(&processed_since_flush);

                            task_set.spawn(async move {
                                let _permit = permit;
                                if let Err(e) = self_clone
                                    .process_chunk_batch(
                                        batch,
                                        progress_clone,
                                        processed_since_flush_clone,
                                    )
                                    .await
                                {
                                    error!("Batch processing error: {}", e);
                                }
                            });
                        }
                    }
                }
            }
        }

        if !current_batch.is_empty() {
            let progress_clone = Arc::clone(&progress);
            let permit = Arc::clone(&semaphore).acquire_owned().await?;
            let self_clone = self.clone();
            let processed_since_flush_clone = Arc::clone(&processed_since_flush);

            task_set.spawn(async move {
                let _permit = permit;
                if let Err(e) = self_clone
                    .process_chunk_batch(current_batch, progress_clone, processed_since_flush_clone)
                    .await
                {
                    error!("Final batch processing error: {}", e);
                }
            });
        }

        while task_set.join_next().await.is_some() {}

        self.sync().await?;
        self.storage_backend.flush().await?;

        progress.finish();

        info!(
            "Imported {} chunks in {:?}",
            progress.position(),
            start.elapsed()
        );

        Ok(())
    }
}

fn check_paths_validity(import_dir: &Path) -> Result<(), WorldError> {
    if !import_dir.exists() {
        return Err(WorldError::InvalidImportPath(
            import_dir.display().to_string(),
        ));
    }
    if import_dir.is_file() {
        return Err(WorldError::InvalidImportPath(
            import_dir.display().to_string(),
        ));
    }

    let region_dir = import_dir.join("region");
    if !region_dir.exists() || !region_dir.is_dir() {
        return Err(WorldError::InvalidImportPath(
            import_dir.display().to_string(),
        ));
    }

    match region_dir.read_dir() {
        Ok(dir) => {
            if dir.count() == 0 {
                return Err(WorldError::NoRegionFiles);
            }
        }
        Err(_) => {
            return Err(WorldError::InvalidImportPath(
                import_dir.display().to_string(),
            ));
        }
    }

    Ok(())
}

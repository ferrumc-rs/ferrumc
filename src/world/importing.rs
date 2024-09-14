use crate::database::encoding::ZstdCodec;
use crate::state::GlobalState;
use crate::utils::hash::hash;
use crate::utils::prelude::*;
use crate::world::chunk_format::Chunk;
use fastanvil::{ChunkData, Region};
use indicatif::{ProgressBar, ProgressStyle};
use nbt_lib::NBTDeserializeBytes;
use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{debug, info, warn};

const DEFAULT_BATCH_SIZE: u8 = 150;

/// A serialized chunk is a tuple of the chunk's hash and the compressed chunk data
/// (hash, compressed_chunk_data)
pub struct SerializedChunk(u64, Vec<u8>);

impl SerializedChunk {
    pub fn new(hash: u64, data: Vec<u8>) -> Self {
        Self(hash, data)
    }
    pub fn hash(&self) -> u64 {
        self.0
    }

    pub fn data(&self) -> &Vec<u8> {
        self.1.as_ref()
    }
}

fn get_batch_size() -> i32 {
    let batch_size = env::args()
        .find(|x| x.starts_with("--batch_size="))
        .and_then(|x| x.split('=').last().and_then(|s| s.parse::<i32>().ok()));

    match batch_size {
        Some(size) => {
            info!("Using custom batch size: {}", size);
            size
        }
        None => {
            info!("Using default batch size: {}", DEFAULT_BATCH_SIZE);
            info!("To change the batch size, use the --batch_size=<num> flag");
            DEFAULT_BATCH_SIZE as i32
        }
    }
}

fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();

    if secs == 0 {
        format!("{}ms", millis)
    } else if secs < 60 {
        format!("{}s {}ms", secs, millis)
    } else if secs < 3600 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else {
        format!("{}h {}m {}s", secs / 3600, (secs % 3600) / 60, secs % 60)
    }
}

async fn get_total_chunks(dir: &PathBuf) -> Result<usize> {
    let files = std::fs::read_dir(dir)?;
    let regions: Vec<Region<File>> = files
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file() && entry.path().extension() == Some("mca".as_ref()))
        .filter_map(|entry| match File::open(entry.path()) {
            Ok(file) => Region::from_stream(file).ok(),
            Err(_) => {
                warn!(
                    "(Skipped) Could not read region file: {}",
                    entry.path().display()
                );
                None
            }
        })
        .collect();

    Ok(regions
        .into_par_iter()
        .map(|mut region| region.iter().count())
        .sum())
}

async fn process_chunk(
    chunk_data: Vec<u8>,
    file_name: &str,
    bar: Arc<ProgressBar>,
) -> Result<SerializedChunk> {
    let mut chunk = Chunk::read_from_bytes(&mut Cursor::new(chunk_data)).map_err(|e| {
        bar.abandon_with_message(format!("Chunk {} failed to import", file_name));
        Error::Generic(format!("Could not read chunk {} {}", e, file_name))
    })?;

    chunk.convert_to_net_mode().map_err(|e| {
        bar.abandon_with_message(format!(
            "Chunk {} {} failed to import",
            chunk.x_pos, chunk.z_pos
        ));
        Error::Generic(format!(
            "Could not convert chunk {} {} to network mode: {}",
            chunk.x_pos, chunk.z_pos, e
        ))
    })?;

    chunk.dimension = Some("overworld".to_string());

    let hash = hash((
        chunk
            .dimension
            .as_ref()
            .unwrap_or_else(|| panic!("Invalid chunk @ ({},{})", chunk.x_pos, chunk.z_pos)),
        chunk.x_pos,
        chunk.z_pos,
    ));
    let chunk_data = ZstdCodec::compress_data(chunk)
        .await
        .expect("Failed to compress chunk");

    Ok(SerializedChunk::new(hash, chunk_data))
}

//noinspection RsBorrowChecker
pub async fn import_regions(state: GlobalState) -> Result<()> {
    let dir = get_import_directory()?;
    debug!("Starting import from: {}", dir.display());

    let start = std::time::Instant::now();
    info!("Analyzing world data... (this won't take long)");

    let total_chunks = get_total_chunks(&dir).await?;
    info!("Preparing to import {} chunks", total_chunks);
    info!("This process may take a while for large worlds. Please be patient.");

    let batch_size = get_batch_size() as usize;
    let bar = Arc::new(create_progress_bar(total_chunks));

    let mut region_files = tokio::fs::read_dir(dir)
        .await
        .map_err(|_| Error::Generic("Could not read the imports directory".to_string()))?;

    while let Some(dir_file) = region_files.next_entry().await? {
        let file_name = dir_file.file_name();
        let file_name = file_name.to_str().unwrap_or("unknown file");
        let file = File::open(dir_file.path())?;
        let mut region = Region::from_stream(file)?;

        let mut chunks: Vec<ChunkData> = region.iter().filter_map(|chunk| chunk.ok()).collect();
        while !chunks.is_empty() {
            let chunk_batch: Vec<ChunkData> = chunks
                .drain(..std::cmp::min(batch_size, chunks.len()))
                .collect();

            let processed_chunks_futures: Vec<_> = chunk_batch
                .into_iter()
                .map(|chunk| {
                    let data = chunk.data.clone();
                    let bar_clone = Arc::clone(&bar);
                    let file_name = file_name.to_string();
                    tokio::spawn(async move {
                        match process_chunk(data, &file_name, Arc::clone(&bar_clone)).await {
                            Ok(processed) => {
                                bar_clone.inc(1);
                                Some(processed)
                            }
                            Err(e) => {
                                warn!("Failed to process chunk: {}. Skipping.", e);
                                None
                            }
                        }
                    })
                })
                .collect();

            let processed_chunks: Vec<SerializedChunk> =
                futures::future::join_all(processed_chunks_futures)
                    .await
                    .into_iter()
                    .filter_map(|result| result.ok().flatten())
                    .collect();

            insert_chunks(&state, processed_chunks, &bar).await?;
        }
    }

    finalize_import(&bar, total_chunks, start.elapsed());
    Ok(())
}

fn get_import_directory() -> Result<PathBuf> {
    if let Ok(root) = env::var("FERRUMC_ROOT") {
        Ok(PathBuf::from(root).join("import"))
    } else {
        env::current_exe()?
            .parent()
            .ok_or_else(|| Error::Generic("Failed to get exe directory".to_string()))
            .map(|path| path.join("import"))
    }
}

fn create_progress_bar(total_chunks: usize) -> ProgressBar {
    let bar = ProgressBar::new(total_chunks as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:100.cyan/blue} {pos:>7}/{len:7} {msg}")
            .expect("Could not set progress bar style")
            .progress_chars("##-"),
    );
    bar.set_message("Importing chunks...");
    bar
}

async fn insert_chunks(
    state: &GlobalState,
    queued_chunks: Vec<SerializedChunk>,
    bar: &ProgressBar,
) -> Result<()> {
    state
        .database
        .batch_insert(queued_chunks)
        .await
        .map_err(|e| {
            bar.abandon_with_message("Chunk insertion failed".to_string());
            Error::Generic(format!("Could not insert chunks: {}", e))
        })?;
    Ok(())
}

fn finalize_import(bar: &ProgressBar, total_chunks: usize, elapsed: std::time::Duration) {
    bar.finish_with_message(format!(
        "Import complete! {} chunks processed.",
        total_chunks
    ));
    info!(
        "Successfully imported {} chunks in {}",
        total_chunks,
        format_duration(elapsed)
    );
}

#[cfg(test)]
mod test {
    use crate::create_state;
    use crate::utils::prelude::*;
    use crate::utils::setup_logger;
    use tokio::net::TcpListener;

    #[tokio::test]
    #[ignore]
    async fn get_chunk_at() -> Result<()> {
        // set environment variable "FERRUMC_ROOT" to the root of the ferrumc project
        setup_logger()?;
        let listener = TcpListener::bind("0.0.0.0:0").await?;
        let state = create_state(listener).await?;

        let chunk = state
            .database
            .get_chunk(0, 0, "overworld".to_string())
            .await?
            .unwrap();

        println!("{:#?}", chunk);

        Ok(())
    }
}

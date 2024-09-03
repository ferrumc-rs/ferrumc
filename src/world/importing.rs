use std::io::Cursor;
use std::path::PathBuf;
use std::process::exit;

use indicatif::ProgressBar;
use nbt_lib::NBTDeserializeBytes;
use tokio::task::JoinSet;
use tracing::{debug, error, info, trace, warn};

use crate::state::GlobalState;
use crate::world::chunkformat::Chunk;

fn format_time(millis: u64) -> String {
    if millis < 1000 {
        format!("{}ms", millis)
    } else if millis < 60_000 {
        format!("{}s {}ms", millis / 1000, millis % 1000)
    } else if millis < 3_600_000 {
        format!(
            "{}m {}s {}ms",
            millis / 60_000,
            (millis % 60_000) / 1000,
            millis % 1000
        )
    } else {
        format!(
            "{}h {}m {}s {}ms",
            millis / 3_600_000,
            (millis % 3_600_000) / 60_000,
            (millis % 60_000) / 1000,
            millis % 1000
        )
    }
}

async fn get_total_chunks(dir: PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let mut region_files = tokio::fs::read_dir(dir).await?;
    let mut total_chunks = 0;
    let mut set = JoinSet::new();
    while let Some(dirfile) = region_files.next_entry().await? {
        set.spawn_blocking(move || {
            let file = std::fs::File::open(dirfile.path()).unwrap();
            let mut region = fastanvil::Region::from_stream(file).unwrap();
            region.iter().count()
        });
    }
    while let Some(Ok(count)) = set.join_next().await {
        total_chunks += count;
    }
    Ok(total_chunks)
}

/// since this is just used to import chunks, it doesn't need to be optimized much
pub async fn import_regions(
    dir: PathBuf,
    state: GlobalState,
) -> Result<(), Box<dyn std::error::Error>> {
    // We aren't, but I can't think of a better way to say "Counting chunks" without it sounding
    // super slow
    info!("Fetching preliminary chunk information");
    let total_chunks = get_total_chunks(dir.clone()).await?;
    let mut region_files = if tokio::fs::read_dir(dir.clone()).await.is_ok() {
        tokio::fs::read_dir(dir).await?
    } else {
        error!("Could not read the imports directory");
        return Ok(());
    };
    info!("Importing {} chunks", total_chunks);
    info!("This could take a while if there are a lot of chunks to import!");
    let start = std::time::Instant::now();
    let bar = ProgressBar::new(total_chunks as u64);
    while let Some(dirfile) = region_files.next_entry().await? {
        let file = std::fs::File::open(dirfile.path())?;
        let mut region = fastanvil::Region::from_stream(file)?;

        let mut queued_chunks = Vec::new();
        let mut pending_chunks = 0u8;

        for chunk in region.iter() {
            let Ok(chunk) = chunk else {
                warn!(
                    "Could not read chunk {}",
                    dirfile.file_name().to_str().unwrap()
                );
                bar.abandon_with_message(format!(
                    "Chunk {} failed to import",
                    dirfile.file_name().to_str().unwrap()
                ));
                exit(1);
            };

            /*           // FIXME: remove this
            if chunk.z != 0 || chunk.x != 0 {
                continue;
            }*/

            let chunk_raw = chunk.data;

            let chunk_nbt = Chunk::read_from_bytes(&mut Cursor::new(chunk_raw));

            if chunk_nbt.is_err() {
                warn!(
                    "Could not read chunk {} {}",
                    chunk_nbt.as_ref().unwrap_err(),
                    dirfile.file_name().to_str().unwrap()
                );
                bar.abandon_with_message(format!(
                    "Chunk {} failed to import",
                    dirfile.file_name().to_str().unwrap()
                ));
                exit(1);
            }
            let mut final_chunk = chunk_nbt.unwrap();
            // final_chunk.convert_to_net_mode().unwrap();
            match final_chunk.convert_to_net_mode() {
                Ok(_) => {}
                Err(e) => {
                    warn!(
                        "Could not convert chunk {} {} to network mode: {}",
                        final_chunk.x_pos, final_chunk.z_pos, e
                    );
                    bar.abandon_with_message(format!(
                        "Chunk {} {} failed to import",
                        final_chunk.x_pos, final_chunk.z_pos
                    ));
                    exit(1);
                }
            }
            let x = final_chunk.x_pos.clone();
            let z = final_chunk.z_pos.clone();
            final_chunk.dimension = Some("overworld".to_string());

            trace!("Inserting chunk {}, {}", x, z);

            queued_chunks.push(final_chunk);
            pending_chunks += 1;

            if pending_chunks == 150 {
                let res = state.database.batch_insert(queued_chunks).await;
                if res.is_err() {
                    error!(
                        "Could not insert chunk {} {}: {}",
                        x,
                        z,
                        res.as_ref().unwrap_err()
                    );
                    bar.abandon_with_message(format!("Chunk {} {} failed to import", x, z));
                    exit(1);
                }
                queued_chunks = Vec::new();
                pending_chunks = 0;
            }
            bar.inc(1);
            bar.set_message(format!("{} {}", x, z));

            trace!("Imported chunk {} {}", x, z);
        }
        if !queued_chunks.is_empty() {
            let res = state.database.batch_insert(queued_chunks).await;
            if res.is_err() {
                error!(
                    "Could not insert chunk {}: {}",
                    dirfile.file_name().to_str().unwrap(),
                    res.as_ref().unwrap_err()
                );
                bar.abandon_with_message(format!(
                    "Chunk {} failed to import",
                    dirfile.file_name().to_str().unwrap()
                ));
                exit(1);
            }
        }
    }
    bar.abandon_with_message(format!("{} chunks imported!", total_chunks));
    info!(
        "Imported {} chunks in {} seconds",
        total_chunks,
        format_time(start.elapsed().as_millis() as u64)
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use tokio::net::TcpListener;

    use crate::create_state;
    use crate::utils::setup_logger;

    #[tokio::test]
    async fn get_chunk_at() {
        // set environment variable "FERRUMC_ROOT" to the root of the ferrumc project
        setup_logger().unwrap();
        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let state = create_state(listener).await.unwrap();

        let chunk = state
            .database
            .get_chunk(0, 0, "overworld")
            .await
            .unwrap()
            .unwrap();

        println!("{:#?}", chunk);
    }
}

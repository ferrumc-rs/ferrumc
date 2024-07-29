use std::path::PathBuf;

use serde::Serialize;
use tracing::{info, warn};

use crate::state::GlobalState;
use crate::world::chunkformat::Chunk;

pub async fn import_regions(
    dir: PathBuf,
    state: GlobalState,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut region_files = tokio::fs::read_dir(dir).await?;
    while let Some(dirfile) = region_files.next_entry().await? {
        let file = std::fs::File::open(dirfile.path())?;
        let mut region = fastanvil::Region::from_stream(file)?;
        for chunk in region.iter() {
            let chunk = chunk?.data;
            let chunk_nbt: Chunk = fastnbt::from_bytes(&chunk)?;
            let record_name = format!("{},{}", chunk_nbt.x_pos, chunk_nbt.z_pos);

            let mut ser = flexbuffers::FlexbufferSerializer::new();
            chunk_nbt.serialize(&mut ser).unwrap();
            let protochunk = crate::world::ProtoChunk {
                x: chunk_nbt.x_pos,
                z: chunk_nbt.z_pos,
                data: ser.take_buffer(),
            };
            let record: Option<Chunk> = state
                .write()
                .await
                .database
                .conn
                .create(("chunks", record_name))
                .content(chunk_nbt.clone())
                .await
                .unwrap();
            match record {
                Some(_) => {
                    info!(
                        "Chunk {} {} added to database",
                        chunk_nbt.clone().x_pos,
                        chunk_nbt.clone().z_pos
                    );
                }
                None => {
                    warn!(
                        "Could not add chunk {} {} to database",
                        chunk_nbt.clone().x_pos,
                        chunk_nbt.clone().z_pos
                    );
                }
            }
        }
        info!(
            "Finished importing region file {}",
            dirfile.file_name().to_str().unwrap()
        );
    }
    Ok(())
}

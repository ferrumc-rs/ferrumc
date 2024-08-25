use crate::world::chunkformat::Heightmaps;
use ferrumc_codec::enc::Encode;
use nbt_lib::{read_tag, NBTSerialize, Serialize};
use std::io::Cursor;
use tracing::debug;

#[tokio::test]
pub async fn test_heightmaps() -> Result<(), Box<dyn std::error::Error>> {
    use crate::utils::setup_logger;
    use tokio::net::TcpListener;
    setup_logger().unwrap();
    let state = crate::create_state(TcpListener::bind("0.0.0.0:0").await.unwrap())
        .await
        .unwrap();

    let chunk = state
        .database
        .get_chunk(0, 0, "overworld")
        .await
        .unwrap()
        .unwrap();

    debug!("Chunk: {:?}", chunk);

    let heightmaps = chunk.heightmaps.unwrap();

    let mut buffer = Vec::new();
    heightmaps.encode(&mut buffer).await.unwrap();

    std::fs::write(".etc/heightmaps.nbt", buffer).unwrap();

    Ok(())
}

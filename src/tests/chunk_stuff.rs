use std::io::Cursor;
use ferrumc_codec::enc::Encode;
use tracing::debug;
use nbt_lib::{read_tag, NBTSerialize};
use crate::world::chunkformat::Heightmaps;

#[tokio::test]
pub async fn test_heightmaps() -> Result<(), Box<dyn std::error::Error>> {
    use crate::utils::setup_logger;
    use tokio::net::TcpListener;
    setup_logger().unwrap();
    /*let state = crate::create_state(TcpListener::bind("0.0.0.0:0").await.unwrap())
        .await
        .unwrap();

    let chunk = state
        .database
        .get_chunk(0, 0, "overworld")
        .await
        .unwrap()
        .unwrap();

    debug!("Chunk: {:?}", chunk);

    let heightmaps = chunk.heightmaps.unwrap();*/
    let heightmaps = Heightmaps {
        motion_blocking_no_leaves: None,
        motion_blocking: Some(vec![1, 2, 3]),
        ocean_floor: None,
        world_surface: None,
    };


    let mut buffer = Vec::new();
    10u8.serialize(&mut buffer).unwrap();
    "heightmaps".serialize(&mut buffer).unwrap();
    heightmaps.serialize(&mut buffer).unwrap();

    std::fs::write(".etc/heightmaps.nbt", buffer).unwrap();

    Ok(())
}
use ferrumc_codec::enc::NetEncode;

use crate::utils::config::get_global_config;

#[tokio::test]
#[ignore]
pub async fn dump_heightmaps() -> Result<(), Box<dyn std::error::Error>> {
    use crate::utils::setup_logger;
    use tokio::net::TcpListener;
    setup_logger().unwrap();
    let state = crate::create_state(TcpListener::bind("0.0.0.0:0").await.unwrap())
        .await
        .unwrap();

    let chunk = state
        .database
        .get_chunk(0, 0, "overworld".to_string())
        .await
        .unwrap()
        .unwrap();

    let heightmaps = chunk.heightmaps.unwrap();

    let mut buffer = Vec::new();
    heightmaps
        .net_encode(
            &mut buffer,
            &get_global_config().compression_and_encode_opt(),
        )
        .await
        .unwrap();

    std::fs::write(".etc/heightmaps.nbt", buffer).unwrap();

    Ok(())
}

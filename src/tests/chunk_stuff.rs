use ferrumc_codec::enc::NetEncode;

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
    heightmaps.net_encode(&mut buffer).await.unwrap();

    std::fs::write(".etc/heightmaps.nbt", buffer).unwrap();

    Ok(())
}

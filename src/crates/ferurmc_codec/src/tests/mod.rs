#[tokio::test]
async fn test_encode_bool() {
    let mut buf = Vec::new();
    true.net_encode(&mut buf).await.unwrap();
    assert_eq!(buf, vec![1]);
    buf.clear();
    false.net_encode(&mut buf).await.unwrap();
    assert_eq!(buf, vec![0]);
}

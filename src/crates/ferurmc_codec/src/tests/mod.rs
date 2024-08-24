use crate::enc::Encode;

#[tokio::test]
async fn test_encode_bool() {
    let mut buf = Vec::new();
    true.encode(&mut buf).await.unwrap();
    assert_eq!(buf, vec![1]);
    buf.clear();
    false.encode(&mut buf).await.unwrap();
    assert_eq!(buf, vec![0]);
}


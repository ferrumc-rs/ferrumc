use ferrumc_macros::NetEncode;
use ferrumc_net_codec::encode::NetEncode;
use std::io::Write;
use tokio::io::AsyncWriteExt;

#[derive(NetEncode)]
struct SomeExampleEncStruct {
    pub field1: u32,
    pub field2: u32,
}

#[test]
fn test_encode() {
    let example = SomeExampleEncStruct {
        field1: 42,
        field2: 69,
    };
    let mut writer = Vec::<u8>::new();
    example
        .encode(&mut writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)
        .unwrap();

    // save in file (.etc/tests)
    let result = std::fs::write(
        r#"D:\Minecraft\framework\ferrumc\ferrumc-2_0\ferrumc\.etc\tests/enc_test_encode"#,
        writer,
    );
    result.unwrap();
}

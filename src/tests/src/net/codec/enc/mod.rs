use ferrumc_macros::NetEncode;
use ferrumc_net_codec::{encode::NetEncode, net_types::var_int::VarInt};
use std::io::Write;
use tokio::io::AsyncWriteExt;

#[derive(NetEncode)]
struct SomeExampleEncStruct {
    pub field1: u32,
    pub field2: u32,
}

#[derive(NetEncode)]
#[expect(dead_code)]
struct SomeExampleEncStructWithPacketId {
    pub packet_id: VarInt, // Make this 0x10
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

#[allow(unreachable_code)]
fn _test_compression() -> ! {
    let example = SomeExampleEncStructWithPacketId {
        packet_id: VarInt::from(0x10),
        field1: 42,
        field2: 69,
    };
    // infinite loop, do not use in cargo test
    loop {
        let mut writer = Vec::<u8>::new();
        example
            .encode(
                &mut writer,
                &ferrumc_net_codec::encode::NetEncodeOpts::Compressed,
            )
            .unwrap();
    }
}
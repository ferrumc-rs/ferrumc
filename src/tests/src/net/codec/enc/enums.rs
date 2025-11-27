use ferrumc_macros::NetEncode;
use ferrumc_protocol::codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_protocol::codec::encode::{NetEncode, NetEncodeOpts};

#[derive(NetEncode)]
enum TestPacket {
    Ping { timestamp: i64 },
}

#[test]
fn main() {
    let packet = TestPacket::Ping {
        timestamp: 1234567890,
    };
    let mut buffer = Vec::new();
    packet.encode(&mut buffer, &NetEncodeOpts::None).unwrap();

    assert_eq!(
        1234567890,
        i64::decode(&mut buffer.as_slice(), &NetDecodeOpts::None).unwrap()
    );
}

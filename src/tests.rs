#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use ferrumc_macros::Decode;
    use ferrumc_utils::encoding::varint::VarInt;

    #[tokio::test]
    async fn test_macro_decode() {
        #[derive(Decode, Default)]
        struct Handshake {
            protocol_version: VarInt,
            server_address: String,
            server_port: u16,
            next_state: VarInt,
        }
        let mut data = Cursor::new(vec![
            0xFB, 0x05, 0x09, 0x31, 0x32, 0x37, 0x2E, 0x30, 0x2E, 0x30, 0x2E, 0x31, 0x63, 0xDD,
            0x01,
        ]);
        let handshake = Handshake::decode(&mut data).await.unwrap();
        assert_eq!(handshake.protocol_version, VarInt::new(763));
        assert_eq!(handshake.server_address, "127.0.0.1".to_string());
        assert_eq!(handshake.server_port, 25565);
        assert_eq!(handshake.next_state, VarInt::new(1));
    }
}

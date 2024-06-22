#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::io::Read;

    use ferrumc_macros::Decode;
    use ferrumc_utils::encoding::varint::VarInt;
    use ferrumc_utils::type_impls::Decode;

    use crate::error::Error;

    #[test]
    fn test_macro_decode()
    {
        #[derive(Decode, Default)]
        struct Handshake {
            protocol_version: VarInt,
            server_address: String,
            server_port: u16,
            next_state: VarInt,
        }
        let mut data = Cursor::new(vec![0xFB, 0x05, 0x09, 0x31, 0x32, 0x37, 0x2E, 0x30, 0x2E, 0x30, 0x2E, 0x31, 0x63, 0xDD, 0x01]);
        let handshake = Handshake::decode(&mut data).unwrap();
        assert_eq!(handshake.protocol_version, VarInt::new(763));
    }
}
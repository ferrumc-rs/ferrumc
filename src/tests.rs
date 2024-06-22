#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::io::Read;

    use ferrumc_macros::Decode;
    use ferrumc_utils::type_impls::Decode;

    use crate::error::Error;

    #[test]
    fn test_macro_decode()
    {
        #[derive(Decode, Default)]
        struct Handshake {
            protocol_version: u32,
            server_address: String,
            server_port: u16,
            next_state: u32,
        }
        let mut data = Cursor::new(vec![0x00, 0x00, 0x00, 0x00, 0x0A]);
        let handshake = Handshake::decode(&mut data);
    }
}
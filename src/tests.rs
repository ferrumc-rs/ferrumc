#[cfg(test)]
mod tests {
    use ferrumc_macros::Decode;

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
        
        let handshake = Handshake::decode(&mut vec![0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8]);
        
    }
}
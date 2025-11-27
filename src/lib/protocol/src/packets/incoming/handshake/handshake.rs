use ferrumc_macros::{packet, NetDecode};
use ferrumc_protocol::codec::net_types::var_int::VarInt;
use ferrumc_protocol::ids;

#[derive(NetDecode, Debug)]
#[packet(id = ids::HANDSHAKE_SERVERBOUND_INTENTION, state = "handshake")]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}

#[cfg(test)]
mod tests {
    use ferrumc_macros::NetDecode;
    use ferrumc_protocol::codec::decode::{NetDecode, NetDecodeOpts};
    use ferrumc_protocol::codec::net_types::var_int::VarInt;
    use std::io::Cursor;

    #[test]
    fn test_macro_decode() {
        #[derive(NetDecode, Default)]
        struct Handshake {
            protocol_version: VarInt,
            server_address: String,
            server_port: u16,
            next_state: VarInt,
        }
        let mut data = Cursor::new(vec![
            255, 5, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 99, 221, 1,
        ]);

        let handshake = Handshake::decode(&mut data, &NetDecodeOpts::None).unwrap();
        // Although the 1.21.8 protocol version is 772, we don't need to actually account for that here,
        // so using the 767 version is fine for testing purposes.
        assert_eq!(handshake.protocol_version, VarInt::new(767));
        assert_eq!(handshake.server_address, "localhost".to_string());
        assert_eq!(handshake.server_port, 25565);
        assert_eq!(handshake.next_state, VarInt::new(1));
    }
}

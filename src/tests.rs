#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use serde_derive::{Deserialize, Serialize};

    use ferrumc_macros::{Decode, NBTDecode};

    use crate::utils::encoding::varint::VarInt;

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

    #[tokio::test]
    async fn test_nbt_decode() {
        #[derive(NBTDecode, Serialize, Deserialize, Clone)]
        struct Test {
            test: i32,
            #[nbtcompound]
            nested: Nested,
        }

        #[derive(NBTDecode, Serialize, Deserialize, Clone)]
        #[nbtcompound]
        struct Nested {
            second_test: i8,
        }

        let structed = Test {
            test: 1,
            nested: Nested { second_test: 2 },
        };

        let data = fastnbt::to_bytes(&structed).unwrap();

        let decoded = Test::decode(data);
        assert!(decoded.is_ok());
        let decoded = decoded.unwrap();
        assert_eq!(decoded.test, 1);
        assert_eq!(decoded.nested.second_test, 2);
    }
}

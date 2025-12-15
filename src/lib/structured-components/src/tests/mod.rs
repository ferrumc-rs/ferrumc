#[cfg(test)]
mod tests {
    use crate::netcode::components::potion_contents::PotionContents;
    use crate::netcode::generated::StructuredComponent;
    use ferrumc_net_codec::decode::NetDecode;
    use ferrumc_net_codec::decode::NetDecodeOpts;
    use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
    use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
    use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
    use ferrumc_net_codec::net_types::var_int::VarInt;
    use std::io::Cursor;

    /// NOTE:
    /// Structured components use an asymmetric protocol:
    /// - client -> server: id + length + data
    /// - server -> client: id + data
    #[test]
    fn decode_structured_component_with_length_prefix_from_client() {
        let bytes = vec![
            42, // component id (potion_contents)
            5,  // length prefix (client-only, ignored in implementation)
            1,  // potion_id present
            1,  // potion_id = 1
            0,  // custom_color absent
            0,  // effects length = 0
            0,  // empty string
        ];

        let mut cursor = Cursor::new(&bytes);

        let component = StructuredComponent::decode(&mut cursor, &NetDecodeOpts::None)
            .expect("decode must succeed");

        match component {
            StructuredComponent::PotionContents(contents) => {
                assert_eq!(contents.potion_id, PrefixedOptional::Some(VarInt(1)));
                assert_eq!(contents.custom_color, PrefixedOptional::None);
                assert!(contents.custom_effects.data.is_empty());
                assert!(contents.custom_name.is_empty());
            }
            other => {
                panic!("Expected PotionContents component, got {:?}", other);
            }
        }

        assert_eq!(
            cursor.position(),
            bytes.len() as u64,
            "Decoder did not consume all bytes (protocol mismatch)"
        );
    }

    #[test]
    fn encode_structured_component_without_length_prefix_for_client() {
        let component = StructuredComponent::PotionContents(PotionContents {
            potion_id: PrefixedOptional::Some(VarInt(1)),
            custom_color: PrefixedOptional::None,
            custom_effects: LengthPrefixedVec::default(),
            custom_name: String::new(),
        });

        let mut buf = Vec::new();

        component
            .encode(&mut buf, &NetEncodeOpts::None)
            .expect("encode must succeed");

        let expected = vec![
            42, // component id
            1,  // potion_id present
            1,  // potion_id = 1
            0,  // custom_color absent
            0,  // effects length = 0
            0,  // empty string
        ];

        assert_eq!(
            buf, expected,
            "Structured component encoding must NOT include length prefix"
        );
    }
}

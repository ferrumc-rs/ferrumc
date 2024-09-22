#[cfg(test)]
mod tests {
    use ferrumc_macros::{NBTDeserialize, NBTSerialize};

    #[test]
    fn test_mixed_slice_types() {
        #[derive(NBTSerialize, NBTDeserialize, Debug, PartialEq)]
        struct MixedSlices<'a> {
            bytes: &'a [u8],
            ints: &'a [i32],
            longs: &'a [i64],
            text: &'a str,
        }

        let original = MixedSlices {
            bytes: &[1, 2, 3, 4],
            ints: &[-1, 0, 1],
            longs: &[1000000000000, -1000000000000],
            text: "Hello, NBT!",
        };

        let mut buffer = Vec::new();
        original.serialize_with_header(&mut buffer);

        let deserialized = MixedSlices::from_bytes(buffer.as_slice()).unwrap();
        
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_nested_structures_with_options() {
        #[derive(NBTSerialize, NBTDeserialize, Debug, PartialEq)]
        struct NestedStructure {
            data: Vec<OptionalData>,
            metadata: Option<Metadata>,
        }

        #[derive(NBTSerialize, NBTDeserialize, Debug, PartialEq)]
        struct OptionalData {
            id: u32,
            value: Option<i64>,
        }

        #[derive(NBTSerialize, NBTDeserialize, Debug, PartialEq)]
        struct Metadata {
            created_at: i64,
            tags: Vec<String>,
        }

        let original = NestedStructure {
            data: vec![
                OptionalData { id: 1, value: Some(100) },
                OptionalData { id: 2, value: None },
                OptionalData { id: 3, value: Some(-100) },
            ],
            metadata: Some(Metadata {
                created_at: 1632268800,
                tags: vec!["test".to_string(), "nbt".to_string()],
            }),
        };

        let mut buffer = Vec::new();
        original.serialize_with_header(&mut buffer);

        let deserialized = NestedStructure::from_bytes(buffer.as_slice()).unwrap();

        assert_eq!(original, deserialized);
    }
    
    #[test]
    fn test_recursive_structure() {
        #[derive(NBTSerialize, NBTDeserialize, Debug, PartialEq)]
        struct Node {
            value: i32,
            children: Vec<Node>,
        }

        fn create_tree(depth: usize) -> Node {
            if depth == 0 {
                Node { value: 1, children: vec![] }
            } else {
                Node {
                    value: depth as i32,
                    children: vec![create_tree(depth - 1), create_tree(depth - 1)],
                }
            }
        }

        let original = create_tree(3);

        let mut buffer = Vec::new();
        original.serialize_with_header(&mut buffer);

        let deserialized = Node::from_bytes(buffer.as_slice()).unwrap();

        assert_eq!(original, deserialized);
    }
}
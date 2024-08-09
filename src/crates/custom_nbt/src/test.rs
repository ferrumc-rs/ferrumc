use std::collections::HashMap;
use crate::nbt_spec::named_tag::NamedTag;
use crate::nbt_spec::serializer::NBTSerialize;
use crate::nbt_spec::tag::Tag;

pub struct NBTTestStruct {
    pub byte_field: i8,
    pub short_field: i16,
    pub int_field: i32,
    pub long_field: i64,
    pub float_field: f32,
    pub double_field: f64,
    pub string_field: String,
    pub byte_array_field: Vec<i8>,
    pub int_array_field: Vec<i32>,
    pub long_array_field: Vec<i64>,
    pub list_field: Vec<Tag>,
    pub compound_field: HashMap<String, NamedTag>,
}

impl NBTTestStruct {
    pub fn new() -> Self {
        NBTTestStruct {
            byte_field: 127,
            short_field: 32767,
            int_field: 2147483647,
            long_field: 9223372036854775807,
            float_field: std::f32::consts::PI,
            double_field: std::f64::consts::E,
            string_field: "Hello, NBT!".to_string(),
            byte_array_field: vec![i8::MIN, 0, i8::MAX],
            int_array_field: vec![i32::MIN, 0, i32::MAX],
            long_array_field: vec![i64::MIN, 0, i64::MAX],
            list_field: vec![
                Tag::Int(1),
                Tag::Int(2),
                Tag::Int(3),
            ],
            compound_field: {
                let mut map = HashMap::new();
                map.insert("nested_string".to_string(), NamedTag::new("nested_string".to_string(), Tag::String("Nested compound value".to_string())));
                map.insert("nested_int".to_string(), NamedTag::new("nested_int".to_string(), Tag::Int(42)));
                map
            },
        }
    }
    pub fn to_nbt(self) -> NamedTag {
        let mut compound = HashMap::new();

        compound.insert("byte_field".to_string(), NamedTag::new("byte_field".to_string(), Tag::Byte(self.byte_field)));
        compound.insert("short_field".to_string(), NamedTag::new("short_field".to_string(), Tag::Short(self.short_field)));
        compound.insert("int_field".to_string(), NamedTag::new("int_field".to_string(), Tag::Int(self.int_field)));
        compound.insert("long_field".to_string(), NamedTag::new("long_field".to_string(), Tag::Long(self.long_field)));
        compound.insert("float_field".to_string(), NamedTag::new("float_field".to_string(), Tag::Float(self.float_field)));
        compound.insert("double_field".to_string(), NamedTag::new("double_field".to_string(), Tag::Double(self.double_field)));
        compound.insert("string_field".to_string(), NamedTag::new("string_field".to_string(), Tag::String(self.string_field)));
        compound.insert("byte_array_field".to_string(), NamedTag::new("byte_array_field".to_string(), Tag::ByteArray(self.byte_array_field)));
        compound.insert("int_array_field".to_string(), NamedTag::new("int_array_field".to_string(), Tag::IntArray(self.int_array_field)));
        compound.insert("long_array_field".to_string(), NamedTag::new("long_array_field".to_string(), Tag::LongArray(self.long_array_field)));
        compound.insert("list_field".to_string(), NamedTag::new("list_field".to_string(), Tag::List(self.list_field)));
        compound.insert("compound_field".to_string(), NamedTag::new("compound_field".to_string(), Tag::Compound(self.compound_field)));

        NamedTag::new("root".to_string(), Tag::Compound(compound))
    }
}

use syn::{Field, LitStr, Meta};
use crate::helpers::is_field_type_optional;

/// Enum representing possible attributes that can be present on a field for serialization/deserialization.
#[derive(Debug)]
pub enum NbtFieldAttribute {
    /// Represents the `rename` attribute, e.g., `#[nbt(rename = "new_name")]`.
    Rename{ new_name: String },
    /// If the field should be completely skipped, and use field's Default method.
    Skip,
    /// If the field is optional or not
    Optional,
}

impl NbtFieldAttribute {
    pub fn from_field(field: &Field) -> Vec<NbtFieldAttribute> {
        let mut attributes = Vec::new();

        for attr in &field.attrs {
            if !attr.path().is_ident("nbt") {
                continue;
            }

            let meta = &attr.meta;
            let Meta::List(list) = meta else {
                continue;
            };

            list.parse_nested_meta(|nested_meta| {
                let name = nested_meta.path.get_ident().expect("Expected an identifier");

                match name.to_string().as_str() {
                    "rename" => {
                        let rename = nested_meta.value().expect("Expected rename to have a value");
                        let rename = rename.parse::<LitStr>().expect("Expected rename to be a string");
                        attributes.push(NbtFieldAttribute::Rename{ new_name: rename.value() });
                    }
                    "skip" => {
                        attributes.push(NbtFieldAttribute::Skip);
                    }
                    _ => panic!("Unknown attribute: {}", name),
                }

                Ok(())
            }).expect("Failed to parse nested meta");
        }

        let optional = is_field_type_optional(field);
        
        if optional {
            attributes.push(NbtFieldAttribute::Optional);
        }
        
        attributes
    }
}

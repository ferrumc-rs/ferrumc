use crate::helpers::is_field_type_optional;
use syn::{Variant, Field, LitStr, LitInt, Meta, DeriveInput};

#[derive(Debug, Clone)]
pub enum Cases {
    Normal,
    LowerCase,
    UpperCase,
    SnakeCase,
    CamelCase,
}

impl Cases {
    pub fn transform(&self, str: impl Into<String>) -> String {
        let str = str.into();
        match self {
            Self::Normal => str,
            Self::UpperCase => str.to_uppercase(),
            Self::LowerCase => str.to_lowercase(),
            Self::SnakeCase => {
                let mut snake_case = String::with_capacity(str.len());
                for (i, c) in str.chars().enumerate() {
                    if c.is_uppercase() {
                        if i > 0 {
                            snake_case.push('_');
                        }
                        snake_case.extend(c.to_lowercase());
                    } else {
                        snake_case.push(c);
                    }
                }
                snake_case
            },
            Self::CamelCase => {
                let mut camel_case = String::with_capacity(str.len());
                let mut next_word = false;
                for c in str.chars() {
                    if c == '_' {
                        next_word = true;
                    } else {
                        if  next_word {
                            camel_case.extend(c.to_uppercase());
                            next_word = false;
                        } else {
                            camel_case.push(c);
                        }
                    }
                }
                camel_case
            },
        }
    }
}

impl From<String> for Cases {
    fn from(value: String) -> Self {
        match value.as_str() {
            "default" => Self::Normal,
            "lower_case" => Self::LowerCase,
            "upper_case" => Self::UpperCase,
            "snake_case" => Self::SnakeCase,
            "camel_case" => Self::CamelCase,
            _ => unimplemented!(),
        }
    }
}

/// Enum representing possible attributes that can be present on a field for serialization/deserialization.
#[derive(Debug)]
pub enum NbtFieldAttribute {
    /// Represents the `rename` attribute, e.g., `#[nbt(rename = "new_name")]`.
    Rename { new_name: String },
    /// Rename all fields or tagged enum variants to match a case.
    RenameAll { case: Cases },
    /// For enums only.
    Tag { tag: String },
    /// For enums only.
    Content { content: String },
    /// Changes the tag type used in seralization.
    TagType { tag: u8 },
    /// Flatten the contents of this field into the container it is defined in.
    Flatten,
    /// Field will be skip if the condition is true.
    SkipIf { condition: String },
    /// If the field should be completely skipped, and use field's Default method.
    Skip,
    /// If the field is optional or not
    Optional,
}

impl NbtFieldAttribute {
    pub fn from_input(input: &DeriveInput) -> Vec<NbtFieldAttribute> {
        let mut attributes = Vec::new();

        for attr in &input.attrs {
            if !attr.path().is_ident("nbt") {
                continue;
            }

            let meta = &attr.meta;
            let Meta::List(list) = meta else {
                continue;
            };

            list.parse_nested_meta(|nested_meta| {
                let name = nested_meta
                    .path
                    .get_ident()
                    .expect("Expected an identifier");

                match name.to_string().as_str() {
                    "tag" => {
                        let tag = nested_meta
                            .value()
                            .expect("Expected tag to have a value");
                        let tag = tag
                            .parse::<LitStr>()
                            .expect("Expected tag to be a string");
                        attributes.push(NbtFieldAttribute::Tag {
                            tag: tag.value(),
                        });
                    }
                    "tag_type" => {
                        let tag = nested_meta
                            .value()
                            .expect("Expected tag to have a value");
                        let tag = tag
                            .parse::<LitInt>()
                            .expect("Expected tag to be a string");
                        attributes.push(NbtFieldAttribute::TagType {
                            tag: tag.base10_parse::<u8>().expect("Not a valid u8"),
                        });
                    }
                    "content" => {
                        let content = nested_meta
                            .value()
                            .expect("Expected contenf to have a value");
                        let content = content
                            .parse::<LitStr>()
                            .expect("Expected content to be a string");
                        attributes.push(NbtFieldAttribute::Content {
                            content: content.value(),
                        });
                    }
                    "rename_all" => {
                        let case = nested_meta
                            .value()
                            .expect("Expected case to have a value");
                        let case: Cases = case
                            .parse::<LitStr>()
                            .expect("Expected case to be a string").value().into();
                        attributes.push(NbtFieldAttribute::RenameAll {
                            case: case
                        });
                    }
                    _ => panic!("Unknown attribute: {}", name),
                }

                Ok(())
            })
            .expect("Failed to parse nested meta");
        }

        attributes
    }

    pub fn from_variant(variant: &Variant) -> Vec<NbtFieldAttribute> {
        let mut attributes = Vec::new();

        for attr in &variant.attrs {
            if !attr.path().is_ident("nbt") {
                continue;
            }

            let meta = &attr.meta;
            let Meta::List(list) = meta else {
                continue;
            };

            list.parse_nested_meta(|nested_meta| {
                let name = nested_meta
                    .path
                    .get_ident()
                    .expect("Expected an identifier");

                match name.to_string().as_str() {
                    "rename" => {
                        let rename = nested_meta
                            .value()
                            .expect("Expected rename to have a value");
                        let rename = rename
                            .parse::<LitStr>()
                            .expect("Expected rename to be a string");
                        attributes.push(NbtFieldAttribute::Rename {
                            new_name: rename.value(),
                        });
                    }
                    _ => panic!("Unknown attribute: {}", name),
                }

                Ok(())
            })
            .expect("Failed to parse nested meta");
        }

        attributes
    }

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
                let name = nested_meta
                    .path
                    .get_ident()
                    .expect("Expected an identifier");

                match name.to_string().as_str() {
                    "rename" => {
                        let rename = nested_meta
                            .value()
                            .expect("Expected rename to have a value");
                        let rename = rename
                            .parse::<LitStr>()
                            .expect("Expected rename to be a string");
                        attributes.push(NbtFieldAttribute::Rename {
                            new_name: rename.value(),
                        });
                    }
                    "skip_if" => {
                        let skip_if = nested_meta
                            .value()
                            .expect("Expected skip_if to have a value");
                        let skip_if = skip_if
                            .parse::<LitStr>()
                            .expect("Expected skip_if to be a string");
                        attributes.push(NbtFieldAttribute::SkipIf {
                            condition: skip_if.value(),
                        });
                    }
                    "skip" => {
                        attributes.push(NbtFieldAttribute::Skip);
                    }
                    "flatten" => {
                        attributes.push(NbtFieldAttribute::Flatten);
                    }
                    _ => panic!("Unknown attribute: {}", name),
                }

                Ok(())
            })
            .expect("Failed to parse nested meta");
        }

        let optional = is_field_type_optional(field);

        if optional {
            attributes.push(NbtFieldAttribute::Optional);
        }

        attributes
    }
}

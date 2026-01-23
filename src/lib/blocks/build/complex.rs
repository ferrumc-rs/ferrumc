use crate::{BuildConfig, ComplexBlock, SingleOrMultiple};
use ferrumc_block_properties::{PropertyDescriptor, TYPES};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashMap;
use heck::ToPascalCase;

struct BlockStateConfiguration {
    name: String,
    properties: Vec<(String, String)>,
    values: HashMap<u32, HashMap<String, String>>,
}

pub fn generate_complex_blocks(config: &BuildConfig, block_states: Vec<(u32, ComplexBlock)>) -> TokenStream {
    let mut missing_types = config.property_types
        .values()
        .flatten()
        .filter(|str| !TYPES.iter().any(|(name, _)| name == &str.as_str()))
        .collect::<Vec<_>>();

    missing_types.sort();
    missing_types.dedup();

    if !missing_types.is_empty() {
        panic!("Missing types for {:?}", missing_types);
    }

    let mut configurations: HashMap<String, HashMap<u32, HashMap<String, String>>> = HashMap::new();

    for (id, block) in block_states.into_iter() {
        let entry = configurations.entry(block.name).or_insert_with(HashMap::new);

        entry.insert(id, block.properties);
    }

    let blocks = configurations
        .into_iter()
        .map(|(name, properties)| {
            let mut property_values = HashMap::new();

            for (_, properties) in properties.iter() {
                for (name, value) in properties.iter() {
                    let possible_types = match config.property_types.get(name) {
                        None => panic!("Property type for {} not found!", name),
                        Some(SingleOrMultiple::Single(ty)) => vec![ty.to_string()],
                        Some(SingleOrMultiple::Multiple(types)) => types.clone(),
                    };

                    let entry = property_values.entry(name.to_string()).or_insert_with(|| (possible_types, Vec::new()));

                    entry.1.push(value.to_string());
                }
            }

            let mut property_map = Vec::new();

            for (name, (possible_types, values)) in property_values {
                let property_type = possible_types
                    .into_iter()
                    .find(|ty| values.iter().all(|value| (property_descriptor_of(ty.as_str()).matches_values)(value.as_str())))
                    .unwrap_or_else(|| panic!("Failed to find property type for values {values:?}"));

                property_map.push((name, property_type));
            }

            property_map.sort();

            BlockStateConfiguration {
                name,
                properties: property_map,
                values: properties
            }
        })
        .collect::<Vec<_>>();

    let mut configs = blocks
        .iter()
        .map(|block| &block.properties)
        .collect::<Vec<_>>();

    configs.sort();
    configs.dedup();

    let mut structs = Vec::with_capacity(blocks.len());

    for config in configs {
        let blocks = blocks
            .iter()
            .filter(|block| {
                &block.properties == config
            })
            .collect::<Vec<_>>();

        let field = config.iter().map(|(name, _)| {
            if name == "type" {
                format_ident!("ty")
            } else {
                format_ident!("{}", name)
            }
        }).collect::<Vec<_>>();
        let types = config.iter().map(|(_, value)| format_ident!("{}", value)).collect::<Vec<_>>();

        let values = blocks.iter().map(|block| (&block.name, &block.values)).collect::<Vec<_>>();

        match blocks.len() {
            0 => continue,
            1 => {
                let block = blocks[0];

                let struct_name = format_ident!("GeneratedBlock_{}", block.name.strip_prefix("minecraft:").unwrap_or(block.name.as_str()).to_pascal_case());

                let trait_impl = generate_trait_impls(&struct_name, None, &blocks);

                structs.push(quote! {
                    #[allow(dead_code, non_camel_case_types)]
                    pub struct #struct_name {
                        #(
                            pub #field: #types,
                        )*
                    }

                    #trait_impl
                });
            },
            _ => {
                let mut variants = Vec::with_capacity(blocks.len());
                for BlockStateConfiguration { name, .. } in &blocks {
                    let variant = format_ident!("{}", name.strip_prefix("minecraft:").unwrap_or(name.as_str()).to_pascal_case());
                    variants.push(variant);
                }

                variants.sort();

                let struct_name = format_ident!("GeneratedBlock_{}", structs.len());
                let enum_name = format_ident!("{}_BlockType", struct_name);

                let trait_impl = generate_trait_impls(&struct_name, Some((&enum_name, &variants)), &blocks);

                structs.push(quote! {
                    #[allow(dead_code, non_camel_case_types)]
                    pub enum #enum_name {
                        #(
                            #variants,
                        )*
                    }

                    #[allow(dead_code, non_camel_case_types)]
                    pub struct #struct_name {
                        pub block_type: #enum_name,
                        #(
                            pub #field: #types,
                        )*
                    }

                    #trait_impl
                })
            }
        }
    }

    quote! {
        use ferrumc_block_properties::*;
        #(#structs)*
    }
}

fn property_descriptor_of(key: &str) -> &PropertyDescriptor {
    &TYPES
        .iter()
        .find(|(name, _)| name == &key)
        .unwrap_or_else(|| panic!("Property type for {} not found!", key))
        .1
}

fn generate_trait_impls(struct_name: &Ident, enum_name: Option<(&Ident, &[Ident])>, values: &[&BlockStateConfiguration]) -> TokenStream {
    let mut from_match_arms = TokenStream::new();
    let mut into_match_arms = TokenStream::new();

    match enum_name {
        Some(enum_name) => {

        },
        None => {
            let BlockStateConfiguration { properties, values, .. } = values[0];

            let mut values = values.into_iter().collect::<Vec<_>>();
            values.sort_by(|(id_a, _), (id_b, _)| id_a.cmp(id_b));

            for (id, values) in values {
                let fields = values.keys().map(|str| {
                    if str == "type" {
                        format_ident!("ty")
                    } else {
                        format_ident!("{str}")
                    }
                }).collect::<Vec<_>>();
                let values = values.iter().map(|(name, value)| {
                    let ty = &properties.iter().find(|(name1, _)| name == name1).unwrap().1;
                    (property_descriptor_of(ty.as_str()).ident_for)(value.as_str())
                }).collect::<Vec<_>>();

                from_match_arms.extend(quote! {
                    #id => Ok(#struct_name { #(#fields: #values),* }),
                });

                into_match_arms.extend(quote! {
                    #struct_name { #(#fields: #values),* } => Ok(#id),
                });
            }
        }
    }

    quote! {
        impl TryFrom<u32> for #struct_name {
            type Error = ();

            fn try_from(value: u32) -> Result<Self, Self::Error> {
                match value {
                    #from_match_arms
                    _ => Err(())
                }
            }
        }

        impl TryInto<u32> for #struct_name {
            type Error = ();

            fn try_into(self) -> Result<u32, Self::Error> {
                #[allow(unreachable_patterns)]
                match self {
                    #into_match_arms
                    _ => Err(())
                }
            }
        }
    }
}
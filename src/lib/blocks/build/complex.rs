use crate::{BuildConfig, ComplexBlock, SingleOrMultiple};
use ferrumc_block_properties::{PropertyDescriptor, TYPES};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;
use heck::ToPascalCase;

struct BlockStateConfiguration {
    name: String,
    properties: HashMap<String, String>,
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

            let mut property_map = HashMap::new();

            for (name, (possible_types, values)) in property_values {
                let property_type = possible_types
                    .into_iter()
                    .find(|ty| values.iter().all(|value| (property_descriptor_of(ty.as_str()).matches_values)(value.as_str())))
                    .unwrap_or_else(|| panic!("Failed to find property type for values {values:?}"));

                property_map.insert(name, property_type);
            }

            BlockStateConfiguration {
                name,
                properties: property_map,
                values: properties
            }
        })
        .collect::<Vec<_>>();

    let mut structs = Vec::with_capacity(blocks.len());

    for BlockStateConfiguration { name, properties, .. } in &blocks {
        let struct_name = format_ident!("GeneratedBlock_{}", name.strip_prefix("minecraft:").unwrap_or(name.as_str()).to_pascal_case());

        let field = properties.keys().into_iter().map(|name| {
            if name == "type" {
                format_ident!("ty")
            } else {
                format_ident!("{}", name)
            }
        }).collect::<Vec<_>>();
        let types = properties.values().into_iter().map(|properties| format_ident!("{}", properties)).collect::<Vec<_>>();

        structs.push(quote! {
            #[allow(non_camel_case_types, dead_code)]
            pub struct #struct_name {
                #(
                    #field: #types,
                )*
            }
        });
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
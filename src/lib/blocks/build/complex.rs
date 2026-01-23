use crate::{BuildConfig, ComplexBlock, SingleOrMultiple};
use ferrumc_block_properties::{PropertyDescriptor, TYPES};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

struct BlockStateConfiguration {
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

    for (_, properties) in configurations.iter() {
        for (_, properties) in properties.iter() {
            for (name, _) in properties.iter() {
                match config.property_types.get(name) {
                    None => {
                        panic!("Property type for {} not found!", name)
                    },
                    Some(SingleOrMultiple::Single(property)) => {
                        (property_descriptor_of(property.as_str()).print_name)()
                    },
                    _ => {}
                }
            }
        }
    }

    quote! {
        // TODO
    }
}

fn property_descriptor_of(key: &str) -> &PropertyDescriptor {
    &TYPES
        .iter()
        .find(|(name, _)| name == &key)
        .unwrap_or_else(|| panic!("Property type for {} not found!", key))
        .1
}
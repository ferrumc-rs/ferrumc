use crate::{format_code, BuildConfig, ComplexBlock, SingleOrMultiple};
use ferrumc_block_properties::{PropertyDescriptor, TYPES};
use heck::{ToPascalCase, ToSnakeCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashMap;
use std::fs;

struct BlockStateConfiguration {
    name: String,
    properties: Vec<(String, String)>,
    values: HashMap<u32, HashMap<String, String>>,
}

pub fn generate_complex_blocks(build_config: &BuildConfig, block_states: Vec<(u32, ComplexBlock)>) -> TokenStream {
    let mut missing_types = build_config.property_types
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
                    let possible_types = match build_config.property_types.get(name) {
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

        let struct_name = struct_name(build_config, structs.len(), config.as_slice());

        match blocks.len() {
            0 => continue,
            1 => {
                let trait_impl = generate_trait_impls(&struct_name, None, &blocks);

                structs.push((struct_name.clone(), quote! {
                    #[allow(dead_code)]
                    pub struct #struct_name {
                        #(
                            pub #field: #types,
                        )*
                    }

                    #trait_impl
                }));
            },
            _ => {
                let mut variants = Vec::with_capacity(blocks.len());
                for BlockStateConfiguration { name, .. } in &blocks {
                    let variant = format_ident!("{}", name.strip_prefix("minecraft:").unwrap_or(name.as_str()).to_pascal_case());
                    variants.push(variant);
                }

                variants.sort();

                let enum_name = format_ident!("{}Type", struct_name);

                let trait_impl = generate_trait_impls(&struct_name, Some((&enum_name, &variants)), &blocks);

                structs.push((struct_name.clone(), quote! {
                    #[allow(dead_code)]
                    pub enum #enum_name {
                        #(
                            #variants,
                        )*
                    }

                    #[allow(dead_code)]
                    pub struct #struct_name {
                        pub block_type: #enum_name,
                        #(
                            pub #field: #types,
                        )*
                    }

                    #trait_impl
                }));
            }
        }
    }

    let mut modules = TokenStream::new();

    fs::remove_dir_all("src/blocks").unwrap();

    for (struct_name, tokens) in structs {
        let name = format_ident!("{}", struct_name.to_string().to_snake_case());
        let tokens = quote! {
            #[allow(unused_imports)]
            use ferrumc_block_properties::*;

            #tokens
        };

        fs::create_dir_all("src/blocks").unwrap();
        fs::write(format!("src/blocks/{name}.rs"), format_code(&tokens.to_string())).unwrap();

        modules.extend(quote! {
            mod #name;

            pub use #name::*;
        });
    }

    quote! {
        #modules
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
    let (from_match_arms, into_match_arms): (Vec<TokenStream>, Vec<TokenStream>) = match enum_name {
        Some((enum_name, enum_variants)) => {
            let mut values = values.into_iter().collect::<Vec<_>>();
            values.sort_by_key(|BlockStateConfiguration { name, .. }| name);

            values.iter()
                .map(|BlockStateConfiguration { name, properties, values }| {
                    let name_ident = format_ident!("{}", name.strip_prefix("minecraft:").unwrap_or(name.as_str()).to_pascal_case());

                    match enum_variants.iter().find(|variant| *variant == &name_ident) {
                        Some(variant) => {
                            let mut values = values.into_iter().collect::<Vec<_>>();
                            values.sort_by(|(id_a, _), (id_b, _)| id_a.cmp(id_b));

                            values.into_iter()
                                .map(move |(id, values)| {
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

                                    (
                                        quote! {
                                            #id => Ok(#struct_name { block_type: #enum_name::#variant, #(#fields: #values),* })
                                        },
                                        quote! {
                                            #struct_name { block_type: #enum_name::#variant, #(#fields: #values),* } => Ok(#id)
                                        }
                                    )
                                })
                        },
                        None => panic!("could not find {} enum variant for {}", enum_name, name),
                    }
                })
                .flatten()
                .unzip()
        },
        None => {
            let BlockStateConfiguration { properties, values, .. } = values[0];

            let mut values = values.into_iter().collect::<Vec<_>>();
            values.sort_by_key(|(id, _)| *id);

            values.into_iter()
                .map(|(id, values)| {
                    let fields = values.keys().map(|str| {
                        match str.as_str() {
                            "type" => format_ident!("ty"),
                            str => format_ident!("{str}"),
                        }
                    }).collect::<Vec<_>>();
                    let values = values.iter().map(|(name, value)| {
                        let ty = &properties.iter().find(|(name1, _)| name == name1).unwrap().1;
                        (property_descriptor_of(ty.as_str()).ident_for)(value.as_str())
                    }).collect::<Vec<_>>();

                    (
                        quote! {
                            #id => Ok(#struct_name { #(#fields: #values),* })
                        },
                        quote! {
                            #struct_name { #(#fields: #values),* } => Ok(#id)
                        }
                    )
                })
                .unzip()
        }
    };

    quote! {
        impl TryFrom<u32> for #struct_name {
            type Error = ();

            fn try_from(value: u32) -> Result<Self, Self::Error> {
                match value {
                    #(#from_match_arms),*,
                    _ => Err(())
                }
            }
        }

        impl TryInto<u32> for #struct_name {
            type Error = ();

            fn try_into(self) -> Result<u32, Self::Error> {
                #[allow(unreachable_patterns)]
                match self {
                    #(#into_match_arms),*,
                    _ => Err(())
                }
            }
        }
    }
}

fn struct_name(config: &BuildConfig, num_structs: usize, properties: &[(String, String)]) -> Ident {
    let prop_str = properties.into_iter().map(|(name, _)| name).cloned().collect::<Vec<_>>();
    let prop_str = prop_str.join("+");

    match config.name_overrides.iter().find(|(key, _)| {
        let mut split = key.split("+").map(|str| str.to_string()).collect::<Vec<_>>();
        split.sort();
        let new_key = split.join("+");

        &new_key == &prop_str
    }) {
        None => format_ident!("GeneratedStruct{}", num_structs),
        Some((_, struct_name)) => format_ident!("{struct_name}"),
    }
}
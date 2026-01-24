use ferrumc_block_properties::{PropertyDescriptor, TYPES};
use heck::{ToPascalCase, ToShoutySnakeCase, ToSnakeCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashMap;
use crate::config::{BuildConfig, SingleOrMultiple};

struct BlockStateConfiguration<'a> {
    name: &'a str,
    properties: Vec<(&'a str, &'a str)>,
    values: HashMap<u32, &'a HashMap<String, String>>,
}

pub struct ComplexBlock {
    pub name: String,
    pub properties: HashMap<String, String>,
}

pub fn fill_complex_block_mappings(build_config: &BuildConfig, block_states: Vec<(u32, ComplexBlock)>, mappings: &mut [TokenStream]) -> Vec<TokenStream> {
    let (blocks, configs) = dedup_blocks(build_config, &block_states);

    configs
        .into_iter()
        .enumerate()
        .map(|(i, config)| {
            let struct_name = struct_name(build_config, i, &config);
            let vtable_name = format_ident!("VTABLE_{}", struct_name.to_string().to_shouty_snake_case());

            blocks
                .iter()
                .filter(|block| &block.properties == &config)
                .for_each(|block| {
                    block
                        .values
                        .iter()
                        .for_each(|(id, _)| {
                            mappings[*id as usize] = quote! { crate::StateBehaviorTable::spin_off(&#vtable_name, #id) }
                        });
                });

            quote! {
                const #vtable_name: crate::BlockBehaviorTable = crate::BlockBehaviorTable::from::<#struct_name>();
            }
        })
        .collect()
}

pub fn generate_complex_blocks(build_config: &BuildConfig, block_states: Vec<(u32, ComplexBlock)>) -> (Vec<((TokenStream, TokenStream), Ident)>, TokenStream) {
    let (blocks, configs) = dedup_blocks(build_config, &block_states);

    let ((structs, impls), names): ((Vec<TokenStream>, Vec<TokenStream>), Vec<Ident>) = configs
        .into_iter()
        .enumerate()
        .map(|(i, config)| {
            let blocks = blocks
                .iter()
                .filter(|block| &block.properties == &config)
                .collect::<Vec<_>>();

            let fields = config
                .iter()
                .map(|(name, _)| {
                    match *name {
                        "type" => format_ident!("ty"),
                        str => format_ident!("{str}"),
                    }
                })
                .collect::<Vec<_>>();

            let types = config
                .iter()
                .map(|(_, value)| format_ident!("{value}"))
                .collect::<Vec<_>>();

            let struct_name = struct_name(build_config, i, &config);

            match blocks.len() {
                0 => panic!("No blocks for {struct_name}"),
                1 => {
                    let trait_impl = generate_trait_impls(&struct_name, None, &blocks);

                    ((
                        quote! {
                            #[allow(unused_imports)]
                            use ferrumc_block_properties::*;

                            #[allow(dead_code)]
                            #[derive(Clone, Debug)]
                            pub struct #struct_name {
                                #(pub #fields: #types,)*
                            }
                        },
                        quote! {
                            #[allow(unused_imports)]
                            use ferrumc_block_properties::*;
                            use crate::#struct_name;

                            #trait_impl
                        }
                    ), struct_name)
                },
                _ => {
                    let mut variants = blocks
                        .iter()
                        .map(|BlockStateConfiguration { name, .. }| {
                            format_ident!("{}", name.strip_prefix("minecraft:").unwrap_or(name).to_pascal_case())
                        })
                        .collect::<Vec<_>>();

                    variants.sort();

                    let enum_name = format_ident!("{}Type", struct_name);
                    let trait_impl = generate_trait_impls(&struct_name, Some((&enum_name, &variants)), &blocks);

                    ((
                        quote! {
                            #[allow(unused_imports)]
                            use ferrumc_block_properties::*;

                            #[allow(dead_code)]
                            #[derive(Clone, Debug)]
                            pub enum #enum_name {
                                #(#variants,)*
                            }

                            #[allow(dead_code)]
                            #[derive(Clone, Debug)]
                            pub struct #struct_name {
                                pub block_type: #enum_name,
                                #(pub #fields: #types),*
                            }
                        },
                        quote! {
                            #[allow(unused_imports)]
                            use ferrumc_block_properties::*;
                            use crate::#struct_name;
                            use crate::#enum_name;

                            #trait_impl
                        }
                    ), struct_name)
                }
            }
        })
        .unzip();

    let mod_names = names
        .iter()
        .map(|name| format_ident!("{}", name.to_string().to_snake_case()))
        .collect::<Vec<_>>();

    let impl_names = mod_names
        .iter()
        .map(|name| format_ident!("{}_impl", name))
        .collect::<Vec<_>>();

    let modules = quote! {
        #(
            mod #mod_names;
            mod #impl_names;
            pub use #mod_names::*;
        )*
    };

    let data = structs
        .into_iter()
        .zip(impls.into_iter())
        .zip(names.into_iter())
        .collect::<Vec<_>>();

    (
        data,
        modules
    )
}

fn property_descriptor_of(key: &str) -> &PropertyDescriptor {
    &TYPES
        .get(key)
        .unwrap_or_else(|| panic!("Property type for {key} not found!"))
}

fn generate_trait_impls(struct_name: &Ident, enum_name: Option<(&Ident, &[Ident])>, values: &[&BlockStateConfiguration]) -> TokenStream {
    fn get_field_values(values: &[(&String, &String)], properties: &[(&str, &str)]) -> (Vec<Ident>, Vec<TokenStream>) {
        values
            .into_iter()
            .map(|(field, value)| {
                (
                    match field.as_str() {
                        "type" => format_ident!("ty"),
                        field => format_ident!("{field}"),
                    },
                    {
                        let ty = &properties.iter().find(|(name1, _)| name1 == field).unwrap().1;
                        (property_descriptor_of(*ty).ident_for)(value)
                    }
                )
            })
            .unzip()
    }

    let (from_match_arms, into_match_arms): (Vec<TokenStream>, Vec<TokenStream>) = match enum_name {
        Some((enum_name, enum_variants)) => {
            let mut values = values.into_iter().collect::<Vec<_>>();
            values.sort_by_key(|BlockStateConfiguration { name, .. }| name);

            values.iter()
                .map(|BlockStateConfiguration { name, properties, values }| {
                    let name_ident = format_ident!("{}", name.strip_prefix("minecraft:").unwrap_or(name).to_pascal_case());

                    match enum_variants.iter().find(|variant| *variant == &name_ident) {
                        Some(variant) => {
                            let mut values = values.into_iter().collect::<Vec<_>>();
                            values.sort_by(|(id_a, _), (id_b, _)| id_a.cmp(id_b));

                            let mut out = Vec::with_capacity(values.len());

                            for (id, values) in values {
                                let mut values = values
                                    .into_iter()
                                    .collect::<Vec<_>>();

                                values.sort_by_key(|(field, _)| field.as_str());

                                let (fields, values) = get_field_values(&values, &properties);

                                let data = quote! {
                                    #struct_name { block_type: #enum_name::#variant, #(#fields: #values),* }
                                };

                                out.push((
                                    quote! {
                                        #id => Ok(#data)
                                    },
                                    quote! {
                                        #data => Ok(#id)
                                    }
                                ))
                            }

                            out
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
                    let mut values = values
                        .into_iter()
                        .collect::<Vec<_>>();

                    values.sort_by_key(|(field, _)| field.as_str());

                    let (fields, values) = get_field_values(&values, &properties);

                    let data = quote! {
                        #struct_name { #(#fields: #values),* }
                    };

                    (
                        quote! {
                            #id => Ok(#data)
                        },
                        quote! {
                            #data => Ok(#id)
                        }
                    )
                })
                .unzip()
        }
    };

    quote! {
        impl TryFrom<u32> for #struct_name {
            type Error = ();

            fn try_from(data: u32) -> Result<Self, Self::Error> {
                match data {
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

fn struct_name(config: &BuildConfig, num_structs: usize, properties: &[(&str, &str)]) -> Ident {
    let prop_str = properties.iter().map(|(name, _)| *name).collect::<Vec<_>>();
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

fn dedup_blocks<'a>(config: &'a BuildConfig, block_states: &'a [(u32, ComplexBlock)]) -> (Vec<BlockStateConfiguration<'a>>, Vec<Vec<(&'a str, &'a str)>>) {
    let mut missing_types = config.property_types
        .values()
        .flatten()
        .filter(|str| !TYPES.contains_key(str.as_str()))
        .collect::<Vec<_>>();

    missing_types.sort();
    missing_types.dedup();

    if !missing_types.is_empty() {
        panic!("Missing types for {:?}", missing_types);
    }

    let configurations: HashMap<&str, HashMap<u32, &HashMap<String, String>>> = block_states
        .into_iter()
        .map(|(id, block)| {
            (block.name.as_str(), (*id, &block.properties))
        })
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.entry(k).or_insert_with(HashMap::new).insert(v.0, v.1);
            acc
        });

    let blocks = configurations
        .into_iter()
        .map(|(name, properties)| {
            let property_values = properties
                .iter()
                .map(|(_, properties)| {
                    properties
                        .into_iter()
                        .map(|(name, value)| {
                            let name = name.as_str();
                            let value = value.as_str();
                            let possible_types: Vec<&str> = match config.property_types.get(name) {
                                None => panic!("Property type for {name} not found!"),
                                Some(SingleOrMultiple::Single(ty)) => vec![ty.as_str()],
                                Some(SingleOrMultiple::Multiple(ty)) => ty.into_iter().map(String::as_str).collect(),
                            };

                            (name, (possible_types, value))
                        })
                })
                .flatten()
                .fold(HashMap::new(), |mut acc, (k, v)| {
                    acc
                        .entry(k)
                        .or_insert_with(|| (v.0, Vec::new()))
                        .1
                        .push(v.1);

                    acc
                });

            let mut property_map: Vec<(&str, &str)> = property_values
                .into_iter()
                .map(|(name, (possible_types, values))| {
                    let property_type = possible_types
                        .into_iter()
                        .find(|ty| values.iter().all(|value| (property_descriptor_of(ty).matches_values)(value)))
                        .unwrap_or_else(|| panic!("Failed to find property type for values {values:?}"));

                    (name, property_type)
                })
                .collect();

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
        .map(|block| block.properties.to_vec())
        .collect::<Vec<_>>();

    configs.sort();
    configs.dedup();

    (blocks, configs)
}
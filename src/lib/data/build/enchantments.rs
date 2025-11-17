use heck::ToShoutySnakeCase;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use syn::{LitFloat, LitInt};

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct Enchantment {
    pub id: u16,
    pub description: Description,
    pub min_cost: Cost,
    pub max_cost: Cost,
    pub anvil_cost: u8,
    pub slots: Vec<String>,
    pub supported_items: String,
    pub weight: u8,
    pub max_level: u8,
    #[serde(default)]
    pub exclusive_set: Option<String>,
    #[serde(default)]
    pub effects: serde_json::Value,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Description {
    pub translate: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Cost {
    pub base: f32,
    pub per_level_above_first: f32,
}

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=../../../assets/extracted/enchantments.json");

    let enchantments: BTreeMap<String, Enchantment> = serde_json::from_str(
        &fs::read_to_string("../../../assets/extracted/enchantments.json").unwrap(),
    )
    .expect("Failed to parse enchantments.json");

    let mut constants = TokenStream::new();
    let mut type_from_id_arms = TokenStream::new();
    let mut type_from_name = TokenStream::new();

    for (name, enchantment) in enchantments.iter() {
        let const_ident = format_ident!(
            "{}",
            name.strip_prefix("minecraft:")
                .unwrap_or(name)
                .to_shouty_snake_case()
        );
        let id_lit = LitInt::new(&enchantment.id.to_string(), Span::call_site());

        let translate = &enchantment.description.translate;
        let min_cost_base = LitFloat::new(
            &format!("{:.1}", enchantment.min_cost.base),
            Span::call_site(),
        );
        let min_cost_per_level = LitFloat::new(
            &format!("{:.1}", enchantment.min_cost.per_level_above_first),
            Span::call_site(),
        );
        let max_cost_base = LitFloat::new(
            &format!("{:.1}", enchantment.max_cost.base),
            Span::call_site(),
        );
        let max_cost_per_level = LitFloat::new(
            &format!("{:.1}", enchantment.max_cost.per_level_above_first),
            Span::call_site(),
        );
        let anvil_cost = LitInt::new(&enchantment.anvil_cost.to_string(), Span::call_site());
        let weight = LitInt::new(&enchantment.weight.to_string(), Span::call_site());
        let max_level = LitInt::new(&enchantment.max_level.to_string(), Span::call_site());

        let supported_items = &enchantment.supported_items;

        let slots = enchantment
            .slots
            .iter()
            .map(|slot| {
                let slot_str = format_ident!("{}", slot.to_uppercase());
                quote! { EnchantmentSlot::#slot_str }
            })
            .collect::<Vec<_>>();

        let exclusive_set = match &enchantment.exclusive_set {
            Some(set) => {
                quote! { Some(#set) }
            }
            None => quote! { None },
        };

        constants.extend(quote! {
            pub const #const_ident: Enchantment = Enchantment {
                id: #id_lit,
                name: #name,
                description: #translate,
                min_cost: Cost {
                    base: #min_cost_base,
                    per_level_above_first: #min_cost_per_level,
                },
                max_cost: Cost {
                    base: #max_cost_base,
                    per_level_above_first: #max_cost_per_level,
                },
                anvil_cost: #anvil_cost,
                slots: &[#(#slots),*],
                supported_items: #supported_items,
                weight: #weight,
                max_level: #max_level,
                exclusive_set: #exclusive_set,
            };
        });

        type_from_id_arms.extend(quote! {
            #id_lit => Some(&Self::#const_ident),
        });

        type_from_name.extend(quote! {
            #name => Some(&Self::#const_ident),
        });
    }

    quote! {
        use std::collections::HashMap;

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct Enchantment {
            pub id: u16,
            pub name: &'static str,
            pub description: &'static str,
            pub min_cost: Cost,
            pub max_cost: Cost,
            pub anvil_cost: u8,
            pub slots: &'static [EnchantmentSlot],
            pub supported_items: &'static str,
            pub weight: u8,
            pub max_level: u8,
            pub exclusive_set: Option<&'static str>,
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct Cost {
            pub base: f32,
            pub per_level_above_first: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum EnchantmentSlot {
            MAINHAND,
            OFFHAND,
            HEAD,
            CHEST,
            LEGS,
            FEET,
            ARMOR,
            ANY,
            HAND,
        }

        impl Enchantment {
            #constants

            #[doc = r" Try to parse an `Enchantment` from a resource location string."]
            pub fn from_name(name: &str) -> Option<&'static Self> {
                let name = name.strip_prefix("minecraft:").unwrap_or(name);
                match name {
                    #type_from_name
                    _ => None
                }
            }

            #[doc = r" Try to get an `Enchantment` from its ID."]
            pub const fn from_id(id: u16) -> Option<&'static Self> {
                match id {
                    #type_from_id_arms
                    _ => None
                }
            }

            #[doc = r" Calculate the minimum cost for this enchantment at the given level."]
            pub const fn min_cost(&self, level: u8) -> f32 {
                self.min_cost.base + self.min_cost.per_level_above_first * (level - 1) as f32
            }

            #[doc = r" Calculate the maximum cost for this enchantment at the given level."]
            pub const fn max_cost(&self, level: u8) -> f32 {
                self.max_cost.base + self.max_cost.per_level_above_first * (level - 1) as f32
            }
        }
    }
}

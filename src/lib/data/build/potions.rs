use std::collections::BTreeMap;
use std::fs;
use heck::ToShoutySnakeCase;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use serde::Deserialize;
use syn::{LitBool, LitInt};

#[derive(Deserialize, Clone, Debug)]
pub struct Potion {
    pub id: u16,
    pub base_name: String,
    pub effects: Vec<PotionEffect>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PotionEffect {
    pub effect_type: String,
    pub duration: i32,
    pub amplifier: u8,
    pub ambient: bool,
    pub show_particles: bool,
    pub show_icon: bool,
}

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=../../../assets/extracted/potion.json");

    let potions: BTreeMap<String, Potion> =
        serde_json::from_str(&fs::read_to_string("../../../assets/extracted/potion.json").unwrap())
            .expect("Failed to parse potion.json");

    let mut constants = TokenStream::new();
    let mut type_from_id_arms = TokenStream::new();
    let mut type_from_name = TokenStream::new();

    for (name, potion) in potions.iter() {
        let const_ident = format_ident!("{}", name.to_shouty_snake_case());
        let id_lit = LitInt::new(&potion.id.to_string(), Span::call_site());
        let base_name = &potion.base_name;
        
        let effects = potion.effects.iter().map(|effect| {
            let effect_type = &effect.effect_type;
            let duration = LitInt::new(&effect.duration.to_string(), Span::call_site());
            let amplifier = LitInt::new(&effect.amplifier.to_string(), Span::call_site());
            let ambient = LitBool::new(effect.ambient, Span::call_site());
            let show_particles = LitBool::new(effect.show_particles, Span::call_site());
            let show_icon = LitBool::new(effect.show_icon, Span::call_site());

            quote! {
                PotionEffect {
                    effect_type: #effect_type,
                    duration: #duration,
                    amplifier: #amplifier,
                    ambient: #ambient,
                    show_particles: #show_particles,
                    show_icon: #show_icon,
                }
            }
        }).collect::<Vec<_>>();

        constants.extend(quote! {
            pub const #const_ident: Potion = Potion {
                id: #id_lit,
                name: #name,
                base_name: #base_name,
                effects: &[#(#effects),*],
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

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Potion {
            pub id: u16,
            pub name: &'static str,
            pub base_name: &'static str,
            pub effects: &'static [PotionEffect],
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct PotionEffect {
            pub effect_type: &'static str,
            pub duration: i32,
            pub amplifier: u8,
            pub ambient: bool,
            pub show_particles: bool,
            pub show_icon: bool,
        }

        impl Potion {
            #constants

            #[doc = r" Try to parse a `Potion` from a resource location string."]
            pub fn from_name(name: &str) -> Option<&'static Self> {
                let name = name.strip_prefix("minecraft:").unwrap_or(name);
                match name {
                    #type_from_name
                    _ => None
                }
            }

            #[doc = r" Try to get a `Potion` from its ID."]
            pub const fn from_id(id: u16) -> Option<&'static Self> {
                match id {
                    #type_from_id_arms
                    _ => None
                }
            }

            #[doc = r" Check if this potion has any effects."]
            pub const fn has_effects(&self) -> bool {
                !self.effects.is_empty()
            }

            #[doc = r" Get the number of effects this potion has."]
            pub const fn effect_count(&self) -> usize {
                self.effects.len()
            }
        }
    }
}
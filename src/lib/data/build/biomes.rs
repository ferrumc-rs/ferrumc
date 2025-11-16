use std::{collections::BTreeMap, fs};
use heck::ToShoutySnakeCase;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use serde::Deserialize;
use syn::{LitBool, LitFloat, LitInt};
use serde_json::Value;

fn deserialize_carvers<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => Ok(vec![s]),
        Value::Array(arr) => {
            let strings: Result<Vec<String>, _> = arr
                .into_iter()
                .map(|v| v.as_str().map(|s| s.to_string()).ok_or_else(|| serde::de::Error::custom("Expected string")))
                .collect();
            strings
        }
        _ => Err(serde::de::Error::custom("Expected string or array")),
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Biome {
    pub has_precipitation: bool,
    pub temperature: f64,
    pub downfall: f64,
    pub effects: BiomeEffects,
    #[serde(deserialize_with = "deserialize_carvers")]
    pub carvers: Vec<String>,
    pub features: Vec<Vec<String>>,
    #[serde(default)]
    pub creature_spawn_probability: Option<f32>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct BiomeEffects {
    pub mood_sound: Option<MoodSound>,
    pub music: Option<Vec<Music>>,
    #[serde(default)]
    pub music_volume: f32,
    pub sky_color: u32,
    pub foliage_color: Option<u32>,
    pub grass_color: Option<u32>,
    pub fog_color: u32,
    pub water_color: u32,
    pub water_fog_color: u32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MoodSound {
    pub sound: String,
    pub tick_delay: i32,
    pub block_search_extent: u32,
    pub offset: f64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Music {
    pub data: MusicData,
    pub weight: u32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MusicData {
    pub sound: String,
    pub min_delay: i32,
    pub max_delay: i32,
    #[serde(default)]
    pub replace_current_music: bool,
}

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=../../../assets/extracted/biome.json");

    let biomes: BTreeMap<String, Biome> =
        serde_json::from_str(&fs::read_to_string("../../../assets/extracted/biome.json").unwrap())
            .expect("Failed to parse biome.json");

    let mut constants = TokenStream::new();
    let mut type_from_name = TokenStream::new();

    for (name, biome) in biomes.iter() {
        let const_ident = format_ident!("{}", name.to_shouty_snake_case());
        
        let has_precipitation = LitBool::new(biome.has_precipitation, Span::call_site());
        let temperature = LitFloat::new(&format!("{:.1}", biome.temperature), Span::call_site());
        let downfall = LitFloat::new(&format!("{:.1}", biome.downfall), Span::call_site());
        
        let sky_color = LitInt::new(&biome.effects.sky_color.to_string(), Span::call_site());
        let fog_color = LitInt::new(&biome.effects.fog_color.to_string(), Span::call_site());
        let water_color = LitInt::new(&biome.effects.water_color.to_string(), Span::call_site());
        let water_fog_color = LitInt::new(&biome.effects.water_fog_color.to_string(), Span::call_site());
        
        let foliage_color = match biome.effects.foliage_color {
            Some(color) => {
                let color_lit = LitInt::new(&color.to_string(), Span::call_site());
                quote! { Some(#color_lit) }
            }
            None => quote! { None },
        };
        
        let grass_color = match biome.effects.grass_color {
            Some(color) => {
                let color_lit = LitInt::new(&color.to_string(), Span::call_site());
                quote! { Some(#color_lit) }
            }
            None => quote! { None },
        };

        let creature_spawn_probability = match biome.creature_spawn_probability {
            Some(prob) => {
                let prob_lit = LitFloat::new(&prob.to_string(), Span::call_site());
                quote! { Some(#prob_lit) }
            }
            None => quote! { None },
        };

        constants.extend(quote! {
            pub const #const_ident: Biome = Biome {
                name: #name,
                has_precipitation: #has_precipitation,
                temperature: #temperature,
                downfall: #downfall,
                sky_color: #sky_color,
                fog_color: #fog_color,
                water_color: #water_color,
                water_fog_color: #water_fog_color,
                foliage_color: #foliage_color,
                grass_color: #grass_color,
                creature_spawn_probability: #creature_spawn_probability,
            };
        });

        type_from_name.extend(quote! {
            #name => Some(&Self::#const_ident),
        });
    }

    quote! {
        use std::hash::Hash;

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct Biome {
            pub name: &'static str,
            pub has_precipitation: bool,
            pub temperature: f64,
            pub downfall: f64,
            pub sky_color: u32,
            pub fog_color: u32,
            pub water_color: u32,
            pub water_fog_color: u32,
            pub foliage_color: Option<u32>,
            pub grass_color: Option<u32>,
            pub creature_spawn_probability: Option<f32>,
        }

        impl Biome {
            #constants

            #[doc = r" Try to parse a `Biome` from a resource location string."]
            pub fn from_name(name: &str) -> Option<&'static Self> {
                let name = name.strip_prefix("minecraft:").unwrap_or(name);
                match name {
                    #type_from_name
                    _ => None
                }
            }

            #[doc = r" Check if this biome has precipitation (rain/snow)."]
            pub const fn has_precipitation(&self) -> bool {
                self.has_precipitation
            }

            #[doc = r" Get the temperature of this biome."]
            pub const fn temperature(&self) -> f64 {
                self.temperature
            }

            #[doc = r" Get the downfall (humidity) of this biome."]
            pub const fn downfall(&self) -> f64 {
                self.downfall
            }

            #[doc = r" Check if this biome is cold (temperature < 0.2)."]
            pub const fn is_cold(&self) -> bool {
                self.temperature < 0.2
            }

            #[doc = r" Check if this biome is hot (temperature > 1.0)."]
            pub const fn is_hot(&self) -> bool {
                self.temperature > 1.0
            }

            #[doc = r" Check if this biome is wet (downfall > 0.5)."]
            pub const fn is_wet(&self) -> bool {
                self.downfall > 0.5
            }

            #[doc = r" Check if this biome is dry (downfall < 0.2)."]
            pub const fn is_dry(&self) -> bool {
                self.downfall < 0.2
            }
        }
    }
}

use std::fs;
use heck::ToShoutySnakeCase;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use serde::Deserialize;
use syn::{LitInt, LitFloat};
use serde_json::Value;

fn deserialize_pattern<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Null => Ok(None),
        Value::String(s) => Ok(Some(vec![s])),
        Value::Array(arr) => {
            let strings: Result<Vec<String>, _> = arr
                .into_iter()
                .map(|v| v.as_str().map(|s| s.to_string()).ok_or_else(|| serde::de::Error::custom("Expected string")))
                .collect();
            Ok(Some(strings?))
        }
        _ => Err(serde::de::Error::custom("Expected string, array, or null")),
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Recipe {
    #[serde(rename = "type")]
    pub recipe_type: String,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub ingredients: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub key: Option<serde_json::Value>,
    #[serde(default, deserialize_with = "deserialize_pattern")]
    pub pattern: Option<Vec<String>>,
    #[serde(default)]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub experience: Option<f32>,
    #[serde(default)]
    pub cookingtime: Option<u32>,
}

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=../../../assets/extracted/recipes.json");

    let recipes: Vec<Recipe> =
        serde_json::from_str(&fs::read_to_string("../../../assets/extracted/recipes.json").unwrap())
            .expect("Failed to parse recipes.json");

    let mut constants = TokenStream::new();
    let mut type_from_name = TokenStream::new();

    for (index, recipe) in recipes.iter().enumerate() {
        let name = format!("recipe_{}", index);
        let const_ident = format_ident!("{}", name.to_shouty_snake_case());
        
        let recipe_type = &recipe.recipe_type;
        let group = match &recipe.group {
            Some(group) => {
                quote! { Some(#group) }
            }
            None => quote! { None },
        };
        
        let category = match &recipe.category {
            Some(category) => {
                quote! { Some(#category) }
            }
            None => quote! { None },
        };

        let experience = match recipe.experience {
            Some(exp) => {
                let exp_str = if exp.to_string().contains('.') {
                    exp.to_string()
                } else {
                    format!("{}.0", exp)
                };
                let exp_lit = LitFloat::new(&exp_str, Span::call_site());
                quote! { Some(#exp_lit) }
            }
            None => quote! { None },
        };

        let cookingtime = match recipe.cookingtime {
            Some(time) => {
                let time_lit = LitInt::new(&time.to_string(), Span::call_site());
                quote! { Some(#time_lit) }
            }
            None => quote! { None },
        };

        constants.extend(quote! {
            pub const #const_ident: Recipe = Recipe {
                name: #name,
                recipe_type: #recipe_type,
                group: #group,
                category: #category,
                experience: #experience,
                cookingtime: #cookingtime,
            };
        });

        type_from_name.extend(quote! {
            #name => Some(&Self::#const_ident),
        });
    }

    quote! {
        use std::collections::HashMap;

        #[derive(Debug, Clone)]
        pub struct Recipe {
            pub name: &'static str,
            pub recipe_type: &'static str,
            pub group: Option<&'static str>,
            pub category: Option<&'static str>,
            pub experience: Option<f32>,
            pub cookingtime: Option<u32>,
        }

        impl Recipe {
            #constants

            #[doc = r" Try to parse a `Recipe` from a resource location string."]
            pub fn from_name(name: &str) -> Option<&'static Self> {
                let name = name.strip_prefix("minecraft:").unwrap_or(name);
                match name {
                    #type_from_name
                    _ => None
                }
            }

            #[doc = r" Check if this is a crafting recipe."]
            pub const fn is_crafting(&self) -> bool {
                matches!(self.recipe_type, "minecraft:crafting_shaped" | "minecraft:crafting_shapeless")
            }

            #[doc = r" Check if this is a smelting recipe."]
            pub const fn is_smelting(&self) -> bool {
                matches!(self.recipe_type, "minecraft:smelting" | "minecraft:blasting" | "minecraft:smoking" | "minecraft:campfire_cooking")
            }

            #[doc = r" Check if this is a stonecutting recipe."]
            pub const fn is_stonecutting(&self) -> bool {
                self.recipe_type == "minecraft:stonecutting"
            }

            #[doc = r" Check if this is a smithing recipe."]
            pub const fn is_smithing(&self) -> bool {
                self.recipe_type == "minecraft:smithing"
            }
        }
    }
}
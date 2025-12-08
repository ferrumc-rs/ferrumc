use heck::ToShoutySnakeCase;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use serde::{Deserialize, Deserializer};
use std::collections::BTreeMap;
use std::fs;
use syn::{LitFloat, LitInt};

#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrVec {
    String(String),
    Vec(Vec<StringOrVec>),
}

impl StringOrVec {
    fn flatten(self) -> Vec<String> {
        match self {
            StringOrVec::String(s) => vec![s],
            StringOrVec::Vec(items) => items.into_iter().flat_map(|f| f.flatten()).collect(),
        }
    }
}

fn deserialize_pattern<'de, D>(deserializer: D) -> Result<Option<Vec<Vec<String>>>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = StringOrVec::deserialize(deserializer)?;

    Ok(Some(match raw {
        StringOrVec::String(s) => vec![vec![s]],
        StringOrVec::Vec(items) => items.into_iter().map(StringOrVec::flatten).collect(),
    }))
}

fn deserialize_key<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<BTreeMap<String, Vec<String>>>, D::Error> {
    let raw = BTreeMap::<String, StringOrVec>::deserialize(deserializer)?;

    Ok(Some(
        raw.into_iter()
            .map(|(k, v)| match v {
                StringOrVec::String(s) => (k, vec![s]),
                StringOrVec::Vec(v) => (k, v.into_iter().flat_map(StringOrVec::flatten).collect()),
            })
            .collect(),
    ))
}

fn deserialize_ingredient<'de, D: Deserializer<'de>>(
    deserialize: D,
) -> Result<Option<Vec<String>>, D::Error> {
    let raw = Vec::<StringOrVec>::deserialize(deserialize)?;

    Ok(Some(
        raw.into_iter().flat_map(StringOrVec::flatten).collect(),
    ))
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct Recipe {
    #[serde(rename = "type")]
    pub recipe_type: String,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default, deserialize_with = "deserialize_ingredient")]
    pub ingredients: Option<Vec<String>>,
    #[serde(default, deserialize_with = "deserialize_key")]
    pub key: Option<BTreeMap<String, Vec<String>>>,
    #[serde(default, deserialize_with = "deserialize_pattern")]
    pub pattern: Option<Vec<Vec<String>>>,
    #[serde(default)]
    pub result: Option<RecipeResult>,
    #[serde(default)]
    pub experience: Option<f32>,
    #[serde(default)]
    pub cookingtime: Option<u32>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RecipeResult {
    pub id: String,
    pub count: Option<u8>,
}

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=../../../assets/extracted/recipes.json");

    let recipes: Vec<Recipe> = serde_json::from_str(
        &fs::read_to_string("../../../assets/extracted/recipes.json").unwrap(),
    )
    .expect("Failed to parse recipes.json");

    let mut constants = TokenStream::new();
    let mut constant_names = TokenStream::new();
    let mut type_from_name = TokenStream::new();

    for (index, recipe) in recipes.iter().enumerate() {
        let name = format!("recipe_{}", index);
        let const_ident = format_ident!("{}", name.to_shouty_snake_case());

        constant_names.extend(quote! {
            &Self::#const_ident,
        });

        let recipe_type = match recipe.recipe_type.as_str() {
            "minecraft:crafting_shaped" | "crafting_shaped" => {
                quote! { RecipeType::CraftingShaped }
            }
            "minecraft:crafting_shapeless" | "crafting_shapeless" => {
                quote! { RecipeType::CraftingShapeless }
            }
            "minecraft:crafting_special_armordye" | "crafting_special_armordye" => {
                quote! { RecipeType::CraftingSpecialArmorDye }
            }
            "minecraft:crafting_special_bannerduplicate" | "crafting_special_bannerduplicate" => {
                quote! { RecipeType::CraftingSpecialBannerDuplicate }
            }
            "minecraft:crafting_transmute" | "crafting_transmute" => {
                quote! { RecipeType::CraftingTransmute }
            }
            "minecraft:crafting_special_bookcloning" | "crafting_special_bookcloning" => {
                quote! { RecipeType::CraftingSpecialBookCloning }
            }
            "minecraft:crafting_decorated_pot" | "crafting_decorated_pot" => {
                quote! { RecipeType::CraftingDecoratedPot }
            }
            "minecraft:crafting_special_firework_rocket" | "crafting_special_firework_rocket" => {
                quote! { RecipeType::CraftingSpecialFireworkRocket }
            }
            "minecraft:crafting_special_firework_star" | "crafting_special_firework_star" => {
                quote! { RecipeType::CraftingSpecialFireworkStar }
            }
            "minecraft:crafting_special_firework_star_fade"
            | "crafting_special_firework_star_fade" => {
                quote! { RecipeType::CraftingSpecialFireworkStarFade }
            }
            "minecraft:crafting_special_mapcloning" | "crafting_special_mapcloning" => {
                quote! { RecipeType::CraftingSpecialMapCloning }
            }
            "minecraft:crafting_special_mapextending" | "crafting_special_mapextending" => {
                quote! { RecipeType::CraftingSpecialMapExtending }
            }
            "minecraft:crafting_special_repairitem" | "crafting_special_repairitem" => {
                quote! { RecipeType::CraftingSpecialRepairItem }
            }
            "minecraft:crafting_special_shielddecoration" | "crafting_special_shielddecoration" => {
                quote! { RecipeType::CraftingSpecialShieldDecoration }
            }
            "minecraft:crafting_special_tippedarrow" | "crafting_special_tippedarrow" => {
                quote! { RecipeType::CraftingSpecialTippedArrow }
            }
            "minecraft:stonecutting" | "stonecutting" => quote! { RecipeType::Stonecutting },
            "minecraft:smelting" | "smelting" => quote! { RecipeType::Smelting },
            "minecraft:campfire_cooking" | "campfire_cooking" => {
                quote! { RecipeType::CampfireCooking }
            }
            "minecraft:smoking" | "smoking" => quote! { RecipeType::Smoking },
            "minecraft:smithing_trim" | "smithing_trim" => quote! { RecipeType::SmithingTrim },
            "minecraft:smithing_transform" | "smithing_transform" => {
                quote! { RecipeType::SmithingTransform }
            }
            "minecraft:blasting" | "blasting" => quote! { RecipeType::Blasting },
            ty => panic!("unknown recipe type: {ty}"),
        };

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

        let key = match &recipe.key {
            Some(key) => {
                let items = key.iter().map(|(a, b)| {
                    let b_items = b.iter();
                    quote! { (#a, &[#(#b_items),*]) }
                });
                quote! { Some(&[#(#items),*])}
            }
            None => quote! { None },
        };

        let ingredients = match &recipe.ingredients {
            Some(ingredients) => quote! { Some(&[#(#ingredients),*]) },
            None => quote! { None },
        };

        let pattern = match &recipe.pattern {
            Some(pattern) => {
                let items = pattern.iter().map(|a| {
                    quote! {  &[#(#a),*] }
                });
                quote! { Some(&[#(#items),*])}
            }
            None => quote! { None },
        };

        let result = match &recipe.result {
            Some(RecipeResult { id, count }) => {
                let count = count.unwrap_or(1);
                quote! {
                    Some(RecipeResult { id: #id, count: #count })
                }
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
                key: #key,
                ingredients: #ingredients,
                result: #result,
                pattern: #pattern,
            };
        });

        type_from_name.extend(quote! {
            #name => Some(&Self::#const_ident),
        });
    }

    quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub struct Recipe {
            pub name: &'static str,
            pub recipe_type: RecipeType,
            pub group: Option<&'static str>,
            pub category: Option<&'static str>,
            pub experience: Option<f32>,
            pub cookingtime: Option<u32>,
            pub key: Option<&'static [(&'static str, &'static [&'static str])]>,
            pub ingredients: Option<&'static [&'static str]>,
            pub result: Option<RecipeResult>,
            pub pattern: Option<&'static [&'static [&'static str]]>
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RecipeResult {
            pub id: &'static str,
            pub count: u8,
        }

        #[derive(Debug, Copy, Clone, PartialEq)]
        pub enum RecipeType {
            CraftingShaped,
            CraftingShapeless,
            CraftingSpecialArmorDye,
            CraftingSpecialBannerDuplicate,
            CraftingTransmute,
            CraftingSpecialBookCloning,
            CraftingDecoratedPot,
            CraftingSpecialFireworkRocket,
            CraftingSpecialFireworkStar,
            CraftingSpecialFireworkStarFade,
            CraftingSpecialMapCloning,
            CraftingSpecialMapExtending,
            CraftingSpecialRepairItem,
            CraftingSpecialShieldDecoration,
            CraftingSpecialTippedArrow,
            Stonecutting,
            Smelting,
            CampfireCooking,
            Smoking,
            SmithingTrim,
            SmithingTransform,
            Blasting,
        }

        impl Recipe {
            #constants

            pub const ALL_RECIPES: &'static [&'static Recipe] = &[#constant_names];

            #[doc = r" Try to parse a `Recipe` from a resource location string."]
            pub const fn try_from_name(name: &str) -> Option<&'static Self> {
                let name = crate::helpers::strip_prefix_or_self(name, "minecraft:");
                match name {
                    #type_from_name
                    _ => None
                }
            }

            #[doc = r" Check if this is a crafting recipe."]
            pub const fn is_crafting(&self) -> bool {
                matches!(self.recipe_type, RecipeType::CraftingShaped | RecipeType::CraftingShapeless)
            }

            #[doc = r" Check if this is a smelting recipe."]
            pub const fn is_smelting(&self) -> bool {
                matches!(self.recipe_type, RecipeType::Smelting | RecipeType::Blasting | RecipeType::Smoking | RecipeType::CampfireCooking)
            }

            #[doc = r" Check if this is a stonecutting recipe."]
            pub const fn is_stonecutting(&self) -> bool {
                matches!(self.recipe_type, RecipeType::Stonecutting)
            }

            #[doc = r" Check if this is a smithing recipe."]
            pub const fn is_smithing(&self) -> bool {
                matches!(self.recipe_type, RecipeType::SmithingTrim | RecipeType::SmithingTransform)
            }
        }
    }
}

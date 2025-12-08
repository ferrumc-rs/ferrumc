use heck::ToShoutySnakeCase;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use serde::Deserialize;
use std::{collections::BTreeMap, fs};
use syn::LitInt;

#[derive(Deserialize)]
pub struct EntityType {
    pub id: u16,
    pub max_health: Option<f32>,
    pub attackable: Option<bool>,
    pub mob: Option<bool>,
    pub limit_per_chunk: Option<i32>,
    pub summonable: bool,
    pub fire_immune: bool,
    pub saveable: bool,
    pub category: MobCategory,
    pub can_spawn_far_from_player: bool,
    pub dimension: [f32; 2],
    pub eye_height: f32,
    pub spawn_restriction: SpawnRestriction,
}

#[derive(Deserialize)]
pub struct SpawnRestriction {
    location: SpawnLocation,
    heightmap: HeightMap,
}

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpawnLocation {
    InLava,
    InWater,
    OnGround,
    Unrestricted,
}

#[derive(Deserialize)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub enum MobCategory {
    MONSTER,
    CREATURE,
    AMBIENT,
    AXOLOTLS,
    UNDERGROUND_WATER_CREATURE,
    WATER_CREATURE,
    WATER_AMBIENT,
    MISC,
}

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HeightMap {
    WorldSurfaceWg,
    WorldSurface,
    OceanFloorWg,
    OceanFloor,
    MotionBlocking,
    MotionBlockingNoLeaves,
}

pub struct NamedEntityType<'a>(&'a str, &'a EntityType);

impl quote::ToTokens for NamedEntityType<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.0;
        let entity = self.1;
        let id = LitInt::new(&entity.id.to_string(), proc_macro2::Span::call_site());

        let max_health = match entity.max_health {
            Some(mh) => quote! { Some(#mh) },
            None => quote! { None },
        };

        let attackable = match entity.attackable {
            Some(a) => quote! { Some(#a) },
            None => quote! { None },
        };

        let spawn_restriction_location = match entity.spawn_restriction.location {
            SpawnLocation::InLava => quote! {SpawnLocation::InLava},
            SpawnLocation::InWater => quote! {SpawnLocation::InWater},
            SpawnLocation::OnGround => quote! {SpawnLocation::OnGround},
            SpawnLocation::Unrestricted => quote! {SpawnLocation::Unrestricted},
        };

        let spawn_restriction_heightmap = match entity.spawn_restriction.heightmap {
            HeightMap::WorldSurfaceWg => quote! { HeightMap::WorldSurfaceWg },
            HeightMap::WorldSurface => quote! { HeightMap::WorldSurface },
            HeightMap::OceanFloorWg => quote! { HeightMap::OceanFloorWg },
            HeightMap::OceanFloor => quote! { HeightMap::OceanFloor },
            HeightMap::MotionBlocking => quote! { HeightMap::MotionBlocking },
            HeightMap::MotionBlockingNoLeaves => quote! { HeightMap::MotionBlockingNoLeaves },
        };

        let spawn_restriction = quote! { SpawnRestriction {
            location: #spawn_restriction_location,
            heightmap: #spawn_restriction_heightmap,
        }};

        let spawn_category = match entity.category {
            MobCategory::MONSTER => quote! { MobCategory::MONSTER },
            MobCategory::CREATURE => quote! { MobCategory::CREATURE },
            MobCategory::AMBIENT => quote! { MobCategory::AMBIENT },
            MobCategory::AXOLOTLS => quote! { MobCategory::AXOLOTLS },
            MobCategory::UNDERGROUND_WATER_CREATURE => {
                quote! { MobCategory::UNDERGROUND_WATER_CREATURE }
            }
            MobCategory::WATER_CREATURE => quote! { MobCategory::WATER_CREATURE },
            MobCategory::WATER_AMBIENT => quote! { MobCategory::WATER_AMBIENT },
            MobCategory::MISC => quote! { MobCategory::MISC },
        };

        let saveable = entity.saveable;
        let summonable = entity.summonable;
        let fire_immune = entity.fire_immune;
        let eye_height = entity.eye_height;

        let mob = entity.mob.unwrap_or(false);
        let limit_per_chunk = entity.limit_per_chunk.unwrap_or(0);
        let can_spawn_far_from_player = entity.can_spawn_far_from_player;

        let dimension0 = entity.dimension[0];
        let dimension1 = entity.dimension[1];

        tokens.extend(quote! {
            EntityType {
                id: #id,
                max_health: #max_health,
                attackable: #attackable,
                mob: #mob,
                saveable: #saveable,
                limit_per_chunk: #limit_per_chunk,
                summonable: #summonable,
                fire_immune: #fire_immune,
                category: &#spawn_category,
                can_spawn_far_from_player: #can_spawn_far_from_player,
                dimension: [#dimension0, #dimension1],
                eye_height: #eye_height,
                spawn_restriction: #spawn_restriction,
                resource_name: #name,
            }
        });
    }
}

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=../../../assets/extracted/entities.json");

    let json: BTreeMap<String, EntityType> = serde_json::from_str(
        &fs::read_to_string("../../../assets/extracted/entities.json").unwrap(),
    )
    .expect("Failed to parse entities.json");

    let mut consts = TokenStream::new();
    let mut type_from_id_arms = TokenStream::new();
    let mut type_from_name = TokenStream::new();

    for (name, entity) in json.iter() {
        let upper_name = format_ident!("{}", name.to_shouty_snake_case());
        let entity_tokens = NamedEntityType(name, entity).to_token_stream();

        consts.extend(quote! {
            pub const #upper_name: EntityType = #entity_tokens;
        });

        let id_lit = LitInt::new(&entity.id.to_string(), Span::call_site());
        type_from_id_arms.extend(quote! {
            #id_lit => Some(&Self::#upper_name),
        });

        type_from_name.extend(quote! {
            #name => Some(&Self::#upper_name),
        });
    }

    quote! {
        use std::hash::Hash;

        #[derive(Debug)]
        pub struct EntityType {
            pub id: u16,
            pub max_health: Option<f32>,
            pub attackable: Option<bool>,
            pub mob: bool,
            pub saveable: bool,
            pub limit_per_chunk: i32,
            pub summonable: bool,
            pub fire_immune: bool,
            pub category: &'static MobCategory,
            pub can_spawn_far_from_player: bool,
            pub dimension: [f32; 2],
            pub eye_height: f32,
            pub spawn_restriction: SpawnRestriction,
            pub resource_name: &'static str,
        }

        impl Hash for EntityType {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state);
            }
        }

        impl PartialEq for EntityType {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }

        impl Eq for EntityType {}

        #[derive(Debug)]
        pub struct SpawnRestriction {
            pub location: SpawnLocation,
            pub heightmap: HeightMap,
        }

        #[derive(Debug)]
        pub enum SpawnLocation {
            InLava,
            InWater,
            OnGround,
            Unrestricted,
        }

        #[derive(Debug)]
        pub enum HeightMap {
            WorldSurfaceWg,
            WorldSurface,
            OceanFloorWg,
            OceanFloor,
            MotionBlocking,
            MotionBlockingNoLeaves,
        }

        #[derive(Debug)]
        #[allow(non_camel_case_types)]
        pub enum MobCategory {
            MONSTER,
            CREATURE,
            AMBIENT,
            AXOLOTLS,
            UNDERGROUND_WATER_CREATURE,
            WATER_CREATURE,
            WATER_AMBIENT,
            MISC,
        }

        impl MobCategory {
            pub const NO_DESPAWN_DISTANCE: i32 = 32;

            pub const fn max_per_chunk(&self) -> i32 {
                match self {
                    Self::MONSTER => 70,
                    Self::CREATURE => 10,
                    Self::AMBIENT => 15,
                    Self::AXOLOTLS => 5,
                    Self::UNDERGROUND_WATER_CREATURE => 5,
                    Self::WATER_CREATURE => 5,
                    Self::WATER_AMBIENT => 20,
                    Self::MISC => -1,
                }
            }

            pub const fn is_friendly(&self) -> bool {
                !matches!(self, Self::MONSTER)
            }

            pub const fn is_persistent(&self) -> bool {
                matches!(self, Self::CREATURE | Self::WATER_CREATURE | Self::MISC)
            }

            pub const fn despawn_distance(&self) -> i32 {
                match self {
                    Self::WATER_AMBIENT => 64,
                    _ => 128,
                }
            }
        }

        impl EntityType {
            #consts

            #[doc = r" Try to get an `EntityType` from its ID."]
            pub const fn try_from_id(id: u16) -> Option<&'static Self> {
                match id {
                    #type_from_id_arms
                    _ => None
                }
            }

            #[doc = r" Try to parse an `EntityType` from a resource location string."]
            pub const fn try_from_name(name: &str) -> Option<&'static Self> {
                let name = crate::helpers::strip_prefix_or_self(name, "minecraft:");
                match name {
                    #type_from_name
                    _ => None
                }
            }

            pub const fn is_mob(&self) -> bool {
                self.mob
            }

            pub const fn is_attackable(&self) -> bool {
                self.attackable.unwrap_or(false)
            }

            pub const fn can_summon(&self) -> bool {
                self.summonable
            }

            pub const fn is_fire_immune(&self) -> bool {
                self.fire_immune
            }
        }
    }
}

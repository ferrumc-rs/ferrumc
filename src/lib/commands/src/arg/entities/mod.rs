mod any_entity;
mod any_player;
mod entity_uuid;
mod player;
mod random_player;

use crate::arg::primitive::PrimitiveArgument;
use crate::arg::{CommandArgument, ParserResult};
use crate::{CommandContext, Suggestion};
use ::uuid::Uuid;
use bevy_ecs::prelude::Entity;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::identity::player_identity::PlayerIdentity;

/// Represents an entity argument in a command.
/// It can be a player name, UUID, or special selectors like @e, @p, @r, @a.
/// This won't get you an entity directly, use `resolve()` to get the entities.
#[derive(Clone, Debug, PartialEq)]
pub enum EntityArgument {
    PlayerName(String),
    Uuid(Uuid),
    AnyEntity,
    AnyPlayer,
    NearestPlayer,
    RandomPlayer,
}

impl CommandArgument for EntityArgument {
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self> {
        const PREFIXES: &[(&str, EntityArgument)] = &[
            ("@e", EntityArgument::AnyEntity),
            ("@p", EntityArgument::NearestPlayer),
            ("@r", EntityArgument::RandomPlayer),
            ("@a", EntityArgument::AnyPlayer),
        ];
        let input = ctx.input.read_string();
        for (prefix, entity_type) in PREFIXES {
            if input == *prefix {
                return Ok(entity_type.clone());
            }
        }
        if input.len() == 36 && input.chars().all(|c| c.is_ascii_hexdigit() || c == '-') {
            let uuid = Uuid::parse_str(&input)
                .map_err(|_| crate::arg::utils::parser_error("invalid UUID format"))?;
            Ok(EntityArgument::Uuid(uuid))
        } else {
            Ok(EntityArgument::PlayerName(input))
        }
    }

    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument::word()
    }

    fn suggest(_ctx: &mut CommandContext) -> Vec<Suggestion> {
        let mut suggestions = vec![
            Suggestion {
                content: "@e".to_string(),
                tooltip: Some(ferrumc_nbt::NBT::new("Any Entity".into())),
            },
            Suggestion {
                content: "@p".to_string(),
                tooltip: Some(ferrumc_nbt::NBT::new("Nearest Player".into())),
            },
            Suggestion {
                content: "@r".to_string(),
                tooltip: Some(ferrumc_nbt::NBT::new("Random Player".into())),
            },
            Suggestion {
                content: "@a".to_string(),
                tooltip: Some(ferrumc_nbt::NBT::new("All Players".into())),
            },
        ];
        let state_opt = ferrumc_state::MORE_GLOBAL_STATE.get();
        if let Some(state) = state_opt {
            for kv in &state.clone().players.player_list {
                let (_, (uuid, name)) = kv.pair();
                suggestions.push(Suggestion {
                    content: name.clone(),
                    tooltip: Some(ferrumc_nbt::NBT::new(
                        uuid::Uuid::from_u128(*uuid)
                            .as_hyphenated()
                            .to_string()
                            .to_uppercase()
                            .into(),
                    )),
                });
            }
        }
        suggestions
    }
}

impl EntityArgument {
    pub fn resolve(
        &self,
        iter: impl Iterator<Item = (Entity, Option<&EntityIdentity>, Option<&PlayerIdentity>)>,
    ) -> Vec<Entity> {
        match self {
            EntityArgument::PlayerName(name) => player::resolve_player_name(name.clone(), iter)
                .map(|e| vec![e])
                .unwrap_or_default(),
            EntityArgument::Uuid(uuid) => entity_uuid::resolve_uuid(*uuid, iter)
                .map(|e| vec![e])
                .unwrap_or_default(),
            EntityArgument::AnyEntity => any_entity::resolve_any_entity(iter),
            EntityArgument::AnyPlayer => any_player::resolve_any_player(iter),
            EntityArgument::NearestPlayer => {
                unimplemented!()
            }
            EntityArgument::RandomPlayer => random_player::resolve_random_player(iter)
                .map(|e| vec![e])
                .unwrap_or_default(),
        }
    }
}

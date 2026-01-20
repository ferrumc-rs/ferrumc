use crate::arg::primitive::PrimitiveArgument;
use crate::arg::{CommandArgument, ParserResult};
use crate::{CommandContext, Suggestion};

#[derive(Clone, Debug, PartialEq)]
pub enum CommandArgEntityType {
    PlayerName(String),
    PlayerUUID(String),
    AnyEntity,
    AnyPlayer,
    NearestPlayer,
    RandomPlayer,
}

impl CommandArgument for CommandArgEntityType {
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self> {
        const PREFIXES: &[(&str, CommandArgEntityType)] = &[
            ("@e", CommandArgEntityType::AnyEntity),
            ("@p", CommandArgEntityType::NearestPlayer),
            ("@r", CommandArgEntityType::RandomPlayer),
            ("@a", CommandArgEntityType::AnyPlayer),
        ];
        let input = ctx.input.read_string();
        for (prefix, entity_type) in PREFIXES {
            if input == *prefix {
                return Ok(entity_type.clone());
            }
        }
        if input.len() == 36 && input.chars().all(|c| c.is_ascii_hexdigit() || c == '-') {
            Ok(CommandArgEntityType::PlayerUUID(input))
        } else {
            Ok(CommandArgEntityType::PlayerName(input))
        }
    }

    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument::word()
    }

    fn suggest(ctx: &mut CommandContext) -> Vec<Suggestion> {
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

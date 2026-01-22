use crate::arg::primitive::PrimitiveArgument;
use crate::arg::utils::parser_error;
use crate::arg::{CommandArgument, ParserResult};
use crate::{CommandContext, Suggestion};
use ferrumc_core::transform::position::Position;

pub struct CommandPosition {
    x: PositionType,
    y: PositionType,
    z: PositionType,
}

enum PositionType {
    Absolute(f64),
    Relative(f64),
}

impl CommandArgument for CommandPosition {
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self> {
        let mut string_input = String::new();
        while ctx.input.has_remaining_input() {
            if !string_input.is_empty() {
                string_input.push(' ');
            }
            string_input.push_str(&ctx.input.read_string());
        }
        let mut parts = string_input.split_whitespace();
        let x_str = parts
            .next()
            .ok_or_else(|| parser_error("missing x coordinate"))?;
        let y_str = parts
            .next()
            .ok_or_else(|| parser_error("missing y coordinate"))?;
        let z_str = parts
            .next()
            .ok_or_else(|| parser_error("missing z coordinate"))?;
        let x = if x_str.starts_with('~') {
            let offset = if x_str.len() > 1 {
                x_str[1..]
                    .parse::<f64>()
                    .map_err(|_| parser_error("invalid x coordinate"))?
            } else {
                0.0
            };
            PositionType::Relative(offset)
        } else {
            let value = x_str
                .parse::<f64>()
                .map_err(|_| parser_error("invalid x coordinate"))?;
            PositionType::Absolute(value)
        };
        let y = if y_str.starts_with('~') {
            let offset = if y_str.len() > 1 {
                y_str[1..]
                    .parse::<f64>()
                    .map_err(|_| parser_error("invalid y coordinate"))?
            } else {
                0.0
            };
            PositionType::Relative(offset)
        } else {
            let value = y_str
                .parse::<f64>()
                .map_err(|_| parser_error("invalid y coordinate"))?;
            PositionType::Absolute(value)
        };
        let z = if z_str.starts_with('~') {
            let offset = if z_str.len() > 1 {
                z_str[1..]
                    .parse::<f64>()
                    .map_err(|_| parser_error("invalid z coordinate"))?
            } else {
                0.0
            };
            PositionType::Relative(offset)
        } else {
            let value = z_str
                .parse::<f64>()
                .map_err(|_| parser_error("invalid z coordinate"))?;
            PositionType::Absolute(value)
        };
        Ok(CommandPosition { x, y, z })
    }

    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument::greedy()
    }

    fn suggest(ctx: &mut CommandContext) -> Vec<Suggestion> {
        vec![Suggestion::of("~ ~ ~")]
    }
}

impl CommandPosition {
    pub fn resolve(&self, position: &Position) -> Position {
        let x = match self.x {
            PositionType::Absolute(val) => val,
            PositionType::Relative(offset) => position.x + offset,
        };
        let y = match self.y {
            PositionType::Absolute(val) => val,
            PositionType::Relative(offset) => position.y + offset,
        };
        let z = match self.z {
            PositionType::Absolute(val) => val,
            PositionType::Relative(offset) => position.z + offset,
        };
        Position::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Command, CommandInput, Sender};
    use std::sync::Arc;

    #[test]
    fn test_parse() {
        let mut ctx = CommandContext {
            input: CommandInput {
                input: "~10 5 ~-10".to_string(),
                cursor: 0,
            },
            command: Arc::new(Command {
                name: "",
                args: vec![],
            }),
            sender: Sender::Server,
        };
        let cmd_pos = CommandPosition::parse(&mut ctx).unwrap();
        match cmd_pos.x {
            PositionType::Relative(offset) => assert_eq!(offset, 10.0),
            _ => panic!("Expected relative x"),
        }
        match cmd_pos.y {
            PositionType::Absolute(value) => assert_eq!(value, 5.0),
            _ => panic!("Expected absolute y"),
        }
        match cmd_pos.z {
            PositionType::Relative(offset) => assert_eq!(offset, -10.0),
            _ => panic!("Expected relative z"),
        }
    }

    #[test]
    fn test_resolve() {
        let cmd_pos = CommandPosition {
            x: PositionType::Relative(10.0),
            y: PositionType::Absolute(5.0),
            z: PositionType::Relative(-10.0),
        };
        let base_position = Position::new(100.0, 64.0, 100.0);
        let resolved = cmd_pos.resolve(&base_position);
        assert_eq!(resolved.x, 110.0);
        assert_eq!(resolved.y, 5.0);
        assert_eq!(resolved.z, 90.0);
    }
}

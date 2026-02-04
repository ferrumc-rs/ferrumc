use crate::arg::primitive::PrimitiveArgument;
use crate::arg::utils::parser_error;
use crate::arg::{CommandArgument, ParserResult};
use crate::{CommandContext, Suggestion};
use ferrumc_core::transform::position::Position;

/// Represents a position argument in a command, which can be absolute or relative.
/// For example: "100 64 -200" (absolute) or "~10 ~ ~-5" (relative).
///
/// The coordinates are initially opaque, in order to get an actual world position you must pass in
/// a base position to resolve against, usually the player calling the command. You can pass in a
/// 0,0,0 position if you want absolute coordinates only.
///
/// # Example
/// ```ignore
/// use ferrumc_commands::arg::position::CommandPosition;
/// use ferrumc_core::transform::position::Position;
///
/// let cmd_pos = CommandPosition::parse("~10 64 ~-5").unwrap;
/// let base_position = Position::new(100.0, 64.0, 100.0);
/// let resolved_position = cmd_pos.resolve(&base_position);
/// assert_eq!(resolved_position, Position::new(110.0, 64.0, 95.0));
/// ```
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
        let x = if let Some(x_str) = x_str.strip_prefix('~') {
            if x_str.is_empty() {
                PositionType::Relative(0.0)
            } else {
                let offset = x_str
                    .parse::<f64>()
                    .map_err(|_| parser_error("invalid x coordinate"))?;
                PositionType::Relative(offset)
            }
        } else {
            let value = x_str
                .parse::<f64>()
                .map_err(|_| parser_error("invalid x coordinate"))?;
            PositionType::Absolute(value)
        };
        let y = if let Some(y_str) = y_str.strip_prefix('~') {
            if y_str.is_empty() {
                PositionType::Relative(0.0)
            } else {
                let offset = y_str
                    .parse::<f64>()
                    .map_err(|_| parser_error("invalid y coordinate"))?;
                PositionType::Relative(offset)
            }
        } else {
            let value = y_str
                .parse::<f64>()
                .map_err(|_| parser_error("invalid y coordinate"))?;
            PositionType::Absolute(value)
        };
        let z = if let Some(z_str) = z_str.strip_prefix('~') {
            if z_str.is_empty() {
                PositionType::Relative(0.0)
            } else {
                let offset = z_str
                    .parse::<f64>()
                    .map_err(|_| parser_error("invalid z coordinate"))?;
                PositionType::Relative(offset)
            }
        } else {
            let value = z_str
                .parse::<f64>()
                .map_err(|_| parser_error("invalid z coordinate"))?;
            PositionType::Absolute(value)
        };
        if parts.next().is_some() {
            return Err(parser_error("too many coordinates provided"));
        }
        Ok(CommandPosition { x, y, z })
    }

    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument::greedy()
    }

    fn suggest(_ctx: &mut CommandContext) -> Vec<Suggestion> {
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
    use ferrumc_state::create_test_state;
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
            state: create_test_state().0.0,
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

    #[test]
    fn parse_valid_inputs() {
        let cases = vec![
            (
                "100 64 -200",
                (
                    PositionType::Absolute(100.0),
                    PositionType::Absolute(64.0),
                    PositionType::Absolute(-200.0),
                ),
            ),
            (
                "~10 ~ ~-5",
                (
                    PositionType::Relative(10.0),
                    PositionType::Relative(0.0),
                    PositionType::Relative(-5.0),
                ),
            ),
            (
                "50 ~20 30",
                (
                    PositionType::Absolute(50.0),
                    PositionType::Relative(20.0),
                    PositionType::Absolute(30.0),
                ),
            ),
            (
                "~ ~ ~",
                (
                    PositionType::Relative(0.0),
                    PositionType::Relative(0.0),
                    PositionType::Relative(0.0),
                ),
            ),
            (
                "1 2 ~",
                (
                    PositionType::Absolute(1.0),
                    PositionType::Absolute(2.0),
                    PositionType::Relative(0.0),
                ),
            ),
            (
                "~-0 ~0 ~+0",
                (
                    PositionType::Relative(-0.0),
                    PositionType::Relative(0.0),
                    PositionType::Relative(0.0),
                ),
            ),
        ];
        for (input, expected) in cases {
            let mut ctx = CommandContext {
                input: CommandInput {
                    input: input.to_string(),
                    cursor: 0,
                },
                command: Arc::new(Command {
                    name: "",
                    args: vec![],
                }),
                sender: Sender::Server,
                state: create_test_state().0.0,
            };
            let cmd_pos = CommandPosition::parse(&mut ctx)
                .unwrap_or_else(|_| panic!("input `{}` should be valid", input));
            match cmd_pos.x {
                PositionType::Absolute(val) => match expected.0 {
                    PositionType::Absolute(exp_val) => assert_eq!(val, exp_val),
                    _ => panic!("Expected relative x for input `{}`", input),
                },
                PositionType::Relative(offset) => match expected.0 {
                    PositionType::Relative(exp_offset) => assert_eq!(offset, exp_offset),
                    _ => panic!("Expected absolute x for input `{}`", input),
                },
            }
            match cmd_pos.y {
                PositionType::Absolute(val) => match expected.1 {
                    PositionType::Absolute(exp_val) => assert_eq!(val, exp_val),
                    _ => panic!("Expected relative y for input `{}`", input),
                },
                PositionType::Relative(offset) => match expected.1 {
                    PositionType::Relative(exp_offset) => assert_eq!(offset, exp_offset),
                    _ => panic!("Expected absolute y for input `{}`", input),
                },
            }
            match cmd_pos.z {
                PositionType::Absolute(val) => match expected.2 {
                    PositionType::Absolute(exp_val) => assert_eq!(val, exp_val),
                    _ => panic!("Expected relative z for input `{}`", input),
                },
                PositionType::Relative(offset) => match expected.2 {
                    PositionType::Relative(exp_offset) => assert_eq!(offset, exp_offset),
                    _ => panic!("Expected absolute z for input `{}`", input),
                },
            }
        }
    }

    #[test]
    fn test_parse_invalid_inputs() {
        let cases = vec!["", "1 2", "1 two 3", "not_a_number 5 6", "~ ~", "1 2 3 4"];
        for input in cases {
            let mut ctx = CommandContext {
                input: CommandInput {
                    input: input.to_string(),
                    cursor: 0,
                },
                command: Arc::new(Command {
                    name: "",
                    args: vec![],
                }),
                sender: Sender::Server,
                state: create_test_state().0.0,
            };
            assert!(
                CommandPosition::parse(&mut ctx).is_err(),
                "input `{}` should be invalid",
                input
            );
        }
    }
}

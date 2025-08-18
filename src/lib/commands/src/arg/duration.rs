use std::{sync::LazyLock, time::Duration};

use regex::Regex;

use crate::{CommandContext, Suggestion};

use super::{primitive::PrimitiveArgument, utils::parser_error, CommandArgument, ParserResult};

static PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("(([1-9][0-9]+|[1-9])[dhms])").unwrap());

impl CommandArgument for Duration {
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self> {
        let mut duration = Duration::ZERO;

        for (_, [line, value]) in PATTERN
            .captures_iter(&ctx.input.read_string())
            .map(|c| c.extract())
        {
            let Some(unit) = line.chars().nth(line.len() - 1) else {
                return Err(parser_error("missing time unit"));
            };
            let Ok(value) = value.parse::<u64>() else {
                return Err(parser_error("invalid number"));
            };

            match unit {
                'd' => duration += Duration::from_days(value),
                'h' => duration += Duration::from_hours(value),
                'm' => duration += Duration::from_mins(value),
                's' => duration += Duration::from_secs(value),
                _ => return Err(parser_error("invalid unit: expected d/h/m/s")),
            }
        }

        if duration.is_zero() {
            return Err(parser_error("invalid input"));
        }

        Ok(duration)
    }

    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument::word()
    }

    fn suggest(ctx: &mut CommandContext) -> Vec<Suggestion> {
        ctx.input.skip_whitespace(u32::MAX, false);
        if !ctx.input.has_remaining_input() {
            return (0..9)
                .map(|i| Suggestion::of(i.to_string()))
                .collect();
        };

        let input = ctx.input.read_string();

        if input.chars().last().unwrap().is_ascii_alphabetic() {
            return vec![];
        }

        ['d', 'h', 'm', 's']
            .into_iter()
            .filter(|unit| !input.contains(*unit))
            .map(|unit| Suggestion::of(input.clone() + &unit.to_string()))
            .collect()
    }
}

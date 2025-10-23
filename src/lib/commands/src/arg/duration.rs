use regex::Regex;
use std::{sync::LazyLock, time::Duration};

use super::{primitive::PrimitiveArgument, utils::parser_error, CommandArgument, ParserResult};
use crate::{CommandContext, Suggestion};

/// Regex pattern for parsing duration strings (e.g., `1d`, `12h`, `30m`, `45s`).
static PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("(([1-9][0-9]+|[1-9])[dhms])").unwrap());

/// Implements `CommandArgument` for `Duration`.
///
/// Accepts strings representing time durations, such as:
/// - `1d` → 1 day
/// - `2h` → 2 hours
/// - `30m` → 30 minutes
/// - `45s` → 45 seconds
///
/// Multiple units can be combined, e.g., `1d2h30m`.
impl CommandArgument for Duration {
    /// Parses a duration string from the command input.
    ///
    /// # Errors
    /// Returns a parser error if:
    /// - The string has an invalid number
    /// - The string has a missing or invalid unit
    /// - No valid duration is found
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

    /// Returns the primitive argument type for this parser.
    ///
    /// A duration is treated as a simple word string in commands.
    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument::word()
    }

    /// Provides suggestions for partial duration input.
    ///
    /// Examples:
    /// - Input `1` → suggestions: `1d`, `1h`, `1m`, `1s`
    /// - Input `1h` → suggestions: `1hd`, `1hm`, `1hs` (filters out used units)
    fn suggest(ctx: &mut CommandContext) -> Vec<Suggestion> {
        ctx.input.skip_whitespace(u32::MAX, false);
        if !ctx.input.has_remaining_input() {
            return (0..9).map(|i| Suggestion::of(i.to_string())).collect();
        }

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

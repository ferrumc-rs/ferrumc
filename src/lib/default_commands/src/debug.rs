//! Debug command for toggling debug information display.
//!
//! This command allows players to toggle various debug overlays
//! that display information in the action bar.

use bevy_ecs::prelude::*;
use ferrumc_commands::{
    arg::{primitive::PrimitiveArgument, utils::parser_error, CommandArgument, ParserResult},
    CommandContext, Sender, Suggestion,
};
use ferrumc_components::player::debug_settings::{
    available_debug_flags, parse_debug_flag, DebugSettings,
};
use ferrumc_macros::command;
use ferrumc_text::{NamedColor, TextComponentBuilder};
use tracing::info;

// ============================================================================
// Debug Flag Argument Type
// ============================================================================

/// Wrapper type for debug flag arguments that implements CommandArgument.
#[derive(Debug, Clone)]
struct DebugFlagArg(String);

impl CommandArgument for DebugFlagArg {
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self> {
        let str = ctx.input.read_string();

        // Validate the flag name
        if str.is_empty() {
            return Err(parser_error("Debug flag name cannot be empty"));
        }

        // Accept "off", "disable", "none" for disabling
        if matches!(str.to_lowercase().as_str(), "off" | "disable" | "none") {
            return Ok(DebugFlagArg(str.to_string()));
        }

        // Validate against known flags
        if parse_debug_flag(&str).is_none() {
            return Err(parser_error(&format!(
                "Unknown debug flag: '{}'. Available: {}",
                str,
                available_debug_flags().join(", ")
            )));
        }

        Ok(DebugFlagArg(str.to_string()))
    }

    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument::word()
    }

    fn suggest(_ctx: &mut CommandContext) -> Vec<Suggestion> {
        let mut suggestions: Vec<Suggestion> = available_debug_flags()
            .iter()
            .map(|&s| Suggestion::of(s))
            .collect();
        suggestions.push(Suggestion::of("off"));
        suggestions
    }
}

// ============================================================================
// Debug Command
// ============================================================================

/// Toggles debug information display in the action bar.
///
/// # Usage
/// - `/debug <flag>` - Toggles the specified debug flag
///
/// # Available Flags
/// - `chunk` - Shows current chunk coordinates
/// - `position` - Shows exact position (X, Y, Z)
/// - `rotation` - Shows rotation (yaw, pitch)
/// - `all` - Toggles all debug information
/// - `off` - Disables all debug displays
#[command("debug")]
fn debug_command(
    #[sender] sender: Sender,
    #[arg] flag: DebugFlagArg,
    mut debug_query: Query<&mut DebugSettings>,
) {
    // Ensure the sender is a player
    let player_entity = match sender {
        Sender::Server => {
            sender.send_message(
                TextComponentBuilder::new("Error: Debug command is player-only.")
                    .color(NamedColor::Red)
                    .build(),
                false,
            );
            return;
        }
        Sender::Player(entity) => entity,
    };

    // Get the player's debug settings
    let mut debug_settings = match debug_query.get_mut(player_entity) {
        Ok(settings) => settings,
        Err(_) => {
            sender.send_message(
                TextComponentBuilder::new("Error: Could not find your debug settings.")
                    .color(NamedColor::Red)
                    .build(),
                false,
            );
            return;
        }
    };

    let flag_name = flag.0.to_lowercase();

    // Handle disable commands
    if matches!(flag_name.as_str(), "off" | "disable" | "none") {
        debug_settings.disable_all();
        sender.send_message(
            TextComponentBuilder::new("Debug display disabled.")
                .color(NamedColor::Yellow)
                .build(),
            false,
        );
        info!("Player disabled debug display");
        return;
    }

    // Toggle the specified flag
    if let Some(flag) = parse_debug_flag(&flag_name) {
        let enabled = debug_settings.toggle(flag);
        let status = if enabled { "enabled" } else { "disabled" };
        let color = if enabled {
            NamedColor::Green
        } else {
            NamedColor::Yellow
        };

        sender.send_message(
            TextComponentBuilder::new(format!("Debug '{}' {}.", flag_name, status))
                .color(color)
                .build(),
            false,
        );
        info!("Player {} debug flag '{}'", status, flag_name);
    }
}

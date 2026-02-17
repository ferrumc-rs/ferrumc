//! Direct world block interactions.
//!
//! This module handles interactions with blocks directly in the world (chunks),
//! without creating ECS entities. This is more performant for simple toggleable
//! blocks like doors, levers, trapdoors, and buttons.
//!
//! ## How it works
//!
//! 1. Player right-clicks on a block
//! 2. System checks if the block is "interactive" based on its name
//! 3. If yes, toggle the relevant property (e.g., "open" for doors)
//! 4. Update the block in the chunk
//! 5. Broadcast BlockUpdate to nearby players
//!
//! ## Supported blocks
//!
//! - Doors (oak_door, iron_door, etc.) - toggles "open" property
//! - Trapdoors - toggles "open" property
//! - Fence gates - toggles "open" property
//! - Levers - toggles "powered" property
//! - Buttons - activates temporarily (TODO: timer system)

use ferrumc_messages::BlockBrokenEvent;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::pos::BlockPos;
use ferrumc_world::vanilla_chunk_format::BlockData;
use std::collections::BTreeMap;
use tracing::{debug, warn};

/// Result of attempting to interact with a block.
#[derive(Debug, Clone)]
pub enum InteractionResult {
    /// Block was toggled, returns the new BlockStateId
    Toggled(BlockStateId),
    /// Block is not interactive
    NotInteractive,
    /// Block state not found (shouldn't happen)
    InvalidBlock,
}

/// Checks if a block is interactive and returns its interaction type.
pub fn get_interaction_type(block_data: &BlockData) -> Option<InteractionType> {
    let name = &block_data.name;

    // Doors
    if name.ends_with("_door") {
        return Some(InteractionType::Toggleable("open"));
    }

    // Trapdoors
    if name.ends_with("_trapdoor") {
        return Some(InteractionType::Toggleable("open"));
    }

    // Fence gates
    if name.ends_with("_fence_gate") {
        return Some(InteractionType::Toggleable("open"));
    }

    // Levers
    if name == "minecraft:lever" {
        return Some(InteractionType::Toggleable("powered"));
    }

    // Buttons
    if name.ends_with("_button") {
        return Some(InteractionType::Momentary("powered"));
    }

    // Redstone components
    if name == "minecraft:repeater" || name == "minecraft:comparator" {
        // These have different interaction behavior (cycle modes)
        return None; // TODO: implement mode cycling
    }

    None
}

/// Type of interaction for a block.
#[derive(Debug, Clone, Copy)]
pub enum InteractionType {
    /// Block toggles between two states (doors, levers)
    /// The string is the property name to toggle
    Toggleable(&'static str),
    /// Block activates momentarily then deactivates (buttons)
    /// The string is the property name
    Momentary(&'static str),
}

/// Attempts to interact with a block and returns the new state if successful.
///
/// This function:
/// 1. Gets the BlockData from the BlockStateId
/// 2. Checks if it's an interactive block
/// 3. Toggles the appropriate property
/// 4. Returns the new BlockStateId
pub fn try_interact(block_state_id: BlockStateId) -> InteractionResult {
    debug!(
        "try_interact called with block_state_id: {:?} (raw: {})",
        block_state_id,
        block_state_id.raw()
    );

    // Get the block data
    let Some(mut block_data) = block_state_id.to_block_data() else {
        warn!(
            "try_interact: InvalidBlock - could not convert {:?} to BlockData",
            block_state_id
        );
        return InteractionResult::InvalidBlock;
    };

    debug!(
        "try_interact: block_data name='{}', properties={:?}",
        block_data.name, block_data.properties
    );

    // Check if it's interactive
    let Some(interaction_type) = get_interaction_type(&block_data) else {
        debug!(
            "try_interact: block '{}' is not interactive",
            block_data.name
        );
        return InteractionResult::NotInteractive;
    };

    debug!("try_interact: interaction_type={:?}", interaction_type);

    // Get or create properties map
    let properties = block_data.properties.get_or_insert_with(BTreeMap::new);

    match interaction_type {
        InteractionType::Toggleable(prop_name) => {
            // Toggle the property
            let current_value = properties
                .get(prop_name)
                .map(|s| s.as_str())
                .unwrap_or("false");
            let new_value = if current_value == "true" {
                "false"
            } else {
                "true"
            };
            debug!(
                "try_interact: toggling '{}' from '{}' to '{}'",
                prop_name, current_value, new_value
            );
            properties.insert(prop_name.to_string(), new_value.to_string());
        }
        InteractionType::Momentary(prop_name) => {
            // For momentary switches, just activate (deactivation handled by timer)
            debug!("try_interact: activating momentary '{}'", prop_name);
            properties.insert(prop_name.to_string(), "true".to_string());
            // TODO: Schedule deactivation after delay
        }
    }

    debug!(
        "try_interact: modified block_data properties={:?}",
        block_data.properties
    );

    // Convert back to BlockStateId
    let new_state_id = BlockStateId::from_block_data(&block_data);
    debug!(
        "try_interact: new_state_id={:?} (raw: {})",
        new_state_id,
        new_state_id.raw()
    );

    if new_state_id.raw() == 0 {
        warn!("try_interact: WARNING - new_state_id is 0 (air)! BlockData lookup failed for: name='{}', props={:?}",
              block_data.name, block_data.properties);
    }

    InteractionResult::Toggled(new_state_id)
}

/// Given a block state, if it's a door, returns the Y offset to the other half.
/// Lower half -> +1, upper half -> -1, not a door -> None.
pub fn door_other_half_y_offset(block_state_id: BlockStateId) -> Option<i32> {
    let data = block_state_id.to_block_data()?;
    if !data.name.ends_with("_door") {
        return None;
    }
    let props = data.properties.as_ref()?;
    let half = props.get("half")?;
    match half.as_str() {
        "lower" => Some(1),
        "upper" => Some(-1),
        _ => None,
    }
}

/// Checks if a block is interactive without modifying it.
pub fn is_interactive(block_state_id: BlockStateId) -> bool {
    block_state_id
        .to_block_data()
        .as_ref()
        .and_then(get_interaction_type)
        .is_some()
}

/// Gets the "open" state of a door/trapdoor/fence gate.
#[allow(dead_code)]
pub fn is_open(block_state_id: BlockStateId) -> Option<bool> {
    let block_data = block_state_id.to_block_data()?;
    let properties = block_data.properties.as_ref()?;
    let open_value = properties.get("open")?;
    Some(open_value == "true")
}

/// Gets the "powered" state of a lever/button.
#[allow(dead_code)]
pub fn is_powered(block_state_id: BlockStateId) -> Option<bool> {
    let block_data = block_state_id.to_block_data()?;
    let properties = block_data.properties.as_ref()?;
    let powered_value = properties.get("powered")?;
    Some(powered_value == "true")
}

/// Breaks a block and its door-pair (if applicable).
/// Sets both positions to air and emits `BlockBrokenEvent` for each.
/// Returns the list of all positions that were broken (always includes `pos`,
/// and may include the other door half).
pub fn break_block_with_door_half(
    chunk: &mut ferrumc_world::MutChunk,
    pos: BlockPos,
    block_break_writer: &mut bevy_ecs::prelude::MessageWriter<BlockBrokenEvent>,
) -> Vec<BlockPos> {
    let current_state = chunk.get_block(pos.chunk_block_pos());
    let other_half = door_other_half_y_offset(current_state).map(|y_off| pos + (0, y_off, 0));

    chunk.set_block(pos.chunk_block_pos(), BlockStateId::default());
    block_break_writer.write(BlockBrokenEvent { position: pos });

    let mut broken = vec![pos];

    if let Some(other_pos) = other_half {
        chunk.set_block(other_pos.chunk_block_pos(), BlockStateId::default());
        block_break_writer.write(BlockBrokenEvent {
            position: other_pos,
        });
        debug!(
            "Also broke other door half at ({}, {}, {})",
            other_pos.pos.x, other_pos.pos.y, other_pos.pos.z
        );
        broken.push(other_pos);
    }

    broken
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_door_detection() {
        let door_data = BlockData {
            name: "minecraft:oak_door".to_string(),
            properties: Some(BTreeMap::from([
                ("facing".to_string(), "north".to_string()),
                ("open".to_string(), "false".to_string()),
                ("half".to_string(), "lower".to_string()),
                ("hinge".to_string(), "left".to_string()),
            ])),
        };

        assert!(matches!(
            get_interaction_type(&door_data),
            Some(InteractionType::Toggleable("open"))
        ));
    }

    #[test]
    fn test_lever_detection() {
        let lever_data = BlockData {
            name: "minecraft:lever".to_string(),
            properties: Some(BTreeMap::from([
                ("face".to_string(), "wall".to_string()),
                ("facing".to_string(), "north".to_string()),
                ("powered".to_string(), "false".to_string()),
            ])),
        };

        assert!(matches!(
            get_interaction_type(&lever_data),
            Some(InteractionType::Toggleable("powered"))
        ));
    }
}

use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_net::connection::StreamWriter;
use tracing::{debug, error, warn};

use ferrumc_net::PlayerAbilitiesReceiver;

use ferrumc_net::packets::outgoing::player_abilities::PlayerAbilities as OutgoingPlayerAbilities;

use ferrumc_components::player::abilities::PlayerAbilities;

/// Handles incoming PlayerAbilities packets (client telling us its flying status)
pub fn handle(
    // 1. Get the queue of incoming packets
    events: Res<PlayerAbilitiesReceiver>,

    // 2. Get mutable access to all PlayerAbilities components
    mut abilities_query: Query<&mut PlayerAbilities>,

    // 3. Get all player connections (to send corrections)
    writer_query: Query<(Entity, &StreamWriter)>,
) {
    // Loop through each packet received this tick
    for (event, trigger_eid) in events.0.try_iter() {
        // `event` is the incoming `PlayerAbilities` { flags: u8 }
        // `trigger_eid` is the Bevy Entity of the player who sent it

        // Get the PlayerAbilities component only for the player who sent this
        let mut abilities = match abilities_query.get_mut(trigger_eid) {
            Ok(abilities) => abilities,
            Err(_) => {
                warn!(
                    "Received PlayerAbilities from entity {} without component.",
                    trigger_eid.index()
                );
                continue;
            }
        };

        // --- Packet Logic ---
        let client_is_flying = (event.flags & 0x02) == 0x02;

        // --- Validation ---
        if abilities.may_fly {
            // Player is allowed to fly. Update the server's state.
            debug!(
                "Player {} toggled flying to: {}",
                trigger_eid.index(),
                client_is_flying
            );
            abilities.flying = client_is_flying; // Update the component
        } else {
            // Player is NOT allowed to fly. Send a correction packet.
            warn!(
                "Player {} tried to fly without permission. Sending correction.",
                trigger_eid.index()
            );

            // 1. Ensure our server-side state is correct
            abilities.flying = false;

            // 2. Find this player's connection
            if let Ok((_, writer)) = writer_query.get(trigger_eid) {
                // 3. Construct the outgoing packet from our component
                let outgoing_flags = (abilities.invulnerable as u8)
                    | (abilities.flying as u8 * 0x02)
                    | (abilities.may_fly as u8 * 0x04)
                    | (abilities.creative_mode as u8 * 0x08);

                let correction_packet = OutgoingPlayerAbilities {
                    flags: outgoing_flags,
                    flying_speed: abilities.flying_speed,
                    field_of_view_modifier: 1.0, // Default for now
                };

                // 4. Send the correction
                if let Err(e) = writer.send_packet_ref(&correction_packet) {
                    error!(
                        "Failed to send abilities correction packet to {}: {:?}",
                        trigger_eid.index(),
                        e
                    );
                }
            }
        }
    }
}

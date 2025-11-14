use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_net::connection::StreamWriter;
use ferrumc_state::GlobalStateResource;
use tracing::{debug, error, warn};

use ferrumc_net::PlayerAbilitiesReceiver;

use ferrumc_net::packets::outgoing::player_abilities::PlayerAbilities as OutgoingPlayerAbilities;

/// This system handles incoming PlayerAbilities packets from clients.
/// It's primarily used to toggle the player's flying status.
pub fn handle(
    // 1. Get the queue of incoming packets
    events: Res<PlayerAbilitiesReceiver>,
    // 2. Get the global state (to find the player)
    state: Res<GlobalStateResource>,
    // 3. Get all player connections (to send corrections)
    writer_query: Query<(Entity, &StreamWriter)>,
) {
    // Loop through each packet received this tick
    for (event, trigger_eid) in events.0.try_iter() {
        // `event` is your incoming `PlayerAbilities` struct: { flags: u8 }
        // `trigger_eid` is the Bevy Entity of the player who sent it

        let player = match state.0.players.player_list.get_mut(&trigger_eid) {
            Some(player) => player,
            None => {
                // This can happen if the player disconnected mid-tick
                warn!(
                    "Received PlayerAbilities from disconnected entity {}",
                    trigger_eid.index()
                );
                continue;
            }
        };

        let abilities = &mut player.abilities; // e.g., PlayerAbilities { flags, flying_speed, ... }

        // --- Packet Logic ---
        // The client is telling us its flying status.
        let client_is_flying = (event.flags & 0x02) == 0x02;

        // --- Validation ---
        // We must check if the server *allows* this player to fly.
        // TODO: Replace this with a real check (e.g., player gamemode, permissions)
        let server_allows_flying = (abilities.flags & 0x04) == 0x04; // Check the 'Allow Flying' bit

        if server_allows_flying {
            // The player is allowed to fly. Update the server's state.
            debug!(
                "Player {} toggled flying to: {}",
                trigger_eid.index(),
                client_is_flying
            );
            if client_is_flying {
                abilities.flags |= 0x02; // Set the server-side 'Flying' bit
            } else {
                abilities.flags &= !0x02; // Clear the server-side 'Flying' bit
            }
        } else {
            // The player is NOT allowed to fly, but they sent a packet saying they are.
            // This is a "client-server mismatch" and we must correct the client.
            warn!(
                "Player {} tried to fly without permission. Sending correction.",
                trigger_eid.index()
            );

            // 1. Ensure our server-side state is correct (not flying)
            abilities.flags &= !0x02;

            // 2. Find this player's connection
            if let Ok((_, writer)) = writer_query.get(trigger_eid) {
                // 3. Construct a new outgoing packet with the server's
                // authoritative state and send it back to the client.
                let correction_packet = OutgoingPlayerAbilities {
                    flags: abilities.flags,
                    flying_speed: abilities.flying_speed,
                    field_of_view_modifier: abilities.field_of_view_modifier,
                };

                // Send the correction
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

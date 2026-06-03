use bevy_ecs::prelude::Component;
use std::collections::{HashSet, VecDeque};
use typename::TypeName;

#[derive(TypeName, Component)]
pub struct ChunkReceiver {
    pub loading: VecDeque<(i32, i32)>,
    pub dirty: VecDeque<(i32, i32)>,
    pub loaded: HashSet<(i32, i32)>,
    pub unloading: VecDeque<(i32, i32)>,
    pub chunks_per_tick: f32,
}

impl Default for ChunkReceiver {
    fn default() -> Self {
        Self::new()
    }
}

impl ChunkReceiver {
    pub fn new() -> Self {
        Self {
            loading: VecDeque::new(),
            loaded: HashSet::new(),
            unloading: VecDeque::new(),
            dirty: VecDeque::new(),
            // 32.5 chunks per tick is enough to send 650 chunks per second (20 ticks per second)
            chunks_per_tick: 32.5,
        }
    }
}

/// Computes the effective chunk view radius (in chunks) for a player from the server's configured
/// render distance and the client's reported view distance.
///
/// A client that has not yet sent its settings packet reports a view distance of 0 (the
/// [`Default`] for `ClientInformationComponent`). Treating that literally with `min(server, 0)`
/// collapses the loaded region to a single chunk — the player sees a tiny island of world with
/// void beyond. So a client distance of 0 or 1 is treated as "unknown" and falls back to the
/// server's render distance; otherwise the smaller of the two is used (never send more than the
/// client wants, never more than the server allows).
///
/// IMPORTANT: the chunk calculator (which decides what to *queue*) and the chunk sender (which
/// decides what to actually *send*) must both use this function AND the same distance metric
/// (Chebyshev / square). If they disagree, chunks get queued but filtered out at send time,
/// never enter `loaded`, and are re-queued forever — starving genuinely new chunks and burning
/// CPU every tick.
pub fn effective_view_radius(server_render_distance: i32, client_view_distance: i32) -> i32 {
    if client_view_distance <= 1 {
        server_render_distance
    } else {
        server_render_distance.min(client_view_distance)
    }
}

#[cfg(test)]
mod tests {
    use super::effective_view_radius;

    #[test]
    fn unknown_client_distance_falls_back_to_server() {
        // A client that hasn't sent settings yet reports 0. We must NOT collapse to a single
        // chunk; fall back to the server render distance instead.
        assert_eq!(effective_view_radius(12, 0), 12);
        // A reported distance of 1 is implausibly tiny and treated the same way.
        assert_eq!(effective_view_radius(12, 1), 12);
    }

    #[test]
    fn uses_smaller_of_the_two_when_client_is_known() {
        // Client wants less than the server allows: honour the client.
        assert_eq!(effective_view_radius(12, 8), 8);
        // Client wants more than the server allows: cap at the server.
        assert_eq!(effective_view_radius(12, 32), 12);
        // Equal.
        assert_eq!(effective_view_radius(10, 10), 10);
    }
}

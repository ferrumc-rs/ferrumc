use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = 0x2B)]
pub struct LoginPlayPacket<'a> {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub dimension_length: VarInt,
    pub dimension_names: &'a [&'a str],
    pub max_players: VarInt,
    pub view_distance: VarInt,
    pub simulation_distance: VarInt,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub do_limited_crafting: bool,
    pub dimension_type: VarInt,
    pub dimension_name: &'a str,
    pub seed_hash: i64,
    pub gamemode: u8,
    pub previous_gamemode: i8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub has_death_location: bool,
    pub death_dimension_name: Option<&'a str>,
    pub death_location: Option<u8>, // change this to actual Position. this won't work!!
    pub portal_cooldown: VarInt,
    pub enforces_secure_chat: bool,
}

impl LoginPlayPacket<'_> {
    pub fn new(conn_id: usize) -> Self {
        Self {
            entity_id: conn_id as i32,
            is_hardcore: false,
            dimension_length: VarInt::from(1),
            dimension_names: &["minecraft:overworld"],
            max_players: VarInt::from(20),
            view_distance: VarInt::from(2),
            simulation_distance: VarInt::from(2),
            reduced_debug_info: false,
            enable_respawn_screen: true,
            do_limited_crafting: false,
            dimension_type: VarInt::new(0),
            dimension_name: "minecraft:overworld",
            seed_hash: 0,
            gamemode: 1,
            previous_gamemode: -1,
            is_debug: false,
            is_flat: false,
            has_death_location: false,
            death_dimension_name: None,
            death_location: None,
            portal_cooldown: VarInt::from(0),
            enforces_secure_chat: false,
        }
    }
}

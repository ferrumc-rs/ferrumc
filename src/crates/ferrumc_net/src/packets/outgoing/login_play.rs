use ferrumc_macros::Encode;
use ferrumc_utils::encoding::varint::VarInt;

// #[derive(Encode)]
pub struct LoginPlay {
    pub packet_id: VarInt,
    pub entity_id: i32,
    pub hardcore: bool,
    pub gamemode: u8,
    pub previous_gamemode: i8,
    // Dimension length is included
    pub dimension_names: Vec<String>,
    // I have no idea how im going to implement this.
    // This needs to encode a big ass json file into the dimension codec struct,
    // then decode that back into an NBT struct. Fuck me.
    //pub registry_codec: IDFK
    pub dimension_type: String,
    pub dimension_name: String,
    pub seed_hash: i64,
    pub max_players: VarInt,
    pub view_distance: VarInt,
    pub simulation_distance: VarInt,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub is_debug: bool,
    pub is_flat: bool,
    pub has_death_location: bool,
    // These require being able to decode options, which we can't really do yet
    // Mostly waiting on being able to override the encoder method
    // pub death_dimension_name: Option<String>,
    // pub death_location: Option<Position>,
    pub portal_cooldown: VarInt
}


use ferrumc_macros::Encode;

use crate::utils::encoding::varint::VarInt;

/// The login play packet is sent by the server to the client to start the play state.
/// Contains info about the world
#[derive(Encode)]
pub struct LoginPlay {
    pub packet_id: VarInt,
    pub entity_id: i32,
    pub hardcore: bool,
    pub gamemode: u8,
    pub previous_gamemode: i8,
    pub dimension_length: VarInt,
    pub dimension_names: Vec<String>,
    /// The codec for the dimension. Baked into the binary, see [crate::net::packets::incoming::loginstart::LoginStart::decode].
    #[encode(raw_bytes(prepend_length = false))]
    pub registry_codec: Vec<u8>,
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
    // pub death_dimension_name: Option<String>,
    // pub death_location: Option<Position>,
    pub portal_cooldown: VarInt,
}

// A test to just produce the codec file
// #[cfg(test)]
#[test]
fn generate_codec() {
    use crate::net::the_dimension_codec::Root;
    let codec_file = std::fs::File::open("../../codec.json").unwrap();
    let reader = std::io::BufReader::new(codec_file);
    let mut codec: Root = serde_json::from_reader(reader).unwrap();
    let mut codec_nbt_file = std::fs::File::create("../../nbt_codec.nbt").unwrap();
    fastnbt::to_writer(&mut codec_nbt_file, &mut codec).unwrap();
}

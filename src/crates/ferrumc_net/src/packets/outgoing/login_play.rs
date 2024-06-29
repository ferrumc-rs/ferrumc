
use ferrumc_macros::Encode;
use ferrumc_utils::encoding::varint::VarInt;

#[derive(Encode)]
pub struct LoginPlay {
    pub packet_id: VarInt,
    pub entity_id: i32,
    pub hardcore: bool,
    pub gamemode: u8,
    pub previous_gamemode: i8,
    pub dimension_length: VarInt,
    pub dimension_names: Vec<String>,
    // I have no idea how im going to implement this.
    // This needs to encode a big ass json file into the dimension codec struct,
    // then decode that back into an NBT struct. Fuck me.
    #[encode(raw_bytes(prepend_length=false))]
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
    // These require being able to decode options, which we can't really do yet
    // Mostly waiting on being able to override the encoder method
    // pub death_dimension_name: Option<String>,
    // pub death_location: Option<Position>,
    pub portal_cooldown: VarInt
}

// A test to just produce the codec file
#[cfg(test)]
#[test]
fn generate_codec() {
    use std::io::Cursor;
    use crate::the_dimension_codec::Root;
    let codec_file =  std::fs::File::open("../ferrumc_net/codec.json").unwrap();
    let reader = std::io::BufReader::new(codec_file);
    let mut codec: Root = serde_json::from_reader(reader).unwrap();
    let nbt_binary = Cursor::new(fastnbt::to_bytes(&mut codec).unwrap());
    let mut codec_zlib_file = std::fs::File::create("../ferrumc_net/nbt_codec.zlib").unwrap();
    zopfli::compress(zopfli::Options::default(), zopfli::Format::Zlib, nbt_binary, &mut codec_zlib_file).unwrap();
    let mut codec_nbt_file = std::fs::File::create("../ferrumc_net/nbt_codec.nbt").unwrap();
    fastnbt::to_writer(&mut codec_nbt_file, &mut codec).unwrap();
    
}


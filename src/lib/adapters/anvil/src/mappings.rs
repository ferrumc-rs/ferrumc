use std::collections::BTreeMap;
use ferrumc_macros::NBTDeserialize;

#[derive(NBTDeserialize, Debug)]
pub struct Chunk<'a> {
    pub dimension: Option<&'a str>,
    #[nbt(rename = "Status")]
    pub status: &'a str,
    #[nbt(rename = "DataVersion")]
    pub data_version: i32,
    #[nbt(rename = "Heightmaps")]
    pub heightmaps: Option<Heightmaps<'a>>,
    #[nbt(rename = "isLightOn")]
    pub is_light_on: Option<i8>,
    #[nbt(rename = "InhabitedTime")]
    pub inhabited_time: Option<i64>,
    #[nbt(rename = "yPos")]
    pub y_pos: i32,
    #[nbt(rename = "xPos")]
    pub x_pos: i32,
    #[nbt(rename = "zPos")]
    pub z_pos: i32,
    pub structures: Option<Structures>,
    #[nbt(rename = "LastUpdate")]
    pub last_update: Option<i64>,
    pub sections: Option<Vec<Section<'a>>>,
}

#[derive(NBTDeserialize, Debug)]
pub struct Heightmaps<'a> {
    // #[nbt(rename = "MOTION_BLOCKING_NO_LEAVES")]
    // pub motion_blocking_no_leaves: Option<Vec<i64>>,
    #[nbt(rename = "MOTION_BLOCKING")]
    pub motion_blocking: Option<&'a [i64]>,
    // #[nbt(rename = "OCEAN_FLOOR")]
    // pub ocean_floor: Option<Vec<i64>>,
    #[nbt(rename = "WORLD_SURFACE")]
    pub world_surface: Option<&'a [i64]>,
}

#[derive(NBTDeserialize, Debug)]
pub struct Structures {
    pub starts: Starts,
    #[nbt(rename = "References")]
    pub references: References,
}

#[derive(NBTDeserialize, Debug)]
pub struct Starts {}

#[derive(NBTDeserialize, Debug)]
pub struct References {}


#[derive(NBTDeserialize, Debug)]
pub struct Section<'a> {
    #[nbt(rename = "block_states")]
    pub block_states: Option<BlockStates<'a>>,
    pub biomes: Option<Biomes<'a>>,
    #[nbt(rename = "Y")]
    pub y: i8,
    #[nbt(rename = "BlockLight")]
    pub block_light: Option<&'a [i8]>,
    #[nbt(rename = "SkyLight")]
    pub sky_light: Option<&'a [i8]>,
}

#[derive(NBTDeserialize, Debug)]
pub struct BlockStates<'a> {
    // These 2 fields don't exist in the chunks stored on disk but will exist when converted to
    // network format
    pub non_air_blocks: Option<i16>,
    pub bits_per_block: Option<i8>,
    pub data: Option<&'a [i64]>,
    // This is the palette for the chunk when stored on disk
    pub palette: Option<Vec<Palette<'a>>>,
    // This is the palette for the chunk when converted to network format
    // pub net_palette: Option<Vec<VarInt>>,
}

#[derive(NBTDeserialize, Debug)]
pub struct Palette<'a> {
    #[nbt(rename = "Name")]
    pub name: &'a str,
    #[nbt(rename = "Properties")]
    pub properties: Option<BTreeMap<&'a str, &'a str>>,
}

#[derive(NBTDeserialize, Debug)]
pub struct Properties<'a> {
    pub snowy: Option<&'a str>,
    pub level: Option<&'a str>,
    pub east: Option<&'a str>,
    pub waterlogged: Option<&'a str>,
    pub north: Option<&'a str>,
    pub west: Option<&'a str>,
    pub up: Option<&'a str>,
    pub down: Option<&'a str>,
    pub south: Option<&'a str>,
    pub drag: Option<&'a str>,
    pub lit: Option<&'a str>,
    pub axis: Option<&'a str>,
}

#[derive(NBTDeserialize, Debug)]
pub struct Biomes<'a> {
    pub palette: Vec<&'a str>,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    // use ferrumc_nbt::NbtTape;
    use ferrumc_utils::root;
    use crate::loader::load_anvil_file;
    use crate::mappings::Chunk;

    #[test]
    fn try_read_one() {
        let file_path = PathBuf::from(root!(".etc/r.0.0.mca"));
        let loaded_file = load_anvil_file(file_path).unwrap();

        let chunk = loaded_file.get_chunk(0, 0).unwrap();
/*
        let mut tape = NbtTape::new(chunk.as_slice());
        tape.parse();

        println!("{:?}", tape.root.unwrap().1);
        */

        let chunk = Chunk::from_bytes(chunk.as_slice()).unwrap();

        println!("{:?}", chunk);
    }
}
use crate::utils::error::Error;
use crate::world::chunkformat::{Chunk, Palette};
use ferrumc_codec::network_types::varint::VarInt;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use std::io::Read;

const BLOCKSFILE: &[u8] = include_bytes!("../../.etc/blockmappings.json");

lazy_static! {
    static ref ID2BLOCK: HashMap<i32, Palette> = {
        let mut bzipreader = bzip2::read::BzDecoder::new(BLOCKSFILE);
        let mut output = String::new();
        bzipreader.read_to_string(&mut output).unwrap();
        let string_keys: HashMap<String, Palette> = serde_json::from_str(&output).unwrap();
        string_keys
            .iter()
            .map(|(k, v)| (k.parse::<i32>().unwrap(), v.clone()))
            .collect()
    };
    static ref BLOCK2ID: HashMap<Palette, i32> =
        ID2BLOCK.iter().map(|(k, v)| (v.clone(), *k)).collect();
}

impl Chunk {
    pub fn convert_to_net_mode(&mut self) -> Result<(), Error> {
        // This looks ugly, but it's the best way I could think of to do the error checking
        let sections = if let Some(c) = self.sections.as_mut() {
            c
        } else {
            return Err(Error::InvalidChunk(
                self.x_pos,
                self.z_pos,
                "No sections found".to_string(),
            ));
        };
        for section in sections {
            if section.block_states.is_none() {
                return Err(Error::InvalidChunk(
                    self.x_pos,
                    self.z_pos,
                    "Section is missing block states".to_string(),
                ));
            }

            let block_states = section.block_states.as_mut().unwrap();
            // TODO: Adapt this for single block sections
            if let Some(data) = &block_states.data {
                block_states.bits_per_block = Some((data.len() * 64 / 4096) as i8);
            }
            let palette = block_states.palette.as_mut().unwrap();

            for palette_entry in palette.iter() {
                if BLOCK2ID.contains_key(palette_entry) {
                    if let Some(checked_palette) = block_states.net_palette.as_mut() {
                        checked_palette.push(VarInt::from(
                            *BLOCK2ID
                                .get(palette_entry)
                                .expect("Block not found in block mappings"),
                        ));
                    }
                } else {
                    return Err(Error::InvalidChunk(
                        self.x_pos,
                        self.z_pos,
                        format!("Block {} not found in block mappings", palette_entry.name),
                    ));
                }
            }
        }

        Ok(())
    }
}

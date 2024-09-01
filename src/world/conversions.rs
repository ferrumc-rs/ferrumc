use crate::utils::error::Error;
use crate::world::chunkformat::{Chunk, Palette, Section};
use ferrumc_codec::enc::NetEncode;
use ferrumc_codec::network_types::varint::VarInt;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use std::io::Read;
use tokio::io::{AsyncWrite, AsyncWriteExt};

const BLOCKSFILE: &[u8] = include_bytes!("../../.etc/blockmappings.bz2");

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
            section.full_imported = Some(true);
            if section.block_states.is_none() {
                if section.y < 0 || section.y > 15 {
                    // This is a valid case, as the section is empty
                    continue;
                }
                return Err(Error::InvalidChunk(
                    self.x_pos,
                    self.z_pos,
                    format!("Section is missing block states in section {}", section.y).to_string(),
                ));
            }

            let block_states = section.block_states.as_mut().unwrap();
            let mut non_air_blocks = 4096i16;
            let air_id = 0i32;
            // TODO: Adapt this for single block sections
            if let Some(data) = &block_states.data {
                block_states.bits_per_block = Some((data.len() * 64 / 4096) as i8);
            } else {
                section.full_imported = Some(false);
            }
            let palette = block_states.palette.as_mut().unwrap();

            for palette_entry in palette.iter() {
                if BLOCK2ID.contains_key(palette_entry) {
                    if let Some(checked_palette) = block_states.net_palette.as_mut() {
                        let block_id = *BLOCK2ID
                            .get(palette_entry)
                            .expect("Block not found in block mappings");
                        if block_id == air_id {
                            non_air_blocks -= 1;
                        }
                        checked_palette.push(VarInt::from(block_id));
                    } else {
                        section.full_imported = Some(false);
                    }
                } else {
                    return Err(Error::InvalidChunk(
                        self.x_pos,
                        self.z_pos,
                        format!("Block {} not found in block mappings", palette_entry.name),
                    ));
                }
            }
            block_states.non_air_blocks = Some(non_air_blocks);
        }

        Ok(())
    }
}

impl NetEncode for Section {
    async fn net_encode<W>(&self, writer: &mut W) -> ferrumc_codec::Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        if self.full_imported.unwrap() {
            if let Some(block_states) = &self.block_states {
                // Non-air blocks
                writer
                    .write_all(&block_states.non_air_blocks.unwrap().to_be_bytes())
                    .await?;
                // Blocks
                writer
                    .write_all(&block_states.bits_per_block.unwrap().to_be_bytes())
                    .await?;
                let _ = &block_states
                    .net_palette
                    .as_ref()
                    .unwrap()
                    .net_encode(writer)
                    .await?;
                VarInt::from(block_states.data.as_ref().unwrap().len() as i32)
                    .net_encode(writer)
                    .await?;
                for long in block_states.data.as_ref().unwrap() {
                    writer.write_all(&long.to_be_bytes()).await?;
                }
                // Biomes
                // For now just write 3 0s
                writer.write_all(&[0; 3]).await?;
            }
        }
        Ok(())
    }
}

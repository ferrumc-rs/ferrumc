use crate::utils::error::Error;
use crate::world::chunk_format::{BlockStates, Chunk, Palette, Section};
use ferrumc_codec::enc::NetEncode;
use ferrumc_codec::network_types::varint::VarInt;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use std::io::Read;
use tokio::io::AsyncWrite;
use tracing::trace;

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

impl Section {
    pub fn set_empty(&mut self) {
        self.block_states = Some(BlockStates {
            non_air_blocks: Some(0),
            bits_per_block: Some(0),
            data: None,
            palette: None,
            net_palette: Some(vec![VarInt::from(0)]),
        });
    }
}

impl Chunk {
    /// Converts a chunk in the disk format to the network format
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
            let mut set_empty = false;
            match section.block_states.as_mut() {
                /*
                If there are no block states, set the section to empty
                This is mostly just if the section is empty or outside the world border
                In network form this would also include sections that are only 1 type of block
                But I'm not sure if this could also be the case for disk form.
                TODO: Adapt this to work for sections that are possibly only 1 block that isn't air
                */
                None => {
                    trace!(
                        "No block states found in section at {}, {}",
                        self.x_pos,
                        self.z_pos
                    );
                    set_empty = true;
                }
                Some(block_states) => {
                    // Set the max number of air blocks then decrease it every time we find and air
                    // block in the data array. This will short-circuit if the section is malformed
                    // but in that case we set it all to air anyway.
                    let mut non_air_blocks = 4096i16;
                    // This is just for readability
                    let air_id = 0i32;
                    // If the palette is missing, we can't do anything and it's actually fucked
                    if block_states.palette.is_none() {
                        return Err(Error::InvalidChunk(
                            self.x_pos,
                            self.z_pos,
                            "Palette is missing".to_string(),
                        ));
                    }

                    let palette = block_states.palette.as_mut().unwrap();

                    // TODO: Adapt this for single block sections
                    if block_states.data.is_some() {
                        let bits_per_entry = (palette.len() as f32).log2().ceil() as i8;
                        block_states.bits_per_block = Some(bits_per_entry.max(4));
                    } else {
                        trace!("No data found in section at {}", section.y);
                        set_empty = true;
                    }

                    block_states.net_palette = Some(Vec::new());

                    // Since the only difference (as far as I know) between the network and disk palettes
                    // is that the disk palette uses full block states and the network palette uses block IDs
                    // we can actually just swap the block states for block IDs. We can't really do this
                    // in place cos of type differences so we'll just make a new vec, iterate over the
                    // block states and push the block IDs to the new vec, then clear the old one.
                    for palette_entry in palette.iter() {
                        if BLOCK2ID.contains_key(palette_entry) {
                            if let Some(checked_palette) = block_states.net_palette.as_mut() {
                                let block_id = *BLOCK2ID
                                    .get(palette_entry)
                                    .expect("Block not found in block mappings");
                                // If the block is air, decrease the non-air blocks count
                                if block_id == air_id {
                                    non_air_blocks -= 1;
                                }
                                checked_palette.push(VarInt::from(block_id));
                            } else {
                                set_empty = true;
                            }
                        } else {
                            return Err(Error::InvalidChunk(
                                self.x_pos,
                                self.z_pos,
                                format!("Block {} not found in block mappings", palette_entry.name),
                            ));
                        }
                    }
                    // This should never happen but if it does, we got some major problems to sort out
                    if non_air_blocks < 0 {
                        return Err(Error::InvalidChunk(
                            self.x_pos,
                            self.z_pos,
                            "Too many air blocks. How the crispy kentucky fried fuck did this happen???".to_string(),
                        ));
                    }
                    block_states.non_air_blocks = Some(non_air_blocks);
                }
            }
            if set_empty {
                trace!("Setting section at {} to empty", section.y);
                section.set_empty();
            }
        }

        Ok(())
    }
}

impl NetEncode for Section {
    async fn net_encode<W>(&self, writer: &mut W) -> ferrumc_codec::Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        if let Some(block_states) = &self.block_states {
            // Non-air blocks
            if let Some(non_air_blocks) = block_states.non_air_blocks {
                (non_air_blocks as u16).net_encode(writer).await?;
            } else {
                // VarInt::from(0).net_encode(writer).await?;
                0u16.net_encode(writer).await?;
            }

            // Blocks
            let bpe = block_states.bits_per_block.unwrap_or(15);
            bpe.net_encode(writer).await?;

            VarInt::from(block_states.net_palette.as_ref().unwrap().len() as i32)
                .net_encode(writer)
                .await?;
            block_states
                .net_palette
                .as_ref()
                .expect("Palette is missing")
                .net_encode(writer)
                .await?;

            if let Some(data) = &block_states.data {
                VarInt::from(data.len() as i32).net_encode(writer).await?;
                for long in block_states.data.as_ref().unwrap() {
                    long.net_encode(writer).await?;
                }
            } else {
                VarInt::from(0).net_encode(writer).await?;
            }

            /*// Biomes
            // For now just write 3 0s
            writer.write_all(&[0; 3]).await?;*/
        }
        Ok(())
    }
}

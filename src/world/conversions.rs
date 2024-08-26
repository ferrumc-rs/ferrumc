use rayon::prelude::*;

use crate::utils::error::Error;
use crate::world::chunkformat::Chunk;

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
            
            // TODO: Adapt this for single block sections
            if let Some(data) = &section.block_states.as_mut().unwrap().data {
                section.block_states.as_mut().unwrap().bits_per_block =
                    Some((data.len() * 64 / 4096) as i8);
            }
        }

        Ok(())
    }
}

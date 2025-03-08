use crate::errors::WorldGenError;
use crate::{BiomeGenerator, NoiseGenerator};
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::vanilla_chunk_format::BlockData;
use std::collections::BTreeMap;

pub(crate) struct PlainsBiome;

impl BiomeGenerator for PlainsBiome {
    fn _biome_id(&self) -> u8 {
        0
    }

    fn _biome_name(&self) -> String {
        "plains".to_string()
    }

    fn generate_chunk(
        &self,
        x: i32,
        z: i32,
        noise: &NoiseGenerator,
    ) -> Result<Chunk, WorldGenError> {
        let mut chunk = Chunk::new(x, z, "overworld".to_string());
        let mut heights = vec![];
        let stone = BlockData {
            name: "minecraft:stone".to_string(),
            properties: None,
        };

        // Fill with water first
        for section_y in -4..4 {
            chunk.set_section(
                section_y as i8,
                BlockData {
                    name: "minecraft:water".to_string(),
                    properties: Some(BTreeMap::from([("level".to_string(), "0".to_string())])),
                },
            )?;
        }

        // Then generate some heights
        for chunk_x in 0..16 {
            for chunk_z in 0..16 {
                let global_x = x * 16 + chunk_x;
                let global_z = z * 16 + chunk_z;
                let height = noise.get_noise(f64::from(global_x), f64::from(global_z));
                let height = (height * 64.0) as i32 + 64;
                heights.push((global_x, global_z, height));
            }
        }

        // Fill in the sections that consist of only stone first with the set_section method since
        // it's faster than set_block
        let y_min = heights.iter().min_by(|a, b| a.2.cmp(&b.2)).unwrap().2;
        let heighst_full_section = y_min / 16;
        for section_y in -4..heighst_full_section {
            chunk.set_section(section_y as i8, stone.clone())?;
        }
        let above_filled_sections = (heighst_full_section * 16) - 1;
        for (global_x, global_z, height) in heights {
            if height > above_filled_sections {
                let height = height - above_filled_sections;
                for y in 0..height {
                    if y + above_filled_sections <= 64 {
                        chunk.set_block(
                            global_x & 0xF,
                            y + above_filled_sections,
                            global_z & 0xF,
                            BlockData {
                                name: "minecraft:sand".to_string(),
                                properties: None,
                            },
                        )?;
                    } else {
                        chunk.set_block(
                            global_x & 0xF,
                            y + above_filled_sections,
                            global_z & 0xF,
                            BlockData {
                                name: "minecraft:grass_block".to_string(),
                                properties: Some(BTreeMap::from([(
                                    "snowy".to_string(),
                                    "false".to_string(),
                                )])),
                            },
                        )?;
                    }
                }
            }
        }

        Ok(chunk)
    }
}

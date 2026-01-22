use std::collections::VecDeque;
use ahash::{HashSet, HashSetExt};
use bevy_math::U8Vec3;
use tracing::debug;
use crate::block_state_id::BlockStateId;
use crate::chunk::Chunk;
use crate::chunk::light::{engine::{LightEngine, LightNode}, storage::LightStorage, LightSection, LightType};
use crate::chunk::light::engine::{LightEngineError, LightResult, PROPAGATION_DIRECTIONS};
use crate::pos::{BlockPos, ChunkBlockPos, ChunkPos, SectionBlockPos};

#[derive(Clone)]
pub struct SkyLightEngine {
    inc: VecDeque<LightNode>,
    dec: VecDeque<LightNode>,

    dirty_chunks: HashSet<ChunkPos>,

    min_world_y: i32,
    max_world_y: i32,
}

impl LightEngine for SkyLightEngine {
    fn fill(&mut self, level: u8) {
        todo!()
    }
}

impl SkyLightEngine {
    pub fn new(min_world_y: i32, max_world_y: i32) -> Self {
        Self {
            inc: VecDeque::with_capacity(4096),
            dec: VecDeque::with_capacity(4096),
            dirty_chunks: HashSet::new(),
            min_world_y,
            max_world_y,
        }
    }

    pub fn initialize_chunk_skylight(&mut self, chunk: &mut Chunk, chunk_pos: &ChunkPos) -> LightResult<()> {
        // Step 1: Update heightmap for the chunk, which includes all sections
        for x in 0..16u8 {
            for z in 0..16u8 {
                self.update_heightmap_column(chunk, x, z)?;
            }
        }

        // Step 2: Do vertical propagation for each
        for x in 0..16u8 {
            for z in 0..16u8 {
                self.propagate_skylight_column(chunk, chunk_pos, x, z)?;
            }
        }

        // Step 3: Horizontal propagation from all lit blocks.
        self.propagate_horizontal_skylight(chunk, chunk_pos)?;
        Ok(())
    }

    fn update_heightmap_column(&mut self, chunk: &mut Chunk, x: u8, z: u8) -> LightResult<()> {
        let mut y = self.max_world_y - 1;

        while y >= self.min_world_y {
            // Tbh i dont think this gunna work, the numbers dont connect in my head but its fine.
            let block_id = chunk.get_block(ChunkBlockPos::new(x, y as i16, z));
            let opacity = Self::opacity(block_id);

            // If the opacity is 0, or below it's a solid block (Technically shouldn't be lower than 0 but who knows)
            if opacity > 0 {
                break;
            }

            y -= 1;
        }

        match chunk.heightmaps.as_mut() {
            Some(heightmap) => {
                heightmap.world_surface.set_height(x, z, y as i16);
                Ok(())
            }
            None => {
                Err(LightEngineError::UnknownHeightmap("update_heightmap_column".to_owned()))
            }
        }
    }

    // Propagates skylight down a single section (vertical)
    fn propagate_skylight_column(&mut self, chunk: &mut Chunk, chunk_pos: &ChunkPos, x: u8, z: u8) -> LightResult<()> {
        match chunk.heightmaps.as_ref() {
            Some(heightmap) => {
                let height = heightmap.world_surface.get_height(x, z);
                let mut current_light = 15u8;

                // Start from the top go down.
                for y in (self.min_world_y..self.max_world_y).rev() {
                    let chunk_block_pos = ChunkBlockPos::new(x, y as i16, z);
                    let section_block_pos = chunk_block_pos.section_block_pos();
                    let section = chunk.get_section_mut(chunk_block_pos)
                        .ok_or(LightEngineError::UnknownChunkSection(chunk_block_pos.y(), chunk_pos.x(), chunk_pos.z()))?;

                    if y > height as i32 {
                        section.light.set_light(section_block_pos, 15, LightType::Sky);
                    } else {
                        // At or below ground level.
                        let block = section.get_block(section_block_pos);
                        let opacity = Self::opacity(block);

                        if opacity == 0 {
                            // Fully transparent blocks
                            if current_light == 15 {
                                section.light.set_light(section_block_pos, 15, LightType::Sky);
                            } else {
                                // reduce level by 1
                                current_light = current_light.saturating_sub(1);
                                section.light.set_light(section_block_pos, current_light, LightType::Sky);
                            }

                            // Queue for horizontal propagation if we have light.
                            if current_light > 0 {
                                self.inc.push_back(LightNode::new(chunk_block_pos.to_block_pos(chunk_pos), current_light));
                            }
                        } else if opacity == 15 {
                            // Fully opaque block.
                            section.light.set_light(section_block_pos, 0, LightType::Sky);
                            current_light = 0;
                        } else {
                            current_light = current_light.saturating_sub(opacity);
                            section.light.set_light(section_block_pos, current_light, LightType::Sky);

                            if current_light > 0 {
                                self.inc.push_back(LightNode::new(chunk_block_pos.to_block_pos(chunk_pos), current_light));
                            }
                        }
                    }

                    // If light is gone, no point continuing down
                    if current_light == 0 {
                        break;
                    }
                }
                Ok(())
            }
            None => {
                Err(LightEngineError::UnknownHeightmap("propagate_skylight_column".to_owned()))
            }
        }

    }

    fn propagate_horizontal_skylight(&mut self, chunk: &mut Chunk, chunk_pos: &ChunkPos) -> LightResult<()> {
        while let Some(node) = self.inc.pop_front() {
            let pos = node.pos;
            let light_level = node.level;

            let chunk_block_pos = pos.chunk_block_pos();
            let section_block_pos = chunk_block_pos.section_block_pos();

            let section = chunk.get_section_mut(chunk_block_pos)
                .ok_or(LightEngineError::UnknownChunkSection(chunk_block_pos.y(), chunk_pos.x(), chunk_pos.z()))?;

            // get current light at position
            let current_light = section.light.get_light(section_block_pos, LightType::Sky);

            // skip if already lit.
            if current_light >= light_level {
                continue;
            }

            // set light at position
            section.light.set_light(section_block_pos, light_level, LightType::Sky);

            // Only propagate if we have light left.
            if light_level <= 1 {
                continue;
            }

            // Propagate all neighbors.
            // Tbh i give up on "decent" code at this point.
            for propagation_pos in PROPAGATION_DIRECTIONS {
                let neighbor_pos = pos.offset(propagation_pos);

                // Skip if out of bounds
                if neighbor_pos.pos.y < self.min_world_y || neighbor_pos.pos.y > self.max_world_y {
                    continue;
                }

                // Check if neighbor is in a different chunk
                let neighbor_chunk_x = neighbor_pos.pos.x >> 4;
                let neighbor_chunk_z = neighbor_pos.pos.z >> 4;

                // For now, skip cross-chunk propagation.
                if neighbor_chunk_x != chunk_pos.x() || neighbor_chunk_z != chunk_pos.z() {
                    continue;
                }

                let local_pos = neighbor_pos.chunk_block_pos();
                let block_id = chunk.get_block(local_pos);
                let opacity = Self::opacity(block_id);

                let new_level = if opacity == 0 {
                    if light_level == 15 && neighbor_pos.pos.y < pos.pos.y {
                        15
                    } else {
                        light_level.saturating_sub(1)
                    }
                } else if opacity >= 15 {
                    0
                } else {
                    light_level.saturating_sub(1 + opacity)
                };

                // Only propagate if new level is better than what it currently is.
                if new_level > 0 {
                    let neighbor_chunk_pos = neighbor_pos.chunk_block_pos();
                    let neighbor_section_pos = neighbor_chunk_pos.section_block_pos();

                    let section = chunk.get_section_mut(chunk_block_pos)
                        .ok_or(LightEngineError::UnknownChunkSection(chunk_block_pos.y(), chunk_pos.x(), chunk_pos.z()))?;

                    let neighbor_light = section.light.get_light(neighbor_section_pos, LightType::Sky);
                    if neighbor_light < new_level {
                        self.inc.push_back(LightNode::new(neighbor_pos, new_level));
                    }
                }
            }
        }

        Ok(())
    }

    pub fn on_block_changed(
        &mut self,
        chunk: &mut Chunk,
        chunk_pos: &ChunkPos,
        pos: BlockPos,
        old_block: BlockStateId,
        new_block: BlockStateId,
    ) -> LightResult<()> {
        let opacity_changed = Self::opacity(old_block) != Self::opacity(new_block);

        if !opacity_changed {
            return Ok(());
        }

        let chunk_block_pos = pos.chunk_block_pos();
        let heightmaps = chunk.heightmaps.as_ref().unwrap();

        // Update heightmap if needed.
        if pos.pos.y >= heightmaps.world_surface.get_height(chunk_block_pos.x(), chunk_block_pos.z()) as i32 {
            // Recalculate entire column
            self.update_heightmap_column(chunk, chunk_block_pos.x(), chunk_block_pos.z())?;
            self.propagate_skylight_column(chunk, chunk_pos, chunk_block_pos.x(), chunk_block_pos.z())?;
            self.propagate_horizontal_skylight(chunk, chunk_pos)?;
        } else {
            // Block changed below the chunk's heightmap
            if Self::opacity(new_block) == 15 {
                // Remove skylight
                self.add_skylight_at(chunk, chunk_pos, pos)?;
            } else if Self::opacity(old_block) == 15 {
                // Add skylight
                self.remove_skylight_at(chunk, chunk_pos, pos)?;
            }
        }

        Ok(())
    }

    fn add_skylight_at(&mut self, chunk: &mut Chunk, chunk_pos: &ChunkPos, pos: BlockPos) -> LightResult<()> {
        for propagation_pos in PROPAGATION_DIRECTIONS {
            let neighbor_pos = pos.offset(propagation_pos);
            if neighbor_pos.pos.y < self.min_world_y || neighbor_pos.pos.y > self.max_world_y {
                continue;
            }

            let chunk_block_pos = pos.chunk_block_pos();
            let section_block_pos = chunk_block_pos.section_block_pos();
            let section = chunk.get_section_mut(chunk_block_pos)
                .ok_or(LightEngineError::UnknownChunkSection(chunk_block_pos.y(), chunk_pos.x(), chunk_pos.z()))?;

            let neighbor_light = section.light.get_light(section_block_pos, LightType::Sky);
            if neighbor_light > 1 {
                self.inc.push_back(LightNode::new(neighbor_pos, neighbor_light));
            }
        }

        self.propagate_horizontal_skylight(chunk, chunk_pos)?;
        Ok(())
    }

    fn remove_skylight_at(&mut self, chunk: &mut Chunk, chunk_pos: &ChunkPos, pos: BlockPos) -> LightResult<()> {
        let chunk_block_pos = pos.chunk_block_pos();
        let section_block_pos = chunk_block_pos.section_block_pos();
        let section = chunk.get_section_mut(chunk_block_pos)
            .ok_or(LightEngineError::UnknownChunkSection(chunk_block_pos.y(), chunk_pos.x(), chunk_pos.z()))?;

        let initial_light = section.light.get_light(section_block_pos, LightType::Sky);
        if initial_light == 0 {
            // No light to remove.
            return Ok(());
        }

        self.dec.push_back(LightNode::new(pos, initial_light));
        section.light.set_light(section_block_pos, 0, LightType::Sky);

        while let Some(node) = self.dec.pop_front() {
            for propagation_pos in PROPAGATION_DIRECTIONS {
                let neighbor_pos = pos.offset(propagation_pos);
                if neighbor_pos.pos.y < self.min_world_y || neighbor_pos.pos.y > self.max_world_y {
                    continue;
                }

                let neighbor_light = section.light.get_light(section_block_pos, LightType::Sky);
                if neighbor_light != 0 && neighbor_light < node.level {
                    // This light came from here, remove it
                    self.dec.push_back(LightNode::new(neighbor_pos, neighbor_light));
                } else if neighbor_light >= node.level {
                    // Independent light source, re-propagate
                    self.inc.push_back(LightNode::new(neighbor_pos, neighbor_light));
                }
            }
        }

        // Re-propagate remaining light
        self.propagate_horizontal_skylight(chunk, chunk_pos)?;
        Ok(())
    }

    fn get_skylight(&self, chunk: &Chunk, pos: &BlockPos) -> Option<u8> {
        None
    }

    fn set_skylight(&mut self, chunk: &mut Chunk, pos: BlockPos, value: u8) {

    }
}

use crate::block_state_id::BlockStateId;
use crate::chunk::light::engine::{LightEngineError, LightResult, PROPAGATION_DIRECTIONS};
use crate::chunk::light::{engine::{LightEngine, LightNode}, LightType};
use crate::chunk::Chunk;
use crate::pos::{BlockPos, ChunkBlockPos, ChunkPos};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct SkyLightEngine {
    inc: VecDeque<LightNode>,
    dec: VecDeque<LightNode>,

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
            min_world_y,
            max_world_y,
        }
    }

    pub fn initialize_chunk_skylight(
        &mut self,
        chunk: &mut Chunk,
        chunk_pos: &ChunkPos,
    ) -> LightResult<()> {
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
            None => Err(LightEngineError::UnknownHeightmap(
                "update_heightmap_column".to_owned(),
            )),
        }
    }

    // Propagates skylight down a single column (vertical)
    fn propagate_skylight_column(
        &mut self,
        chunk: &mut Chunk,
        chunk_pos: &ChunkPos,
        x: u8,
        z: u8,
    ) -> LightResult<()> {
        // This code does not work with different sections, will fix
        // grab the heightmap of the chunk.
        match chunk.heightmaps.as_ref() {
            Some(heightmap) => {
                // grab the height of the heightmap at x, z (where the propagation occurs)
                let height = heightmap.world_surface.get_height(x, z);
                let mut current_light = 15u8;

                // Start from the top go down.
                for y in (self.min_world_y..self.max_world_y).rev() {
                    // grabs the chunk's relative position (x, y, z)
                    // y = current loop iteration
                    // Which also grabs the section where the current relative position is.
                    let chunk_block_pos = ChunkBlockPos::new(x, y as i16, z);
                    let section_block_pos = chunk_block_pos.section_block_pos();
                    let section = chunk.get_section_mut(chunk_block_pos).ok_or(
                        LightEngineError::UnknownChunkSection(
                            chunk_block_pos.y(),
                            chunk_pos.x(),
                            chunk_pos.z(),
                        ),
                    )?;

                    // If y is above the heightmap, set skylight to 15
                    // If not find opacity of block and set skylight accordingly
                    if y > height as i32 {
                        section
                            .light
                            .set_light(section_block_pos, 15, LightType::Sky);
                    } else {
                        // At or below ground level (heightmap).
                        // Which pretty much means, as of right now
                        // if any block that isn't air is 15 opacity.
                        // Which gradients downward until light level 0
                        let block = section.get_block(section_block_pos);
                        let opacity = Self::opacity(block);

                        // If opacity is 0 (air)
                        // else if opacity is 15 (any other block)
                        if opacity == 0 {
                            // Fully transparent blocks
                            if current_light == 15 {
                                section
                                    .light
                                    .set_light(section_block_pos, 15, LightType::Sky);
                            } else {
                                // reduce level by 1
                                current_light = current_light.saturating_sub(1);
                                section.light.set_light(
                                    section_block_pos,
                                    current_light,
                                    LightType::Sky,
                                );
                            }

                            // Queue for horizontal propagation if we have light.
                            if current_light > 0 {
                                self.inc.push_back(LightNode::new(
                                    chunk_block_pos.to_block_pos(chunk_pos),
                                    current_light,
                                ));
                            }
                        } else if opacity == 15 {
                            // Fully opaque block.
                            section
                                .light
                                .set_light(section_block_pos, 0, LightType::Sky);
                            current_light = 14; // just for testing purposes
                        } else {
                            current_light = current_light.saturating_sub(opacity);
                            section.light.set_light(
                                section_block_pos,
                                current_light,
                                LightType::Sky,
                            );

                            if current_light > 0 {
                                self.inc.push_back(LightNode::new(
                                    chunk_block_pos.to_block_pos(chunk_pos),
                                    current_light,
                                ));
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
            None => Err(LightEngineError::UnknownHeightmap(
                "propagate_skylight_column".to_owned(),
            )),
        }
    }

    fn propagate_horizontal_skylight(
        &mut self,
        chunk: &mut Chunk,
        chunk_pos: &ChunkPos,
    ) -> LightResult<()> {
        while let Some(node) = self.inc.pop_front() {
            let pos = node.pos;
            let light_level = node.level;

            let chunk_block_pos = pos.chunk_block_pos();
            let section_block_pos = chunk_block_pos.section_block_pos();

            // Check current light level
            let current_light = {
                let section = chunk.get_section(chunk_block_pos).ok_or(
                    LightEngineError::UnknownChunkSection(
                        chunk_block_pos.y(),
                        chunk_pos.x(),
                        chunk_pos.z(),
                    ),
                )?;
                section.light.get_light(section_block_pos, LightType::Sky)
            };

            if current_light >= light_level {
                continue;
            }

            // Set the light at this position
            {
                let section = chunk.get_section_mut(chunk_block_pos).ok_or(
                    LightEngineError::UnknownChunkSection(
                        chunk_block_pos.y(),
                        chunk_pos.x(),
                        chunk_pos.z(),
                    ),
                )?;
                section.light.set_light(section_block_pos, light_level, LightType::Sky);
            }

            if light_level <= 1 {
                continue;
            }

            // Propagate to neighbors
            for propagation_pos in PROPAGATION_DIRECTIONS {
                let neighbor_pos = pos.offset(propagation_pos);

                if neighbor_pos.pos.y < self.min_world_y || neighbor_pos.pos.y >= self.max_world_y {
                    continue;
                }

                let neighbor_chunk_x = neighbor_pos.pos.x >> 4;
                let neighbor_chunk_z = neighbor_pos.pos.z >> 4;

                if neighbor_chunk_x != chunk_pos.x() || neighbor_chunk_z != chunk_pos.z() {
                    continue;
                }

                let neighbor_chunk_pos = neighbor_pos.chunk_block_pos();
                let neighbor_section_pos = neighbor_chunk_pos.section_block_pos();

                // Get block opacity
                let block_id = chunk.get_block(neighbor_chunk_pos);
                let opacity = Self::opacity(block_id);

                // Calculate new light level
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

                if new_level > 0 {
                    // Get neighbor's current light
                    let neighbor_light = {
                        let neighbor_section = chunk.get_section(neighbor_chunk_pos).ok_or(
                            LightEngineError::UnknownChunkSection(
                                neighbor_chunk_pos.y(),
                                chunk_pos.x(),
                                chunk_pos.z(),
                            ),
                        )?;
                        neighbor_section.light.get_light(neighbor_section_pos, LightType::Sky)
                    };

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
        let old_opacity = Self::opacity(old_block);
        let new_opacity = Self::opacity(new_block);

        if old_opacity == new_opacity {
            return Ok(());
        }

        let chunk_block_pos = pos.chunk_block_pos();
        let heightmaps = chunk.heightmaps.as_ref()
            .ok_or(LightEngineError::UnknownHeightmap("on_block_changed".to_owned()))?;

        let height = heightmaps.world_surface.get_height(chunk_block_pos.x(), chunk_block_pos.z()) as i32;

        if pos.pos.y >= height {
            // Block is at or above heightmap - recalculate entire column
            self.update_heightmap_column(chunk, chunk_block_pos.x(), chunk_block_pos.z())?;
            self.propagate_skylight_column(chunk, chunk_pos, chunk_block_pos.x(), chunk_block_pos.z())?;
            self.propagate_horizontal_skylight(chunk, chunk_pos)?;
        } else {
            // Block changed below heightmap - localized update
            if new_opacity == 15 && old_opacity < 15 {
                // Block became opaque - remove light
                self.remove_skylight_at(chunk, chunk_pos, pos)?;
            } else if new_opacity < 15 && old_opacity == 15 {
                // Block became transparent - add light
                self.add_skylight_at(chunk, chunk_pos, pos)?;
            }
        }

        Ok(())
    }

    fn add_skylight_at(
        &mut self,
        chunk: &mut Chunk,
        chunk_pos: &ChunkPos,
        pos: BlockPos,
    ) -> LightResult<()> {
        // Check all 6 neighbors for light to propagate from
        for propagation_pos in PROPAGATION_DIRECTIONS {
            let neighbor_pos = pos.offset(propagation_pos);

            if neighbor_pos.pos.y < self.min_world_y || neighbor_pos.pos.y >= self.max_world_y {
                continue;
            }

            // Skip cross-chunk
            let neighbor_chunk_x = neighbor_pos.pos.x >> 4;
            let neighbor_chunk_z = neighbor_pos.pos.z >> 4;
            if neighbor_chunk_x != chunk_pos.x() || neighbor_chunk_z != chunk_pos.z() {
                continue;
            }

            let neighbor_chunk_pos = neighbor_pos.chunk_block_pos();
            let neighbor_section_pos = neighbor_chunk_pos.section_block_pos();

            if let Some(neighbor_section) = chunk.get_section(neighbor_chunk_pos) {
                let neighbor_light = neighbor_section.light.get_light(neighbor_section_pos, LightType::Sky);

                if neighbor_light > 1 {
                    self.inc.push_back(LightNode::new(neighbor_pos, neighbor_light));
                }
            }
        }

        self.propagate_horizontal_skylight(chunk, chunk_pos)?;
        Ok(())
    }

    fn remove_skylight_at(
        &mut self,
        chunk: &mut Chunk,
        chunk_pos: &ChunkPos,
        pos: BlockPos,
    ) -> LightResult<()> {
        let chunk_block_pos = pos.chunk_block_pos();
        let section_block_pos = chunk_block_pos.section_block_pos();

        let initial_light = if let Some(section) = chunk.get_section(chunk_block_pos) {
            section.light.get_light(section_block_pos, LightType::Sky)
        } else {
            return Ok(());
        };

        if initial_light == 0 {
            return Ok(());
        }

        // Set current block to 0
        if let Some(section) = chunk.get_section_mut(chunk_block_pos) {
            section.light.set_light(section_block_pos, 0, LightType::Sky);
        }

        self.dec.push_back(LightNode::new(pos, initial_light));

        while let Some(node) = self.dec.pop_front() {
            for propagation_pos in PROPAGATION_DIRECTIONS {
                let neighbor_pos = node.pos.offset(propagation_pos);

                if neighbor_pos.pos.y < self.min_world_y || neighbor_pos.pos.y >= self.max_world_y {
                    continue;
                }

                // Skip cross-chunk
                let neighbor_chunk_x = neighbor_pos.pos.x >> 4;
                let neighbor_chunk_z = neighbor_pos.pos.z >> 4;
                if neighbor_chunk_x != chunk_pos.x() || neighbor_chunk_z != chunk_pos.z() {
                    continue;
                }

                let neighbor_chunk_pos = neighbor_pos.chunk_block_pos();
                let neighbor_section_pos = neighbor_chunk_pos.section_block_pos();

                if let Some(neighbor_section) = chunk.get_section_mut(neighbor_chunk_pos) {
                    let neighbor_light = neighbor_section.light.get_light(neighbor_section_pos, LightType::Sky);

                    if neighbor_light != 0 && neighbor_light < node.level {
                        // This light came from us - remove it
                        self.dec.push_back(LightNode::new(neighbor_pos, neighbor_light));
                        neighbor_section.light.set_light(neighbor_section_pos, 0, LightType::Sky);
                    } else if neighbor_light >= node.level {
                        // Independent light source - re-propagate
                        self.inc.push_back(LightNode::new(neighbor_pos, neighbor_light));
                    }
                }
            }
        }

        // Re-propagate remaining light
        self.propagate_horizontal_skylight(chunk, chunk_pos)?;
        Ok(())
    }

    fn get_skylight(&self, chunk: &Chunk, pos: &ChunkBlockPos) -> Option<u8> {
        chunk.get_section(*pos).map(|section| {
            section
                .light
                .get_light(pos.section_block_pos(), LightType::Sky)
        })
    }

    fn set_skylight(&mut self, chunk: &mut Chunk, pos: &ChunkBlockPos, level: u8) {
        if let Some(section) = chunk.get_section_mut(*pos) {
            section
                .light
                .set_light(pos.section_block_pos(), level, LightType::Sky);
        }
    }
}

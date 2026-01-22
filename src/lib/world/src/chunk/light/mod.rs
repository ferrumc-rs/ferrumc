use bitcode_derive::{Decode, Encode};
use deepsize::DeepSizeOf;

use crate::chunk::light::engine::LightEngine;
use crate::pos::{ChunkBlockPos, SectionBlockPos};

pub mod block_light;
pub mod engine;
pub mod network;
pub mod sky_light;
pub mod storage;

#[derive(Clone, Copy, Debug)]
pub enum LightType {
    Sky,
    Block,
}

#[derive(Default, Clone, DeepSizeOf, Encode, Decode)]
pub enum LightSection {
    #[default]
    Empty,
    Full,
    Mixed {
        light_data: Box<[u8]>,
    },
}

#[derive(Clone, DeepSizeOf, Encode, Decode)]
pub struct SectionLightData {
    pub sky_light: LightSection,
    block_light: LightSection,
}

impl Default for SectionLightData {
    fn default() -> Self {
        Self {
            sky_light: LightSection::Mixed { light_data: Box::new([0; 2048]) },
            block_light: LightSection::default(),
        }
    }
}

impl From<Vec<i8>> for LightSection {
    fn from(data: Vec<i8>) -> Self {
        if data.len() != 2048 {
            Self::Empty
        } else {
            let mut all_on = true;
            let mut all_off = true;

            for b in data.iter() {
                if *b != i8::MAX {
                    all_on = false
                };
                if *b != i8::MIN {
                    all_off = false
                };
            }

            if all_on {
                Self::Full
            } else if all_off {
                Self::Empty
            } else {
                Self::Mixed {
                    light_data: data.into_iter().map(|v| v as u8).collect(),
                }
            }
        }
    }
}

impl SectionLightData {
    pub(crate) fn with_data(sky_light: LightSection, block_light: LightSection) -> Self {
        Self {
            sky_light,
            block_light,
        }
    }

    fn index(x: u8, y: u8, z: u8) -> usize {
        ((y as usize) << 8) | ((z as usize) << 4) | (x as usize)
    }

    pub fn get_light(&self, pos: SectionBlockPos, light_type: LightType) -> u8 {
        match light_type {
            LightType::Sky => {
                match &self.sky_light {
                    LightSection::Empty => 0u8,
                    LightSection::Full => 15u8,
                    LightSection::Mixed { light_data } => {
                        let index = Self::index(pos.x, pos.y, pos.z);
                        let byte_index = index / 2;
                        let byte = light_data[byte_index];

                        if index % 2 == 0 {
                            byte & 0x0F
                        } else {
                            (byte >> 4) & 0x0F
                        }
                    }
                }
            },
            LightType::Block => 0u8,
        }
    }

    pub fn set_light(&mut self, pos: SectionBlockPos, level: u8, light_type: LightType) {
        match light_type {
            LightType::Sky => {
                match &mut self.sky_light {
                    LightSection::Mixed { light_data } => {
                        let index = Self::index(pos.x, pos.y, pos.z);
                        let byte_index = index / 2;
                        let level = level & 0x0F;

                        // DEBUG: Log first few sets
                        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
                        if COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 10 {
                            println!("Setting sky light at ({},{},{}) to {} (index={}, byte_index={})",
                                     pos.x, pos.y, pos.z, level, index, byte_index);
                        }
                        
                        if index % 2 == 0 {
                            light_data[byte_index] = (light_data[byte_index] & 0xF0) | level;
                        } else {
                            light_data[byte_index] = (light_data[byte_index] & 0x0F) | (level << 4);
                        }

                    },
                    _ => {}
                }
            },
            LightType::Block => {}
        }
    }

    #[inline]
    pub fn contains_sky_light(&self) -> bool {
        self.sky_light.contains_light()
    }

    #[inline]
    pub fn contains_block_light(&self) -> bool {
        self.block_light.contains_light()
    }
}

impl LightSection {
    #[inline]
    pub fn contains_light(&self) -> bool {
        match self {
            LightSection::Empty => false,
            LightSection::Full => true,
            LightSection::Mixed { .. } => true,
        }
    }
}

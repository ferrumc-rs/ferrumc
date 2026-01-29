use bitcode_derive::{Decode, Encode};
use deepsize::DeepSizeOf;

use crate::pos::SectionBlockPos;

pub mod block_light;
pub mod engine;
pub mod network;
pub mod sky_light;
pub mod storage;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug)]
pub enum LightType {
    Sky,
    Block,
}

#[derive(Clone, DeepSizeOf, Encode, Decode)]
pub struct LightSection {
    light_data: Box<[u8]>,
}

impl Default for LightSection {
    fn default() -> Self {
        Self::new(Box::new([0u8; 2048]))
    }
}

impl LightSection {
    pub fn new(light_data: Box<[u8]>) -> Self {
        assert_eq!(light_data.len(), 2048);
        Self { light_data }
    }

    pub fn is_empty(&self) -> bool {
        self.light_data.iter().all(|b| *b == 0)
    }
}

#[derive(Default, Clone, DeepSizeOf, Encode, Decode)]
pub struct SectionLightData {
    pub sky_light: LightSection,
    pub block_light: LightSection,
}

impl From<Vec<i8>> for LightSection {
    fn from(data: Vec<i8>) -> Self {
        if data.len() != 2048 {
            panic!("Light section size mismatch (must be 2048)");
        } else {
            Self::new(data.into_iter().map(|v| v as u8).collect())
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
                let light_data = &self.sky_light.light_data;

                let index = Self::index(pos.x, pos.y, pos.z);
                let byte_index = index / 2;
                let byte = light_data[byte_index];

                if index % 2 == 0 {
                    byte & 0x0F
                } else {
                    (byte >> 4) & 0x0F
                }
            }
            LightType::Block => 0u8,
        }
    }

    pub fn set_light(&mut self, pos: SectionBlockPos, level: u8, light_type: LightType) {
        match light_type {
            LightType::Sky => {
                let light_data = &mut self.sky_light.light_data;

                let index = Self::index(pos.x, pos.y, pos.z);
                let byte_index = index / 2;
                let level = level & 0x0F;

                if index % 2 == 0 {
                    light_data[byte_index] = (light_data[byte_index] & 0xF0) | level;
                } else {
                    light_data[byte_index] = (light_data[byte_index] & 0x0F) | (level << 4);
                }
            }
            LightType::Block => {}
        }
    }
}

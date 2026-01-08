use bitcode_derive::{Decode, Encode};
use deepsize::DeepSizeOf;

pub mod network;

#[derive(Default, Clone, DeepSizeOf, Encode, Decode)]
pub(crate) enum LightStorage {
    #[default]
    Empty,
    Full,
    Mixed {
        light_data: Box<[u8]>,
    },
}

#[derive(Clone, DeepSizeOf, Encode, Decode)]
pub struct SectionLightData {
    sky_light: LightStorage,
    block_light: LightStorage,
}

impl Default for SectionLightData {
    fn default() -> Self {
        Self {
            sky_light: LightStorage::Full,
            block_light: LightStorage::default(),
        }
    }
}

impl SectionLightData {
    #[inline]
    pub fn contains_sky_light(&self) -> bool {
        self.sky_light.contains_light()
    }

    #[inline]
    pub fn contains_block_light(&self) -> bool {
        self.block_light.contains_light()
    }
}

impl LightStorage {
    #[inline]
    pub fn contains_light(&self) -> bool {
        match self {
            LightStorage::Empty => false,
            LightStorage::Full => true,
            LightStorage::Mixed { .. } => true,
        }
    }
}
